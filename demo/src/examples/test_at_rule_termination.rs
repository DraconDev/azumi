use azumi::prelude::*;

#[azumi::component]
pub fn at_rule_termination_test() -> impl Component {
    html! {
        <div class={foo}>"Test"</div>
        <style>
            @media (min-width: 100px) {
                .foo { color: "blue"; }
            }
            // This rule should be OUTSIDE the media query
            .bar { color: "green"; }
        </style>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at_rule_correctly_terminated() {
        let output = azumi::render_to_string(&at_rule_termination_test());
        println!("Output: {}", output);

        // We know Azumi simply concatenates string content for AtRules.
        // If parsing is correct:
        // @media (min-width: 100px) { .foo { color: "blue"; } } .bar { color: "green"; }

        // If parsing is BROKEN (swallowing):
        // @media (min-width: 100px) { .foo { color: "blue"; } .bar { color: "green"; } }

        // So checking the index of closing brace of media query relative to .bar is tricky in minified string.
        // But we can check if .bar is separate.

        // Let's rely on the parsing of the structure.
        // The output CSS is constructed by concatenating items.
        // If .bar is swollowed, it becomes part of the AtRule content string.
        // If separate, it is a StyleRule.

        // The `render_to_string` output produces the final CSS string.
        // If .bar is inside, it will be wrapped in the media block's braces.

        // Parse the OUTPUT CSS to verify structure? Too complex/dependency heavy.

        // Check finding "@media ... { ... } .bar"
        // In the broken case, we would find "@media ... { ... .bar ... }"
        // i.e. the closing brace of media query would be AFTER .bar

        let media_start = output.find("@media").expect("Missing @media");
        let bar_start = output.find(".bar").expect("Missing .bar");

        // Find the LAST closing brace.
        // If correct: ... } .bar ... } (closing style tag?) or ... } .bar { ... }
        // The output from `style.rs` `reconstruct_css_from_tokens` (or `process_global`)
        // appends "; " or " " after at_rule content.

        // Let's assume the bug exists:
        // @media ... { ... .bar ... }
        // So valid CSS structure (syntactically) but wrong semantics.

        // How to distinguish? contents of tokens?
        // Wait, if it's swallowed, it's just text inside the AtRule.
        // Code gen: `raw_css.push_str(&at_rule.content);`

        // If correctly parsed as separate rule, `reconstruct_css` puts a space or something?

        // Actually, searching for the closing brace of the media query is the best way.
        // The content of the media query is `.foo { ... }`.
        // So parse string between `@media` and `.bar`. Count braces?

        let segment = &output[media_start..bar_start];
        println!("Segment between @media and .bar: {}", segment);

        // If correct, segment should contain a closing brace `}` that balances the opening brace of media query.
        // @media (...) { ... } .bar
        // Brace count in segment should be balanced (or net 0 if we include opening).

        let open_count = segment.matches('{').count();
        let close_count = segment.matches('}').count();

        // If correct: opens == closes.
        // If broken (swallowed): opens > closes (because caching brace is AFTER .bar)

        assert_eq!(open_count, close_count, "Media query block should be closed before .bar starts. Found {} opens and {} closes in segment.", open_count, close_count);
    }
}
