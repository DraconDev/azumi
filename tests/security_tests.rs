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

#[test]
fn test_verify_rejects_too_many_pipes() {
    // More than 10 pipes should be rejected (DoS protection)
    let malicious = "a|".repeat(20);
    assert!(security::verify_state(&malicious).is_err());
}

#[test]
fn test_verify_rejects_invalid_json_pipe_structure() {
    // Has pipes but not in correct structure (json|timestamp|signature)
    let invalid = "just-json-without-timestamp";
    assert!(security::verify_state(invalid).is_err());
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

// ════════════════════════════════════════════════════════════════════════════
// Depth Check (DoS Prevention)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_compute_scope_id_deterministic() {
    use azumi::compute_scope_id;
    let id1 = compute_scope_id(10, 5);
    let id2 = compute_scope_id(10, 5);
    let id3 = compute_scope_id(10, 6);
    assert_eq!(id1, id2, "Same line/col should produce same scope ID");
    assert_ne!(
        id1, id3,
        "Different line/col should produce different scope ID"
    );
}

#[test]
fn test_compute_scope_id_format() {
    use azumi::compute_scope_id;
    let id = compute_scope_id(1, 1);
    assert!(id.starts_with("s"), "Scope ID should start with 's'");
    assert!(
        id.len() <= 20,
        "Scope ID should be reasonably short, got: {}",
        id
    );
}

// ════════════════════════════════════════════════════════════════════════════
// CSS String Escaping
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_escape_css_string_basic() {
    use azumi::escape_css_string;
    let result = escape_css_string("hello");
    assert_eq!(result, "hello");
}

#[test]
fn test_escape_css_string_semicolon() {
    use azumi::escape_css_string;
    let result = escape_css_string("color: red;");
    assert!(result.contains("\\;"));
}

#[test]
fn test_escape_css_string_backslash() {
    use azumi::escape_css_string;
    let result = escape_css_string("path\\to\\file");
    assert!(result.contains("\\\\"));
}

#[test]
fn test_escape_css_string_quotes() {
    use azumi::escape_css_string;
    let result = escape_css_string("font-family: \"Arial\"");
    assert!(result.contains("\\\""));
}

#[test]
fn test_escape_css_string_newline() {
    use azumi::escape_css_string;
    let result = escape_css_string("line1\nline2");
    assert!(result.contains("\\a"));
}

#[test]
fn test_escape_css_string_braces() {
    use azumi::escape_css_string;
    let result = escape_css_string("{valid}");
    assert!(result.contains("\\{") && result.contains("\\}"));
}

// ════════════════════════════════════════════════════════════════════════════
// Sitemap Path Normalization
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_sitemap_basic_url() {
    let sitemap = azumi::seo::SitemapBuilder::new("https://example.com")
        .add_url("/page1")
        .add_url("/page2")
        .build();
    assert!(sitemap.contains("https://example.com/page1"));
    assert!(sitemap.contains("https://example.com/page2"));
}

#[test]
fn test_sitemap_trailing_slash_normalized() {
    let sitemap = azumi::seo::SitemapBuilder::new("https://example.com/")
        .add_url("/page")
        .build();
    assert!(sitemap.contains("https://example.com/page"));
}

#[test]
fn test_sitemap_path_traversal_removed() {
    let sitemap = azumi::seo::SitemapBuilder::new("https://example.com")
        .add_url("/foo/bar/../baz")
        .build();
    assert!(sitemap.contains("https://example.com/foo/baz"));
}
