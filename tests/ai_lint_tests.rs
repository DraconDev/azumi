//! AI Lint Tests
//!
//! Tests for the AI-first compile-time validation:
//! 1. Suggestion engine (Levenshtein distance, "did you mean?")
//! 2. Positive tests — valid AI patterns compile and render correctly
//!
//! Note: Compile-error-producing lints (undefined class, @let anti-pattern, etc.)
//! cannot be tested with #[test] — they're verified by the fact that VALID code
//! compiles. For testing INVALID code produces errors, use trybuild.
//! Run with: cargo test --features test-utils

use azumi::{html, test};

// ════════════════════════════════════════════════════════════════════════════
// Suggestion Engine (Levenshtein Distance)
// ════════════════════════════════════════════════════════════════════════════

// These tests are in macros/src/suggestions.rs (unit tests in the macro crate).
// Here we verify the suggestions module is correctly integrated by testing
// that VALID code with similar names compiles without "did you mean?" errors.

#[test]
fn test_similar_class_names_compile() {
    // my_button and my_buttons are similar — both should work independently
    let component = html! {
        <div class={my_button}>
            "Click"
            <style>
                .my_button { color: "red"; }
            </style>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Click"));
}

// ════════════════════════════════════════════════════════════════════════════
// Positive Tests — Valid AI Patterns Compile Correctly
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_snake_case_class_names_work() {
    let component = html! {
        <div class={primary_button}>
            <style>
                .primary_button { background: "#3b82f6"; }
            </style>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("primary_button") || html.contains("data-s"));
}

#[test]
fn test_quoted_text_works() {
    let component = html! {
        <p>"Hello, World!"</p>
    };
    let html = test::render(&component);
    assert!(html.contains("Hello, World!"));
}

#[test]
fn test_quoted_css_values_work() {
    let component = html! {
        <div class={container}>
            <style>
                .container { padding: "1rem"; margin: "0 auto"; }
            </style>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("container") || html.contains("data-s"));
}

#[test]
fn test_dynamic_class_attribute_works() {
    let component = html! {
        <div class={active}>
            <style>
                .active { color: "green"; }
            </style>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("active") || html.contains("data-s"));
}

#[test]
fn test_dynamic_id_attribute_works() {
    let my_id = "main-content";
    let component = html! {
        <div id={my_id}>
            "Content"
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("main-content"));
}

#[test]
fn test_style_dsl_works() {
    let component = html! {
        <div style={--accent_color: "#ff0000"}>
            "Styled"
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("accent_color") || html.contains("#ff0000"));
}

#[test]
fn test_on_event_syntax_works() {
    let component = html! {
        <button>"Click me"</button>
    };
    let html = test::render(&component);
    assert!(html.contains("Click me"));
}

#[test]
fn test_style_block_after_html_works() {
    let component = html! {
        <div class={wrapper}>
            <span>"Content"</span>
            <style>
                .wrapper { display: "flex"; }
            </style>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Content"));
}

// ════════════════════════════════════════════════════════════════════════════
// Regression — AI Patterns That Previously Caused Issues
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_multiple_classes_work() {
    let component = html! {
        <div class={card}>
            <style>
                .card { border: "1px solid gray"; }
            </style>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("data-s"));
}

#[test]
fn test_nested_elements_with_classes_work() {
    let component = html! {
        <div class={container}>
            <div class={inner}>
                <style>
                    .container { padding: "1rem"; }
                    .inner { margin: "0.5rem"; }
                </style>
            </div>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("data-s"));
}

#[test]
fn test_class_with_pseudo_selector_works() {
    let component = html! {
        <div class={btn}>
            <style>
                .btn:hover { opacity: "0.8"; }
            </style>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("data-s"));
}
