#[cfg(test)]
mod tests {
    use crate::{compute_scope_id, scope_css};

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
}
