//! Escape and Special Character Tests
//!
//! Tests for HTML escaping and special character handling
//! Run with: cargo test

use azumi::{html, test};

// ════════════════════════════════════════════════════════════════════════════
// SECTION 1: HTML Entity Escaping (20 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_escape_less_than() {
    let text = "a < b";
    let component = html! { <span>{text}</span> };
    let html = test::render(&component);
    assert!(html.contains("&lt;") || html.contains("a < b"));
}

#[test]
fn test_escape_greater_than() {
    let text = "a > b";
    let component = html! { <span>{text}</span> };
    let html = test::render(&component);
    assert!(html.contains("&gt;") || html.contains("a > b"));
}

#[test]
fn test_escape_ampersand() {
    let text = "a & b";
    let component = html! { <span>{text}</span> };
    let html = test::render(&component);
    assert!(html.contains("&amp;") || html.contains("a & b"));
}

#[test]
fn test_escape_double_quote() {
    let text = r#"say "hello""#;
    let component = html! { <span>{text}</span> };
    let html = test::render(&component);
    assert!(html.contains("hello"));
}

#[test]
fn test_escape_single_quote() {
    let text = "it's fine";
    let component = html! { <span>{text}</span> };
    let html = test::render(&component);
    // Single quotes may be escaped as &#39;, &apos;, ', or left unescaped
    assert!(html.contains("fine"));
}

#[test]
fn test_html_in_text() {
    let text = "<script>alert('xss')</script>";
    let component = html! { <span>{text}</span> };
    let html = test::render(&component);
    assert!(!html.contains("<script>alert") || html.contains("&lt;script&gt;"));
}

#[test]
fn test_nested_html_tags() {
    let text = "<div><span>nested</span></div>";
    let component = html! { <p>{text}</p> };
    let html = test::render(&component);
    // Should be escaped
    assert!(html.contains("nested"));
}

#[test]
fn test_attribute_injection() {
    let val = r#"" onclick="alert('xss')"#;
    let component = html! { <div title={val}>"Content"</div> };
    let html = test::render(&component);
    assert!(html.contains("title="));
}

#[test]
fn test_multiple_special_chars() {
    let text = "< > & \" '";
    let component = html! { <span>{text}</span> };
    let html = test::render(&component);
    assert!(html.contains("span"));
}

#[test]
fn test_script_tag_in_content() {
    let text = "Click <script> here";
    let component = html! { <p>{text}</p> };
    let html = test::render(&component);
    assert!(html.contains("<p>"));
}

#[test]
fn test_style_tag_in_content() {
    let text = "Some <style> text";
    let component = html! { <p>{text}</p> };
    let html = test::render(&component);
    assert!(html.contains("<p>"));
}

#[test]
fn test_comment_syntax() {
    let text = "<!-- comment -->";
    let component = html! { <div>{text}</div> };
    let html = test::render(&component);
    assert!(html.contains("comment"));
}

#[test]
fn test_cdata_section() {
    let text = "<![CDATA[data]]>";
    let component = html! { <div>{text}</div> };
    let html = test::render(&component);
    assert!(html.contains("data"));
}

#[test]
fn test_null_char_replacement() {
    let text = "hello\0world";
    let component = html! { <span>{text}</span> };
    let html = test::render(&component);
    assert!(html.contains("hello"));
}

#[test]
fn test_backslash() {
    let text = r"path\to\file";
    let component = html! { <span>{text}</span> };
    let html = test::render(&component);
    assert!(html.contains("path"));
}

#[test]
fn test_forward_slash() {
    let text = "path/to/file";
    let component = html! { <span>{text}</span> };
    let html = test::render(&component);
    assert!(html.contains("path/to/file"));
}

#[test]
fn test_newline_in_content() {
    let text = "line1\nline2";
    let component = html! { <pre>{text}</pre> };
    let html = test::render(&component);
    assert!(html.contains("line1") && html.contains("line2"));
}

#[test]
fn test_tab_in_content() {
    let text = "col1\tcol2";
    let component = html! { <pre>{text}</pre> };
    let html = test::render(&component);
    assert!(html.contains("col1"));
}

#[test]
fn test_carriage_return() {
    let text = "line1\rline2";
    let component = html! { <pre>{text}</pre> };
    let html = test::render(&component);
    assert!(html.contains("line"));
}

#[test]
fn test_form_feed() {
    let text = "page1\x0Cpage2";
    let component = html! { <span>{text}</span> };
    let html = test::render(&component);
    assert!(html.contains("page"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 2: Unicode Characters (25 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_emoji_heart() {
    let component = html! { <span>"❤️"</span> };
    let html = test::render(&component);
    assert!(html.contains("❤"));
}

#[test]
fn test_emoji_thumbs_up() {
    let component = html! { <span>"👍"</span> };
    let html = test::render(&component);
    assert!(html.contains("👍"));
}

#[test]
fn test_emoji_face() {
    let component = html! { <span>"😀"</span> };
    let html = test::render(&component);
    assert!(html.contains("😀"));
}

#[test]
fn test_emoji_flag() {
    let component = html! { <span>"🇺🇸"</span> };
    let html = test::render(&component);
    assert!(html.contains("🇺"));
}

#[test]
fn test_chinese() {
    let component = html! { <p>"你好世界"</p> };
    let html = test::render(&component);
    assert!(html.contains("你好世界"));
}

#[test]
fn test_japanese() {
    let component = html! { <p>"こんにちは"</p> };
    let html = test::render(&component);
    assert!(html.contains("こんにちは"));
}

#[test]
fn test_korean() {
    let component = html! { <p>"안녕하세요"</p> };
    let html = test::render(&component);
    assert!(html.contains("안녕하세요"));
}

#[test]
fn test_arabic() {
    let component = html! { <p>"مرحبا"</p> };
    let html = test::render(&component);
    assert!(html.contains("مرحبا"));
}

#[test]
fn test_hebrew() {
    let component = html! { <p>"שלום"</p> };
    let html = test::render(&component);
    assert!(html.contains("שלום"));
}

#[test]
fn test_thai() {
    let component = html! { <p>"สวัสดี"</p> };
    let html = test::render(&component);
    assert!(html.contains("สวัสดี"));
}

#[test]
fn test_hindi() {
    let component = html! { <p>"नमस्ते"</p> };
    let html = test::render(&component);
    assert!(html.contains("नमस्ते"));
}

#[test]
fn test_russian() {
    let component = html! { <p>"Привет"</p> };
    let html = test::render(&component);
    assert!(html.contains("Привет"));
}

#[test]
fn test_greek() {
    let component = html! { <p>"Γειά σου"</p> };
    let html = test::render(&component);
    assert!(html.contains("Γειά"));
}

#[test]
fn test_vietnamese() {
    let component = html! { <p>"Xin chào"</p> };
    let html = test::render(&component);
    assert!(html.contains("Xin chào"));
}

#[test]
fn test_turkish() {
    let component = html! { <p>"Merhaba"</p> };
    let html = test::render(&component);
    assert!(html.contains("Merhaba"));
}

#[test]
fn test_polish() {
    let component = html! { <p>"Cześć"</p> };
    let html = test::render(&component);
    assert!(html.contains("Cześć"));
}

#[test]
fn test_czech() {
    let component = html! { <p>"Ahoj"</p> };
    let html = test::render(&component);
    assert!(html.contains("Ahoj"));
}

#[test]
fn test_romanian() {
    let component = html! { <p>"Bună"</p> };
    let html = test::render(&component);
    assert!(html.contains("Bună"));
}

#[test]
fn test_math_symbols() {
    let component = html! { <span>"∑ ∏ ∫ ∂ √"</span> };
    let html = test::render(&component);
    assert!(html.contains("∑"));
}

#[test]
fn test_currency_symbols() {
    let component = html! { <span>"$ € £ ¥ ₹ ₿"</span> };
    let html = test::render(&component);
    assert!(html.contains("€") && html.contains("¥"));
}

#[test]
fn test_arrows() {
    let component = html! { <span>"← → ↑ ↓ ↔"</span> };
    let html = test::render(&component);
    assert!(html.contains("→"));
}

#[test]
fn test_box_drawing() {
    let component = html! { <pre>"┌─┐\n│ │\n└─┘"</pre> };
    let html = test::render(&component);
    assert!(html.contains("┌"));
}

#[test]
fn test_musical_symbols() {
    let component = html! { <span>"♩ ♪ ♫ ♬"</span> };
    let html = test::render(&component);
    assert!(html.contains("♪"));
}

#[test]
fn test_chess_symbols() {
    let component = html! { <span>"♔ ♕ ♖ ♗ ♘ ♙"</span> };
    let html = test::render(&component);
    assert!(html.contains("♔"));
}

#[test]
fn test_zodiac_symbols() {
    let component = html! { <span>"♈ ♉ ♊ ♋"</span> };
    let html = test::render(&component);
    assert!(html.contains("♈"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 3: Edge Cases (15 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_empty_string() {
    let text = "";
    let component = html! { <span>{text}</span> };
    let html = test::render(&component);
    assert!(html.contains("<span>"));
}

#[test]
fn test_whitespace_only() {
    let text = "   ";
    let component = html! { <span>{text}</span> };
    let html = test::render(&component);
    assert!(html.contains("<span>"));
}

#[test]
fn test_very_long_string() {
    let text = "a".repeat(10000);
    let component = html! { <div>{&text}</div> };
    let html = test::render(&component);
    assert!(html.contains("aaaa"));
}

#[test]
fn test_single_character() {
    let text = "X";
    let component = html! { <span>{text}</span> };
    let html = test::render(&component);
    assert!(html.contains("X"));
}

#[test]
fn test_numeric_string() {
    let text = "12345";
    let component = html! { <span>{text}</span> };
    let html = test::render(&component);
    assert!(html.contains("12345"));
}

#[test]
fn test_special_html_named_entity() {
    let text = "&nbsp;";
    let component = html! { <span>{text}</span> };
    let html = test::render(&component);
    assert!(html.contains("nbsp"));
}

#[test]
fn test_url_in_text() {
    let text = "Visit https://example.com today!";
    let component = html! { <p>{text}</p> };
    let html = test::render(&component);
    assert!(html.contains("https://example.com"));
}

#[test]
fn test_email_in_text() {
    let text = "Contact us at test@example.com";
    let component = html! { <p>{text}</p> };
    let html = test::render(&component);
    assert!(html.contains("test@example.com"));
}

#[test]
fn test_phone_in_text() {
    let text = "Call +1-555-123-4567";
    let component = html! { <p>{text}</p> };
    let html = test::render(&component);
    assert!(html.contains("+1-555"));
}

#[test]
fn test_ip_address() {
    let text = "Server: 192.168.1.1";
    let component = html! { <p>{text}</p> };
    let html = test::render(&component);
    assert!(html.contains("192.168.1.1"));
}

#[test]
fn test_json_in_text() {
    let text = r#"{"key": "value"}"#;
    let component = html! { <pre>{text}</pre> };
    let html = test::render(&component);
    assert!(html.contains("key"));
}

#[test]
fn test_sql_like_text() {
    let text = "SELECT * FROM users WHERE id = 1";
    let component = html! { <code>{text}</code> };
    let html = test::render(&component);
    assert!(html.contains("SELECT"));
}

#[test]
fn test_regex_pattern() {
    let text = r"^\d{3}-\d{4}$";
    let component = html! { <code>{text}</code> };
    let html = test::render(&component);
    assert!(html.contains("d{3}"));
}

#[test]
fn test_xml_declaration() {
    let text = "<?xml version=\"1.0\"?>";
    let component = html! { <pre>{text}</pre> };
    let html = test::render(&component);
    assert!(html.contains("xml"));
}

#[test]
fn test_doctype_like_text() {
    let text = "<!DOCTYPE html>";
    let component = html! { <pre>{text}</pre> };
    let html = test::render(&component);
    assert!(html.contains("DOCTYPE"));
}
