#[cfg(test)]
mod tests {
    use crate::{compute_scope_id, scope_css, AZUMI_AI_HASH, AZUMI_RULES, AZUMI_VERSION};

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
}
