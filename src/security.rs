use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::env;

type HmacSha256 = Hmac<Sha256>;

const DEFAULT_SECRET: &str = "azumi-dev-secret-do-not-use-in-prod";

fn get_secret() -> String {
    env::var("AZUMI_SECRET").unwrap_or_else(|_| {
        #[cfg(debug_assertions)]
        {
            eprintln!(
                "⚠️  WARNING: Using default dev HMAC secret. Set AZUMI_SECRET for production!"
            );
            DEFAULT_SECRET.to_string()
        }
        #[cfg(not(debug_assertions))]
        {
            panic!(
                "FATAL: AZUMI_SECRET environment variable is REQUIRED in release builds.\n\
                 The default dev secret is publicly known and insecure.\n\
                 Set AZUMI_SECRET to a random 64+ character string before deploying."
            );
        }
    })
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

    #[test]
    fn test_default_secret_is_obviously_dev() {
        // The default secret must be obviously a dev placeholder
        assert!(
            DEFAULT_SECRET.contains("dev"),
            "Default secret should contain 'dev'"
        );
        assert!(
            DEFAULT_SECRET.contains("do-not-use"),
            "Default secret should contain 'do-not-use'"
        );
        assert!(
            DEFAULT_SECRET.len() < 50,
            "Default secret should be short enough to not look like a real key"
        );
    }

    #[test]
    fn test_sign_verify_empty_string() {
        let json = "";
        let signed = sign_state(json);
        let verified = verify_state(&signed).unwrap();
        assert_eq!(verified, json);
    }

    #[test]
    fn test_sign_verify_with_pipes_in_json() {
        // JSON containing '|' should not break the separator logic
        let json = r#"{"msg": "a|b|c"}"#;
        let signed = sign_state(json);
        let verified = verify_state(&signed).unwrap();
        assert_eq!(verified, json);
    }

    #[test]
    fn test_invalid_base64_signature() {
        let result = verify_state(r#"{"count": 10}|not-valid-base64!!!"#);
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_separator() {
        let result = verify_state(r#"{"count": 10}"#);
        assert!(result.is_err());
    }
}
