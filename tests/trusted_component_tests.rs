// Integration tests for TrustedHtml and SessionCleanupScript Components

use azumi::{html, test, Component};

#[test]
fn test_trusted_html_renders_without_escaping() {
    let comp = html! {
        <div>{azumi::TrustedHtml::new("<span>Hello &amp; World</span>")}</div>
    };
    let output = test::render(&comp);
    assert!(
        output.contains("<span>Hello &amp; World</span>"),
        "TrustedHtml should render without escaping, got: {}",
        output
    );
}

#[test]
fn test_trusted_html_preserves_script_tags() {
    let comp = html! {
        <div>{azumi::TrustedHtml::new("<script>console.log('test')</script>")}</div>
    };
    let output = test::render(&comp);
    assert!(
        output.contains("<script>console.log('test')</script>"),
        "TrustedHtml should preserve script tags, got: {}",
        output
    );
}

#[test]
fn test_trusted_html_with_html_entities() {
    let comp = html! {
        <div>{azumi::TrustedHtml::new("&lt;div&gt;Test&lt;/div&gt;")}</div>
    };
    let output = test::render(&comp);
    assert!(
        output.contains("&lt;div&gt;Test&lt;/div&gt;"),
        "TrustedHtml should not double-escape entities, got: {}",
        output
    );
}

#[test]
fn test_session_cleanup_script_renders_correctly() {
    let comp = html! {
        <head>{azumi::session_cleanup_script()}</head>
    };
    let output = test::render(&comp);
    assert!(
        output.contains("<script>"),
        "Should contain opening script tag, got: {}",
        output
    );
    assert!(
        output.contains("</script>"),
        "Should contain closing script tag, got: {}",
        output
    );
    assert!(
        output.contains("window.location.hash"),
        "Should contain session cleanup logic"
    );
    assert!(
        output.contains("session_token"),
        "Should reference session_token"
    );
}

#[test]
fn test_session_cleanup_script_in_layout() {
    let comp = html! {
        <html>
            <head>{azumi::session_cleanup_script()}</head>
            <body><div>"Content"</div></body>
        </html>
    };
    let output = test::render(&comp);
    assert!(
        output.contains("<head><script>"),
        "Script should be inside head, got: {}",
        output
    );
}

#[test]
fn test_trusted_html_empty_string() {
    let comp = html! {
        <div>{azumi::TrustedHtml::new("")}</div>
    };
    let output = test::render(&comp);
    assert!(
        output.contains("<div></div>"),
        "Empty TrustedHtml should render nothing, got: {}",
        output
    );
}

#[test]
fn test_trusted_html_with_complex_html() {
    let complex = r#"<div class="container"><h1>Title</h1><p>Paragraph with <a href="/link">link</a></p></div>"#;
    let comp = html! {
        <article>{azumi::TrustedHtml::new(complex)}</article>
    };
    let output = test::render(&comp);
    assert!(
        output.contains(r#"class="container""#),
        "Should preserve attributes, got: {}",
        output
    );
    assert!(
        output.contains("<h1>Title</h1>"),
        "Should preserve nested elements"
    );
}

#[test]
fn test_azumi_script_and_session_cleanup_together() {
    let comp = html! {
        <head>
            {azumi::azumi_script()}
            {azumi::session_cleanup_script()}
        </head>
    };
    let output = test::render(&comp);
    assert!(
        output.contains("azumi"),
        "Should contain azumi runtime"
    );
    assert!(
        output.contains("window.location.hash"),
        "Should contain session cleanup"
    );
}