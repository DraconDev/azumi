//! HTML Rendering Tests
//!
//! Comprehensive tests for Azumi's html! macro output verification.
//! Run with: cargo test --features test-utils

use azumi::{html, test};

// ════════════════════════════════════════════════════════════════════════════
// SECTION 1: Basic Tag Rendering (10 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_empty_div() {
    let component = html! { <div></div> };
    let html = test::render(&component);
    test::assert_selector(&html, "div", None);
}

#[test]
fn test_empty_span() {
    let component = html! { <span></span> };
    let html = test::render(&component);
    test::assert_selector(&html, "span", None);
}

#[test]
fn test_paragraph_with_text() {
    let component = html! { <p>"Hello World"</p> };
    let html = test::render(&component);
    test::assert_selector(&html, "p", Some("Hello World"));
}

#[test]
fn test_heading_h1() {
    let component = html! { <h1>"Title"</h1> };
    let html = test::render(&component);
    test::assert_selector(&html, "h1", Some("Title"));
}

#[test]
fn test_heading_h2() {
    let component = html! { <h2>"Subtitle"</h2> };
    let html = test::render(&component);
    test::assert_selector(&html, "h2", Some("Subtitle"));
}

#[test]
fn test_heading_h3() {
    let component = html! { <h3>"Section"</h3> };
    let html = test::render(&component);
    test::assert_selector(&html, "h3", Some("Section"));
}

#[test]
fn test_anchor_tag() {
    let component = html! { <a href="https://example.com">"Link"</a> };
    let html = test::render(&component);
    test::assert_selector(&html, "a", Some("Link"));
    assert!(html.contains("href=\"https://example.com\""));
}

#[test]
fn test_button_tag() {
    let component = html! { <button>"Click Me"</button> };
    let html = test::render(&component);
    test::assert_selector(&html, "button", Some("Click Me"));
}

#[test]
fn test_nav_tag() {
    let component = html! { <nav>"Navigation"</nav> };
    let html = test::render(&component);
    test::assert_selector(&html, "nav", Some("Navigation"));
}

#[test]
fn test_article_tag() {
    let component = html! { <article>"Content"</article> };
    let html = test::render(&component);
    test::assert_selector(&html, "article", Some("Content"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 2: Void Elements (Self-closing tags) (8 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_input_text() {
    let component = html! { <input type="text" /> };
    let html = test::render(&component);
    assert!(html.contains("<input") && html.contains("type=\"text\""));
}

#[test]
fn test_input_email() {
    let component = html! { <input type="email" name="email" /> };
    let html = test::render(&component);
    assert!(html.contains("type=\"email\"") && html.contains("name=\"email\""));
}

#[test]
fn test_input_password() {
    let component = html! { <input type="password" /> };
    let html = test::render(&component);
    assert!(html.contains("type=\"password\""));
}

#[test]
fn test_input_checkbox() {
    let component = html! { <input type="checkbox" /> };
    let html = test::render(&component);
    assert!(html.contains("type=\"checkbox\""));
}

#[test]
fn test_br_tag() {
    let component = html! { <br /> };
    let html = test::render(&component);
    assert!(html.contains("<br"));
}

#[test]
fn test_hr_tag() {
    let component = html! { <hr /> };
    let html = test::render(&component);
    assert!(html.contains("<hr"));
}

#[test]
fn test_img_tag() {
    let component = html! { <img src="/image.png" alt="Test Image" /> };
    let html = test::render(&component);
    assert!(html.contains("alt=\"Test Image\""));
}

#[test]
fn test_meta_tag() {
    let component = html! { <meta charset="utf-8" /> };
    let html = test::render(&component);
    assert!(html.contains("charset=\"utf-8\""));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 3: Static Attributes (10 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_title_attribute() {
    let component = html! { <div title="Tooltip text"></div> };
    let html = test::render(&component);
    assert!(html.contains("title=\"Tooltip text\""));
}

#[test]
fn test_data_attribute() {
    let component = html! { <div data-id="123"></div> };
    let html = test::render(&component);
    assert!(html.contains("data-id=\"123\""));
}

#[test]
fn test_aria_label() {
    let component = html! { <button aria-label="Close">"X"</button> };
    let html = test::render(&component);
    assert!(html.contains("aria-label=\"Close\""));
}

#[test]
fn test_role_attribute() {
    let component = html! { <div role="button">"Click"</div> };
    let html = test::render(&component);
    assert!(html.contains("role=\"button\""));
}

#[test]
fn test_tabindex_attribute() {
    let component = html! { <div tabindex="0">"Focusable"</div> };
    let html = test::render(&component);
    assert!(html.contains("tabindex=\"0\""));
}

#[test]
fn test_placeholder_attribute() {
    let component = html! { <input placeholder="Enter name" /> };
    let html = test::render(&component);
    assert!(html.contains("placeholder=\"Enter name\""));
}

#[test]
fn test_href_attribute() {
    let component = html! { <a href="/about">"About"</a> };
    let html = test::render(&component);
    assert!(html.contains("href=\"/about\""));
}

#[test]
fn test_target_attribute() {
    let component = html! { <a href="#" target="_blank">"External"</a> };
    let html = test::render(&component);
    assert!(html.contains("target=\"_blank\""));
}

#[test]
fn test_rel_attribute() {
    let component = html! { <a href="#" rel="noopener">"Safe Link"</a> };
    let html = test::render(&component);
    assert!(html.contains("rel=\"noopener\""));
}

#[test]
fn test_name_attribute() {
    let component = html! { <input name="username" /> };
    let html = test::render(&component);
    assert!(html.contains("name=\"username\""));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 4: Dynamic Attributes (10 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_dynamic_title() {
    let title = "Dynamic Title";
    let component = html! { <div title={title}></div> };
    let html = test::render(&component);
    assert!(html.contains("title=\"Dynamic Title\""));
}

#[test]
fn test_dynamic_href() {
    let url = "/dynamic-page";
    let component = html! { <a href={url}>"Go"</a> };
    let html = test::render(&component);
    assert!(html.contains("href=\"/dynamic-page\""));
}

#[test]
fn test_dynamic_src() {
    let img_src = "/images/photo.jpg";
    let component = html! { <img src={img_src} alt="Photo" /> };
    let html = test::render(&component);
    assert!(html.contains("/images/photo.jpg"));
}

#[test]
fn test_dynamic_data_attribute() {
    let user_id = 42;
    let component = html! { <div data-user-id={user_id}></div> };
    let html = test::render(&component);
    assert!(html.contains("data-user-id=\"42\""));
}

#[test]
fn test_dynamic_placeholder() {
    let placeholder_text = "Search...";
    let component = html! { <input placeholder={placeholder_text} /> };
    let html = test::render(&component);
    assert!(html.contains("placeholder=\"Search...\""));
}

#[test]
fn test_dynamic_aria_label() {
    let label = "Menu toggle";
    let component = html! { <button aria-label={label}>"☰"</button> };
    let html = test::render(&component);
    assert!(html.contains("aria-label=\"Menu toggle\""));
}

#[test]
fn test_dynamic_value() {
    let default_value = "John";
    let component = html! { <input value={default_value} /> };
    let html = test::render(&component);
    assert!(html.contains("value=\"John\""));
}

#[test]
fn test_expression_in_attribute() {
    let count = 5;
    let component = html! { <div data-count={count * 2}></div> };
    let html = test::render(&component);
    assert!(html.contains("data-count=\"10\""));
}

#[test]
fn test_method_call_in_attribute() {
    let items = vec!["a", "b", "c"];
    let component = html! { <div data-count={items.len()}></div> };
    let html = test::render(&component);
    assert!(html.contains("data-count=\"3\""));
}

#[test]
fn test_boolean_in_attribute() {
    let is_active = true;
    let component = html! { <div data-active={is_active}></div> };
    let html = test::render(&component);
    assert!(html.contains("data-active=\"true\""));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 5: Text Content & Interpolation (10 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_static_text() {
    let component = html! { <p>"Static content"</p> };
    let html = test::render(&component);
    test::assert_selector(&html, "p", Some("Static content"));
}

#[test]
fn test_variable_interpolation() {
    let name = "World";
    let component = html! { <p>"Hello " {name}</p> };
    let html = test::render(&component);
    assert!(html.contains("Hello") && html.contains("World"));
}

#[test]
fn test_number_interpolation() {
    let count = 42;
    let component = html! { <p>"Count: " {count}</p> };
    let html = test::render(&component);
    assert!(html.contains("Count:") && html.contains("42"));
}

#[test]
fn test_expression_interpolation() {
    let a = 10;
    let b = 5;
    let component = html! { <p>"Sum: " {a + b}</p> };
    let html = test::render(&component);
    assert!(html.contains("15"));
}

#[test]
fn test_method_interpolation() {
    let text = "hello";
    let component = html! { <p>{text.to_uppercase()}</p> };
    let html = test::render(&component);
    assert!(html.contains("HELLO"));
}

#[test]
fn test_multiple_text_segments() {
    let first = "Hello";
    let last = "World";
    let component = html! { <p>{first} " and " {last}</p> };
    let html = test::render(&component);
    assert!(html.contains("Hello") && html.contains("and") && html.contains("World"));
}

#[test]
fn test_empty_string() {
    let empty = "";
    let component = html! { <p>{empty}</p> };
    let html = test::render(&component);
    assert!(html.contains("<p>") && html.contains("</p>"));
}

#[test]
fn test_special_characters_escaped() {
    let dangerous = "<script>alert('xss')</script>";
    let component = html! { <p>{dangerous}</p> };
    let html = test::render(&component);
    // Should be escaped, not raw script
    assert!(!html.contains("<script>alert"));
}

#[test]
fn test_unicode_text() {
    let emoji = "🚀 Rocket";
    let component = html! { <p>{emoji}</p> };
    let html = test::render(&component);
    assert!(html.contains("🚀") && html.contains("Rocket"));
}

#[test]
fn test_multiline_text() {
    let component = html! {
        <p>
            "Line 1"
            "Line 2"
        </p>
    };
    let html = test::render(&component);
    assert!(html.contains("Line 1") && html.contains("Line 2"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 6: Nested Elements (8 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_nested_divs() {
    let component = html! {
        <div>
            <div>
                <div>"Deep"</div>
            </div>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Deep"));
    // Count opening div tags
    assert!(html.matches("<div").count() >= 3);
}

#[test]
fn test_list_structure() {
    let component = html! {
        <ul>
            <li>"Item 1"</li>
            <li>"Item 2"</li>
            <li>"Item 3"</li>
        </ul>
    };
    let html = test::render(&component);
    test::assert_selector(&html, "ul", None);
    assert!(html.contains("Item 1") && html.contains("Item 2") && html.contains("Item 3"));
}

#[test]
fn test_table_structure() {
    let component = html! {
        <table>
            <thead>
                <tr><th>"Header"</th></tr>
            </thead>
            <tbody>
                <tr><td>"Cell"</td></tr>
            </tbody>
        </table>
    };
    let html = test::render(&component);
    test::assert_selector(&html, "table", None);
    test::assert_selector(&html, "th", Some("Header"));
    test::assert_selector(&html, "td", Some("Cell"));
}

#[test]
fn test_form_structure() {
    let component = html! {
        <form>
            <label>"Name"</label>
            <input type="text" />
            <button type="submit">"Submit"</button>
        </form>
    };
    let html = test::render(&component);
    test::assert_selector(&html, "form", None);
    test::assert_selector(&html, "label", Some("Name"));
    test::assert_selector(&html, "button", Some("Submit"));
}

#[test]
fn test_header_structure() {
    let component = html! {
        <header>
            <nav>
                <a href="/">"Home"</a>
                <a href="/about">"About"</a>
            </nav>
        </header>
    };
    let html = test::render(&component);
    test::assert_selector(&html, "header", None);
    test::assert_selector(&html, "nav", None);
}

#[test]
fn test_footer_structure() {
    let component = html! {
        <footer>
            <p>"© 2024"</p>
        </footer>
    };
    let html = test::render(&component);
    test::assert_selector(&html, "footer", None);
    assert!(html.contains("© 2024"));
}

#[test]
fn test_section_with_children() {
    let component = html! {
        <section>
            <h2>"Section Title"</h2>
            <p>"Section content"</p>
        </section>
    };
    let html = test::render(&component);
    test::assert_selector(&html, "section", None);
    test::assert_selector(&html, "h2", Some("Section Title"));
}

#[test]
fn test_mixed_nesting() {
    let component = html! {
        <main>
            <article>
                <header><h1>"Title"</h1></header>
                <p>"Body"</p>
                <footer>"Author"</footer>
            </article>
        </main>
    };
    let html = test::render(&component);
    test::assert_selector(&html, "main", None);
    test::assert_selector(&html, "article", None);
    test::assert_selector(&html, "h1", Some("Title"));
}
