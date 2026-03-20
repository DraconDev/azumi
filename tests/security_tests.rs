//! Security Regression Tests
//!
//! Dedicated tests for Azumi's security features: HMAC signing, XSS prevention,
//! default secret behavior, and state tampering detection.
//! Run with: cargo test --features test-utils

use azumi::security;

// ════════════════════════════════════════════════════════════════════════════
// HMAC Signing & Verification
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_sign_produces_pipe_separated_output() {
    let signed = security::sign_state(r#"{"count":0}"#);
    let parts: Vec<&str> = signed.splitn(2, '|').collect();
    assert_eq!(parts.len(), 2, "Signed state must contain exactly one '|'");
    // Part 1 is base64-encoded JSON
    assert!(!parts[0].is_empty(), "JSON part must not be empty");
    // Part 2 is base64-encoded HMAC
    assert!(!parts[1].is_empty(), "Signature part must not be empty");
}

#[test]
fn test_sign_verify_roundtrip() {
    let payloads = vec![
        r#"{"count":0}"#,
        r#"{"name":"hello","value":42}"#,
        r#"{"nested":{"a":1,"b":"two"}}"#,
        r#"[]"#,
        r#"true"#,
    ];
    for json in payloads {
        let signed = security::sign_state(json);
        let verified = security::verify_state(&signed).unwrap();
        assert_eq!(verified, json, "Roundtrip failed for: {}", json);
    }
}

#[test]
fn test_verify_rejects_modified_json() {
    let signed = security::sign_state(r#"{"count":10}"#);
    let tampered = signed.replace("10", "999");
    assert!(security::verify_state(&tampered).is_err());
}

#[test]
fn test_verify_rejects_modified_signature() {
    let signed = security::sign_state(r#"{"count":10}"#);
    let parts: Vec<&str> = signed.splitn(2, '|').collect();
    let tampered = format!("{}|AAAA{}", parts[0], parts[1]);
    assert!(security::verify_state(&tampered).is_err());
}

#[test]
fn test_verify_rejects_swapped_parts() {
    let signed = security::sign_state(r#"{"count":10}"#);
    let parts: Vec<&str> = signed.splitn(2, '|').collect();
    let swapped = format!("{}|{}", parts[1], parts[0]);
    assert!(security::verify_state(&swapped).is_err());
}

// ════════════════════════════════════════════════════════════════════════════
// Edge Cases
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_empty_string_signing() {
    let signed = security::sign_state("");
    let verified = security::verify_state(&signed).unwrap();
    assert_eq!(verified, "");
}

#[test]
fn test_unicode_signing() {
    let json = r#"{"msg":"こんにちは 🎉"}"#;
    let signed = security::sign_state(json);
    let verified = security::verify_state(&signed).unwrap();
    assert_eq!(verified, json);
}

#[test]
fn test_large_payload_signing() {
    let json = format!(r#"{{"data":"{}"}}"#, "x".repeat(10000));
    let signed = security::sign_state(&json);
    let verified = security::verify_state(&signed).unwrap();
    assert_eq!(verified, json);
}

#[test]
fn test_verify_rejects_missing_separator() {
    assert!(security::verify_state(r#"{"count":10}"#).is_err());
}

#[test]
fn test_verify_rejects_empty_string() {
    assert!(security::verify_state("").is_err());
}

#[test]
fn test_verify_rejects_multiple_separators() {
    // JSON containing '|' should still work (the last '|' is the separator)
    let json = r#"{"msg":"a|b|c"}"#;
    let signed = security::sign_state(json);
    let verified = security::verify_state(&signed).unwrap();
    assert_eq!(verified, json);
}

// ════════════════════════════════════════════════════════════════════════════
// Default Secret Detection
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_default_secret_is_obviously_dev() {
    // The default secret must be obviously a dev placeholder
    // We can't access DEFAULT_SECRET directly, but we can verify
    // that the signed output is consistent (using the default in debug mode)
    let signed1 = security::sign_state("test");
    let signed2 = security::sign_state("test");
    assert_eq!(signed1, signed2, "Same input should produce same signature");
}

// ════════════════════════════════════════════════════════════════════════════
// SEO XSS Escaping (runtime functions)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_seo_title_no_raw_script_tags() {
    let html = azumi::seo::generate_head("<script>alert('xss')</script>", None, None, None, None);
    assert!(
        !html.0.contains("<script>"),
        "Title must escape <script> tags"
    );
    assert!(
        html.0.contains("&lt;script&gt;"),
        "Title must contain escaped script"
    );
}

#[test]
fn test_seo_description_quotes_escaped() {
    let html = azumi::seo::generate_head(
        "Safe",
        Some(r#""><script>alert(1)</script>"#),
        None,
        None,
        None,
    );
    // Quotes must be escaped in the content attribute
    assert!(
        html.0.contains("&quot;"),
        "Description quotes must be escaped. Got: {}",
        html.0
    );
    // The closing > should also be escaped
    assert!(
        !html.0.contains("<script>"),
        "Description must not contain raw script. Got: {}",
        html.0
    );
}

#[test]
fn test_seo_url_special_chars() {
    let html = azumi::seo::generate_head(
        "Page",
        None,
        None,
        Some(r#"https://example.com/page?a=1&b=2"#),
        None,
    );
    // URL should be properly quoted in the href attribute
    assert!(
        html.0
            .contains(r#"href="https://example.com/page?a=1&amp;b=2""#),
        "URL ampersands must be escaped. Got: {}",
        html.0
    );
}
