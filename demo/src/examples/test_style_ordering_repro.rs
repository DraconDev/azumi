use azumi::prelude::*;

#[azumi::component]
pub fn style_ordering_test() -> impl Component {
    html! {
        <div class={foo}>"Test"</div>
        <style>
            .foo { color: "red"; }
            @media (min-width: 100px) {
                .foo { color: "blue"; }
            }
            .bar { color: "green"; }
        </style>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_ordering_preserved() {
        let output = azumi::render_to_string(&style_ordering_test());
        println!("Output: {}", output);

        // We expect the CSS to be in the component's style block (or separate file depending on config, but usually inline for tests/dev)
        // Azumi usually minifys CSS, so newlines might be gone.
        // We look for the sequence: .foo ... @media ... .bar

        // Find the index of .foo definition
        let foo_idx = output
            .find(".foo{color:red;}")
            .expect("Should contain .foo rule");

        // Find the index of media query
        let media_idx = output
            .find("@media (min-width: 100px)")
            .expect("Should contain media query");

        // Find the index of .bar definition
        let bar_idx = output
            .find(".bar{color:green;}")
            .expect("Should contain .bar rule");

        // Assert order: foo < media < bar
        assert!(foo_idx < media_idx, ".foo should be before @media");
        assert!(media_idx < bar_idx, "@media should be before .bar");
    }
}
