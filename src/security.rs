use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::env;

type HmacSha256 = Hmac<Sha256>;

const DEFAULT_SECRET: &str = "azumi-dev-secret-do-not-use-in-prod";

fn get_secret() -> String {
    env::var("AZUMI_SECRET").unwrap_or_else(|_| DEFAULT_SECRET.to_string())
}

/// Signs a state string with HMAC-SHA256.
/// Returns format: "{json}|{signature_base64}"
pub fn sign_state(state_json: &str) -> String {
    let secret = get_secret();
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take any size key");

    mac.update(state_json.as_bytes());
    let result = mac.finalize();
    let signature = BASE64.encode(result.into_bytes());

    format!("{}|{}", state_json, signature)
}

/// Verifies a signed state string.
/// Returns the original JSON if valid, or an Err if invalid.
pub fn verify_state(signed_state: &str) -> Result<String, String> {
    // Expected format: "json|signature"
    // We split from the right to handle potential pipes in json (though pipes in JSON are rare, last pipe is safer)
    let idx = signed_state
        .rfind('|')
        .ok_or("Invalid state format: missing signature separator")?;

    let state_json = &signed_state[..idx];
    let signature_b64 = &signed_state[idx + 1..];

    let secret = get_secret();
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take any size key");

    mac.update(state_json.as_bytes());

    let signature_bytes = BASE64
        .decode(signature_b64)
        .map_err(|_| "Invalid base64 signature")?;

    mac.verify_slice(&signature_bytes)
        .map_err(|_| "Invalid signature: State tampering detected")?;

    Ok(state_json.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_verify() {
        let json = r#"{"count": 10}"#;
        let signed = sign_state(json);
        assert!(signed.contains('|'));

        let verified = verify_state(&signed).unwrap();
        assert_eq!(verified, json);
    }

    #[test]
    fn test_tamper_fails() {
        let json = r#"{"count": 10}"#;
        let signed = sign_state(json);

        // Tamper with the JSON part
        let tampered = signed.replace("10", "99");
        let result = verify_state(&tampered);
        assert!(result.is_err());
    }
}
