use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

const DEFAULT_SECRET: &str = "azumi-dev-secret-do-not-use-in-prod";
const MAX_STATE_AGE_SECS: u64 = 3600; // 1 hour max age for signed state

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

fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Signs a state string with HMAC-SHA256 and includes a timestamp for replay protection.
/// Returns format: "{json}|{timestamp}|{signature_base64}"
pub fn sign_state(state_json: &str) -> String {
    let secret = get_secret();
    let timestamp = get_current_timestamp();

    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take any size key");

    mac.update(state_json.as_bytes());
    mac.update(&timestamp.to_be_bytes());
    let result = mac.finalize();
    let signature = BASE64.encode(result.into_bytes());

    format!("{}|{}|{}", state_json, timestamp, signature)
}

/// Verifies a signed state string and checks timestamp for replay protection.
/// Returns the original JSON if valid, or an Err if invalid or expired.
pub fn verify_state(signed_state: &str) -> Result<String, String> {
    // Limit input length to prevent DoS attacks
    if signed_state.len() > 100_000 {
        return Err("State too large: maximum size is 100KB".to_string());
    }

    // Expected format: "json|timestamp|signature"
    let parts: Vec<&str> = signed_state.split('|').collect();
    if parts.len() != 3 {
        return Err("Invalid state format: expected 'json|timestamp|signature'".to_string());
    }

    let state_json = parts[0];
    let timestamp_str = parts[1];
    let signature_b64 = parts[2];

    // Parse and validate timestamp
    let timestamp: u64 = timestamp_str
        .parse()
        .map_err(|_| "Invalid timestamp format")?;

    let current_time = get_current_timestamp();
    if current_time.saturating_sub(timestamp) > MAX_STATE_AGE_SECS {
        return Err("State has expired: replay detected or state too old".to_string());
    }

    let secret = get_secret();
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take any size key");

    mac.update(state_json.as_bytes());
    mac.update(&timestamp.to_be_bytes());

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
        assert_eq!(signed.matches('|').count(), 2); // json|timestamp|signature

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
        // JSON containing '|' should work since we split and take first element
        let json = r#"{"msg": "a|b|c"}"#;
        let signed = sign_state(json);
        let verified = verify_state(&signed).unwrap();
        assert_eq!(verified, json);
    }

    #[test]
    fn test_invalid_base64_signature() {
        // New format needs 3 parts
        let result = verify_state(r#"{"count": 10}|1234567890|not-valid-base64!!!"#);
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_separator() {
        // Old format (no timestamp) should fail
        let result = verify_state(r#"{"count": 10}"#);
        assert!(result.is_err());
    }

    #[test]
    fn test_expired_state_fails() {
        // Create state with timestamp of 0 (epoch)
        let json = r#"{"count": 10}"#;
        let expired = format!("{}|0|invalid", json);
        let result = verify_state(&expired);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expired"));
    }

    #[test]
    fn test_state_too_large_fails() {
        let json = "x".repeat(100_001);
        let result = verify_state(&json);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("too large"));
    }
}
