#[cfg(test)]
mod tests {
    use crate::{
        azumi_script, compute_scope_id, scope_css, Component, AZUMI_AI_HASH, AZUMI_RULES,
        AZUMI_VERSION,
    };

    #[test]
    fn test_scope_css_basic() {
        let css = ".button { color: red; }";
        let scoped = scope_css(css, "abc");
        assert!(scoped.contains(".button[data-abc]"));
        assert!(scoped.contains("color: red;"));
    }

    #[test]
    fn test_scope_css_multiple_selectors() {
        let css = ".button, .link { color: blue; }";
        let scoped = scope_css(css, "s123");
        assert!(scoped.contains(".button[data-s123], .link[data-s123]"));
    }

    #[test]
    fn test_compute_scope_id_deterministic() {
        let id1 = compute_scope_id(10, 5);
        let id2 = compute_scope_id(10, 5);
        assert_eq!(id1, id2, "Same input should produce same scope ID");
        assert!(id1.starts_with('s'), "Scope ID should start with 's'");
    }

    #[test]
    fn test_compute_scope_id_different_inputs() {
        let id1 = compute_scope_id(10, 5);
        let id2 = compute_scope_id(10, 6);
        let id3 = compute_scope_id(11, 5);
        assert_ne!(id1, id2, "Different column should produce different ID");
        assert_ne!(id1, id3, "Different line should produce different ID");
    }

    #[test]
    fn test_ai_hash_is_populated() {
        assert!(!AZUMI_AI_HASH.is_empty(), "AZUMI_AI_HASH must not be empty");
        assert!(
            AZUMI_AI_HASH.len() >= 8,
            "AZUMI_AI_HASH should be at least 8 chars, got {}",
            AZUMI_AI_HASH.len()
        );
    }

    #[test]
    fn test_version_matches_cargo() {
        assert_eq!(AZUMI_VERSION, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn test_ai_rules_are_nonempty() {
        assert!(!AZUMI_RULES.is_empty(), "AZUMI_RULES must not be empty");
        assert!(
            AZUMI_RULES.len() >= 10,
            "Should have at least 10 strict rules"
        );
    }

    #[test]
    fn test_ai_rules_contain_key_rules() {
        let all_rules = AZUMI_RULES.join("\n");
        assert!(all_rules.contains("quoted"), "Rules must mention quoting");
        assert!(
            all_rules.contains("snake_case"),
            "Rules must mention snake_case"
        );
        assert!(all_rules.contains("HMAC"), "Rules must mention HMAC");
        assert!(all_rules.contains("@let"), "Rules must mention @let");
    }

    #[test]
    fn test_azumi_script_returns_component() {
        let script = azumi_script();
        let _ = script;
    }

    #[test]
    fn test_azumi_script_renders_correctly() {
        let script = azumi_script();
        let output = crate::render_to_string(&script);
        assert!(
            output.starts_with("<script>"),
            "Should start with <script>, got: {}",
            output
        );
        assert!(
            output.ends_with("</script>"),
            "Should end with </script>, got: {}",
            output
        );
    }

    #[test]
    fn test_azumi_script_escapes_script_end_tag() {
        let script = azumi_script();
        let output = crate::render_to_string(&script);
        let js_content = &output[8..output.len() - 9];
        if js_content.contains("</script>") {
            assert!(
                output.contains(r"<\/script>"),
                "If JS contains </script>, it should be escaped as <\\/script>"
            );
        }
    }

    #[test]
    fn test_azumi_script_contains_azumi_code() {
        let script = azumi_script();
        let output = crate::render_to_string(&script);
        assert!(
            output.contains("azumi"),
            "Should contain 'azumi' identifier"
        );
    }

    #[test]
    fn test_session_cleanup_script_renders() {
        let script = crate::session_cleanup_script();
        let output = crate::render_to_string(&script);
        assert!(
            output.starts_with("<script>"),
            "Should start with <script>, got: {}",
            output
        );
        assert!(
            output.ends_with("</script>"),
            "Should end with </script>, got: {}",
            output
        );
        assert!(
            output.contains("window.location.hash"),
            "Should contain session cleanup logic"
        );
    }

    #[test]
    fn test_trusted_html_renders_without_escaping() {
        let html = crate::TrustedHtml::new("<div>Hello &amp; World</div>");
        let output = crate::render_to_string(&html);
        assert_eq!(
            output, "<div>Hello &amp; World</div>",
            "TrustedHtml should render without escaping"
        );
    }

    #[test]
    fn test_trusted_html_preserves_script_tags() {
        let html = crate::TrustedHtml::new("<script>alert('test')</script>");
        let output = crate::render_to_string(&html);
        assert!(
            output.contains("<script>"),
            "TrustedHtml should preserve script tags, got: {}",
            output
        );
    }
}
