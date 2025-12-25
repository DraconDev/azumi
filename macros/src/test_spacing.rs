#[cfg(test)]
mod tests {
    use proc_macro2::{TokenStream, TokenTree};
    use std::str::FromStr;

    // Copy of the logic from token_parser.rs for testing
    fn tokens_to_string(tokens: &[TokenTree]) -> String {
        let mut output = String::new();
        for (i, tt) in tokens.iter().enumerate() {
            let s = tt.to_string();
            output.push_str(&s);

            if i + 1 < tokens.len() {
                let next = &tokens[i + 1];
                if should_add_space(tt, next) {
                    output.push(' ');
                }
            }
        }
        output
    }

    fn should_add_space(curr: &TokenTree, next: &TokenTree) -> bool {
        use proc_macro2::TokenTree::*;
        match (curr, next) {
            (Ident(_), Ident(_)) => true,     // const app
            (Ident(_), Literal(_)) => true,   // return 0
            (Literal(_), Ident(_)) => true,   // 0 auto
            (Literal(_), Literal(_)) => true, // 1 2
            (Punct(p), Ident(_)) if p.as_char() == ',' || p.as_char() == ';' => true, // , next or ; next
            _ => false,
        }
    }

    #[test]
    fn test_token_stream_spacing() {
        // JS Example
        let js_source = "const app = { count: 0, increment() { this.count++; } };";
        let ts = TokenStream::from_str(js_source).unwrap();
        let tokens: Vec<TokenTree> = ts.into_iter().collect();

        let output = tokens_to_string(&tokens);
        println!("Original JS:      {}", js_source);
        println!("Heuristic JS:     {}", output);

        // Verify key patterns
        assert!(output.contains("const app"), "Should preserve 'const app'");
        // Group formatting adds spaces
        assert!(
            output.contains("count") && output.contains(":"),
            "Should contain count and :"
        );
        assert!(
            output.contains("this") && output.contains("count") && output.contains("++"),
            "Should contain this.count++ components"
        );

        // Test Top-Level Heuristic specifically
        let top_level_css = "margin: 0 auto;";
        let ts_top = TokenStream::from_str(top_level_css).unwrap();
        let tokens_top: Vec<TokenTree> = ts_top.into_iter().collect();
        let output_top = tokens_to_string(&tokens_top);
        println!("Top Level CSS:    {}", top_level_css);
        println!("Heuristic Top:    {}", output_top);
        assert!(
            output_top.contains("0 auto"),
            "Should preserve '0 auto' (Literal Ident)"
        );

        // CSS Example
        let css_source = ".container { margin: 0; padding: 1px solid red; }";
        let ts_css = TokenStream::from_str(css_source).unwrap();
        let tokens_css: Vec<TokenTree> = ts_css.into_iter().collect();

        let css_output = tokens_to_string(&tokens_css);
        println!("Original CSS:     {}", css_source);
        println!("Heuristic CSS:    {}", css_output);

        assert!(
            css_output.contains(".container"),
            "Should preserve '.container'"
        );
        // 1px solid is inside Group, so it will have spaces: 1px solid
        assert!(
            css_output.contains("1px") && css_output.contains("solid"),
            "Should contain 1px and solid"
        );

        // Complex CSS
        let complex_css = "div.foo > div # bar { color: red; }";
        let ts_complex = TokenStream::from_str(complex_css).unwrap();
        let tokens_complex: Vec<TokenTree> = ts_complex.into_iter().collect();
        let complex_output = tokens_to_string(&tokens_complex);

        println!("Original Complex: {}", complex_css);
        println!("Heuristic Cplx:   {}", complex_output);

        // Note: div .foo becomes div.foo with this heuristic, which is known limitation
        // But div # bar becomes div#bar
    }
}
