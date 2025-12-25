//! Script and Embed Tests
//!
//! Tests for script, embed, iframe, and object elements
//! Run with: cargo test

use azumi::{html, test};

// ════════════════════════════════════════════════════════════════════════════
// SECTION 1: Script Elements (10 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_script_src() {
    let component = html! { <script src="/app.js"></script> };
    let html = test::render(&component);
    assert!(html.contains("<script") && html.contains("src="));
}

#[test]
fn test_script_defer() {
    let component = html! { <script src="/app.js" defer="true"></script> };
    let html = test::render(&component);
    assert!(html.contains("defer"));
}

#[test]
fn test_script_async() {
    let component = html! { <script src="/app.js" async="true"></script> };
    let html = test::render(&component);
    assert!(html.contains("async"));
}

#[test]
fn test_script_module() {
    let component = html! { <script type="module" src="/app.mjs"></script> };
    let html = test::render(&component);
    assert!(html.contains("type=\"module\""));
}

#[test]
fn test_script_nomodule() {
    let component = html! { <script data-nomodule="true" src="/legacy.js"></script> };
    let html = test::render(&component);
    assert!(html.contains("data-nomodule"));
}

#[test]
fn test_script_integrity() {
    let component =
        html! { <script src="/app.js" integrity="sha384-abc" crossorigin="anonymous"></script> };
    let html = test::render(&component);
    assert!(html.contains("integrity="));
}

#[test]
fn test_script_nonce() {
    let nonce = "abc123";
    let component = html! { <script src="/app.js" nonce={nonce}></script> };
    let html = test::render(&component);
    assert!(html.contains("nonce="));
}

#[test]
fn test_script_referrerpolicy() {
    let component = html! { <script src="/app.js" referrerpolicy="no-referrer"></script> };
    let html = test::render(&component);
    assert!(html.contains("referrerpolicy"));
}

#[test]
fn test_multiple_scripts() {
    let scripts = vec!["/lib.js", "/app.js"];
    let component = html! {
        <div>
            @for src in &scripts {
                <script src={*src}></script>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("lib.js") && html.contains("app.js"));
}

#[test]
fn test_script_type_json() {
    let component = html! { <script type="application/json">"json data"</script> };
    let html = test::render(&component);
    assert!(html.contains("application/json"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 2: Iframe Elements (10 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_iframe_basic() {
    let component = html! { <iframe src="https://example.com" title="Example">"Iframe"</iframe> };
    let html = test::render(&component);
    assert!(html.contains("<iframe") && html.contains("src="));
}

#[test]
fn test_iframe_width_height() {
    let component =
        html! { <iframe src="/page.html" title="Page" width="600" height="400">"Iframe"</iframe> };
    let html = test::render(&component);
    assert!(html.contains("width=\"600\"") && html.contains("height=\"400\""));
}

#[test]
fn test_iframe_sandbox() {
    let component = html! { <iframe src="/page.html" title="Sandboxed" sandbox="allow-scripts">"Iframe"</iframe> };
    let html = test::render(&component);
    assert!(html.contains("sandbox="));
}

#[test]
fn test_iframe_loading_lazy() {
    let component =
        html! { <iframe src="/page.html" title="Lazy" loading="lazy">"Iframe"</iframe> };
    let html = test::render(&component);
    assert!(html.contains("loading=\"lazy\""));
}

#[test]
fn test_iframe_allow() {
    let component =
        html! { <iframe src="/video.html" title="Video" allow="autoplay">"Iframe"</iframe> };
    let html = test::render(&component);
    assert!(html.contains("allow="));
}

#[test]
fn test_iframe_allowfullscreen() {
    let component = html! { <iframe src="/video.html" title="Video" data-allowfullscreen="true">"Iframe"</iframe> };
    let html = test::render(&component);
    assert!(html.contains("data-allowfullscreen"));
}

#[test]
fn test_iframe_srcdoc() {
    let component = html! { <iframe srcdoc="<h1>Hello</h1>" title="Content">"Iframe"</iframe> };
    let html = test::render(&component);
    assert!(html.contains("srcdoc="));
}

#[test]
fn test_iframe_name() {
    let component =
        html! { <iframe src="/page.html" title="Named" name="content">"Iframe"</iframe> };
    let html = test::render(&component);
    assert!(html.contains("name=\"content\""));
}

#[test]
fn test_iframe_referrerpolicy() {
    let component = html! { <iframe src="https://external.com" title="External" referrerpolicy="no-referrer">"Iframe"</iframe> };
    let html = test::render(&component);
    assert!(html.contains("referrerpolicy"));
}

#[test]
fn test_youtube_embed() {
    let video_id = "dQw4w9WgXcQ";
    let src = format!("https://www.youtube.com/embed/{}", video_id);
    let component =
        html! { <iframe src={&src} title="YouTube Video" data-fullscreen="true">"Video"</iframe> };
    let html = test::render(&component);
    assert!(html.contains("youtube.com/embed"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 3: Canvas and SVG (5 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_canvas() {
    let component = html! { <canvas width="400" height="300">"Canvas not supported"</canvas> };
    let html = test::render(&component);
    assert!(html.contains("<canvas") && html.contains("width="));
}

#[test]
fn test_svg_basic() {
    let component = html! { <svg width="100" height="100">"SVG content"</svg> };
    let html = test::render(&component);
    assert!(html.contains("<svg") && html.contains("width="));
}

#[test]
fn test_svg_viewbox() {
    let component = html! { <svg viewBox="0 0 100 100">"SVG"</svg> };
    let html = test::render(&component);
    assert!(html.contains("viewBox="));
}

#[test]
fn test_svg_xmlns() {
    let component = html! { <svg xmlns="http://www.w3.org/2000/svg">"SVG"</svg> };
    let html = test::render(&component);
    assert!(html.contains("xmlns="));
}

#[test]
fn test_svg_fill() {
    let component = html! { <svg fill="currentColor">"SVG"</svg> };
    let html = test::render(&component);
    assert!(html.contains("fill="));
}
