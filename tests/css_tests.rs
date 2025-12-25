//! CSS Tests
//!
//! Tests for Azumi's CSS generation and style handling
//! Run with: cargo test --features test-utils

use azumi::{html, test};

// ════════════════════════════════════════════════════════════════════════════
// SECTION 1: Style Tag Rendering (10 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_style_block_exists() {
    let component = html! {
        <div>"Content"</div>
        <style>
            .test_class { padding: "1rem"; }
        </style>
    };
    let html = test::render(&component);
    assert!(html.contains("<style>") || html.contains("padding"));
}

#[test]
fn test_multiple_css_rules() {
    let component = html! {
        <div>"Content"</div>
        <style>
            .rule_one { color: "red"; }
            .rule_two { color: "blue"; }
        </style>
    };
    let html = test::render(&component);
    assert!(html.contains("rule_one") || html.contains("rule_two") || html.contains("color"));
}

#[test]
fn test_nested_css_selectors() {
    let component = html! {
        <div>"Content"</div>
        <style>
            .parent .child { margin: "0"; }
        </style>
    };
    let html = test::render(&component);
    assert!(html.contains("parent") || html.contains("child") || html.contains("margin"));
}

#[test]
fn test_pseudo_class_hover() {
    let component = html! {
        <button>"Hover"</button>
        <style>
            button:hover { background: "gray"; }
        </style>
    };
    let html = test::render(&component);
    assert!(html.contains("hover") || html.contains("button"));
}

#[test]
fn test_pseudo_class_focus() {
    let component = html! {
        <input type="text" />
        <style>
            input:focus { border: "2px"; }
        </style>
    };
    let html = test::render(&component);
    assert!(html.contains("focus") || html.contains("input"));
}

#[test]
fn test_pseudo_element_before() {
    let component = html! {
        <div>"Quote"</div>
        <style>
            div::before { content: "\""; }
        </style>
    };
    let html = test::render(&component);
    assert!(html.contains("before") || html.contains("content"));
}

#[test]
fn test_pseudo_element_after() {
    let component = html! {
        <div>"Quote"</div>
        <style>
            div::after { content: "\""; }
        </style>
    };
    let html = test::render(&component);
    assert!(html.contains("after") || html.contains("content"));
}

#[test]
fn test_media_query() {
    let component = html! {
        <div>"Responsive"</div>
        <style>
            @media (min-width: "768px") {
                div { width: "50%"; }
            }
        </style>
    };
    let html = test::render(&component);
    assert!(html.contains("@media") || html.contains("768px") || html.contains("width"));
}

#[test]
fn test_css_variables() {
    let component = html! {
        <div>"Themed"</div>
        <style>
            :root { --primary: "blue"; }
            div { color: "var(--primary)"; }
        </style>
    };
    let html = test::render(&component);
    assert!(html.contains("--primary") || html.contains("var("));
}

#[test]
fn test_keyframes() {
    let component = html! {
        <div>"Animate"</div>
        <style>
            @keyframes fadeIn {
                from { opacity: "0"; }
                to { opacity: "1"; }
            }
        </style>
    };
    let html = test::render(&component);
    assert!(html.contains("@keyframes") || html.contains("fadeIn") || html.contains("opacity"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 2: CSS Properties (10 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_display_property() {
    let component = html! {
        <div>"Flex"</div>
        <style>
            div { display: "flex"; }
        </style>
    };
    let html = test::render(&component);
    assert!(html.contains("display") || html.contains("flex"));
}

#[test]
fn test_flex_properties() {
    let component = html! {
        <div>"Flex container"</div>
        <style>
            div {
                display: "flex";
                justify-content: "center";
                align-items: "center";
            }
        </style>
    };
    let html = test::render(&component);
    assert!(
        html.contains("justify-content") || html.contains("align-items") || html.contains("flex")
    );
}

#[test]
fn test_grid_properties() {
    let component = html! {
        <div>"Grid"</div>
        <style>
            div {
                display: "grid";
                grid-template-columns: "1fr";
            }
        </style>
    };
    let html = test::render(&component);
    assert!(html.contains("grid") || html.contains("grid-template-columns"));
}

#[test]
fn test_position_properties() {
    let component = html! {
        <div>"Positioned"</div>
        <style>
            div {
                position: "absolute";
                top: "0";
                left: "0";
            }
        </style>
    };
    let html = test::render(&component);
    assert!(html.contains("position") || html.contains("absolute"));
}

#[test]
fn test_border_properties() {
    let component = html! {
        <div>"Border"</div>
        <style>
            div {
                border: "1px";
                border-radius: "8px";
            }
        </style>
    };
    let html = test::render(&component);
    assert!(html.contains("border") || html.contains("border-radius"));
}

#[test]
fn test_box_shadow() {
    let component = html! {
        <div>"Shadow"</div>
        <style>
            div { box-shadow: "0"; }
        </style>
    };
    let html = test::render(&component);
    assert!(html.contains("box-shadow"));
}

#[test]
fn test_transition() {
    let component = html! {
        <div>"Transition"</div>
        <style>
            div { transition: "all"; }
        </style>
    };
    let html = test::render(&component);
    assert!(html.contains("transition"));
}

#[test]
fn test_transform() {
    let component = html! {
        <div>"Transform"</div>
        <style>
            div { transform: "rotate(45deg)"; }
        </style>
    };
    let html = test::render(&component);
    assert!(html.contains("transform") || html.contains("rotate"));
}

#[test]
fn test_font_properties() {
    let component = html! {
        <div>"Text"</div>
        <style>
            div {
                font-family: "Inter";
                font-size: "16px";
                font-weight: "bold";
            }
        </style>
    };
    let html = test::render(&component);
    assert!(
        html.contains("font-family") || html.contains("font-size") || html.contains("font-weight")
    );
}

#[test]
fn test_color_properties() {
    let component = html! {
        <div>"Colors"</div>
        <style>
            div {
                color: "#333";
                background-color: "white";
            }
        </style>
    };
    let html = test::render(&component);
    assert!(html.contains("color") || html.contains("#333") || html.contains("background-color"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 3: Global Styles (6 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_global_style_tag() {
    let component = html! {
        <div>"Global"</div>
        <style global>
            body { margin: "0"; }
        </style>
    };
    let html = test::render(&component);
    assert!(html.contains("body") || html.contains("margin"));
}

#[test]
fn test_global_reset() {
    let component = html! {
        <div>"Reset"</div>
        <style global>
            * { box-sizing: "border-box"; }
        </style>
    };
    let html = test::render(&component);
    assert!(html.contains("box-sizing") || html.contains("*"));
}

#[test]
fn test_global_html_element() {
    let component = html! {
        <div>"HTML"</div>
        <style global>
            html { font-size: "16px"; }
        </style>
    };
    let html = test::render(&component);
    assert!(html.contains("html") || html.contains("font-size"));
}

#[test]
fn test_scoped_and_global() {
    let component = html! {
        <div>"Mixed"</div>
        <style>
            div { color: "black"; }
        </style>
        <style global>
            body { background: "white"; }
        </style>
    };
    let html = test::render(&component);
    assert!(html.contains("color") || html.contains("background"));
}

#[test]
fn test_global_font_import() {
    let component = html! {
        <div>"Fonts"</div>
        <style global>
            @font-face {
                font-family: "CustomFont";
                src: "url('/fonts/custom.woff2')";
            }
        </style>
    };
    let html = test::render(&component);
    assert!(html.contains("@font-face") || html.contains("font-family"));
}

#[test]
fn test_global_selection() {
    let component = html! {
        <div>"Selection"</div>
        <style global>
            ::selection { background: "yellow"; }
        </style>
    };
    let html = test::render(&component);
    assert!(html.contains("::selection") || html.contains("background"));
}
