//! Error Handling and Edge Case Tests
//!
//! Tests for error conditions and edge cases
//! Run with: cargo test --features test-utils

use azumi::{html, test};

// ════════════════════════════════════════════════════════════════════════════
// SECTION 1: Empty/Minimal Values (15 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_empty_string_content() {
    let text = "";
    let component = html! { <span>{text}</span> };
    let html = test::render(&component);
    assert!(html.contains("<span>"));
}

#[test]
fn test_empty_string_attribute() {
    let value = "";
    let component = html! { <input type="text" value={value} /> };
    let html = test::render(&component);
    assert!(html.contains("value="));
}

#[test]
fn test_empty_vec_loop() {
    let items: Vec<&str> = vec![];
    let component = html! {
        <ul>
            @for item in &items {
                <li>{item}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("<ul>") && html.contains("</ul>"));
    assert!(!html.contains("<li>"));
}

#[test]
fn test_none_option_if_let() {
    let value: Option<&str> = None;
    let component = html! {
        <div>
            @if let Some(v) = value {
                <span>{v}</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("<div>") && !html.contains("<span>"));
}

#[test]
fn test_zero_number() {
    let count = 0;
    let component = html! { <span>{count}</span> };
    let html = test::render(&component);
    assert!(html.contains("0"));
}

#[test]
fn test_false_boolean() {
    let active = false;
    let component = html! { <span>{active}</span> };
    let html = test::render(&component);
    assert!(html.contains("false"));
}

#[test]
fn test_whitespace_only_string() {
    let text = "   ";
    let component = html! { <span>{text}</span> };
    let html = test::render(&component);
    assert!(html.contains("<span>"));
}

#[test]
fn test_single_char() {
    let c = 'X';
    let component = html! { <span>{c}</span> };
    let html = test::render(&component);
    assert!(html.contains("X"));
}

#[test]
fn test_very_long_string() {
    let long_text = "a".repeat(10000);
    let component = html! { <p>{&long_text}</p> };
    let html = test::render(&component);
    assert!(html.len() > 10000);
}

#[test]
fn test_vec_single_item() {
    let items = vec!["only"];
    let component = html! {
        <ul>
            @for item in &items {
                <li>{item}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("only"));
}

#[test]
fn test_option_some() {
    let value: Option<&str> = Some("present");
    let component = html! {
        <div>
            @if let Some(v) = value {
                <span>{v}</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("present"));
}

#[test]
fn test_negative_zero() {
    let n = -0.0f64;
    let component = html! { <span>{n}</span> };
    let html = test::render(&component);
    assert!(html.contains("0"));
}

#[test]
fn test_max_i32() {
    let n = i32::MAX;
    let component = html! { <span>{n}</span> };
    let html = test::render(&component);
    assert!(html.contains("2147483647"));
}

#[test]
fn test_min_i32() {
    let n = i32::MIN;
    let component = html! { <span>{n}</span> };
    let html = test::render(&component);
    assert!(html.contains("-2147483648"));
}

#[test]
fn test_infinity() {
    let n = f64::INFINITY;
    let component = html! { <span>{n}</span> };
    let html = test::render(&component);
    assert!(html.contains("inf") || html.contains("∞"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 2: Nested Conditional Logic (15 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_nested_if_both_true() {
    let a = true;
    let b = true;
    let component = html! {
        <div>
            @if a {
                @if b {
                    <span>"Both true"</span>
                }
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Both true"));
}

#[test]
fn test_nested_if_outer_false() {
    let a = false;
    let b = true;
    let component = html! {
        <div>
            @if a {
                @if b {
                    <span>"Inner"</span>
                }
            }
        </div>
    };
    let html = test::render(&component);
    assert!(!html.contains("Inner"));
}

#[test]
fn test_nested_if_inner_false() {
    let a = true;
    let b = false;
    let component = html! {
        <div>
            @if a {
                @if b {
                    <span>"Inner"</span>
                }
            }
        </div>
    };
    let html = test::render(&component);
    assert!(!html.contains("Inner"));
}

#[test]
fn test_if_inside_loop() {
    let items = vec![1, 2, 3, 4, 5];
    let component = html! {
        <ul>
            @for item in &items {
                @if *item % 2 == 0 {
                    <li>{item}" (even)"</li>
                }
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("2 (even)") && html.contains("4 (even)"));
    assert!(!html.contains("1 (even)"));
}

#[test]
fn test_loop_inside_if() {
    let show = true;
    let items = vec!["a", "b", "c"];
    let component = html! {
        <div>
            @if show {
                <ul>
                    @for item in &items {
                        <li>{item}</li>
                    }
                </ul>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("a") && html.contains("b") && html.contains("c"));
}

#[test]
fn test_loop_inside_if_false() {
    let show = false;
    let items = vec!["a", "b", "c"];
    let component = html! {
        <div>
            @if show {
                <ul>
                    @for item in &items {
                        <li>{item}</li>
                    }
                </ul>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(!html.contains("<li>"));
}

#[test]
fn test_if_else_chain() {
    let value = 2;
    let component = html! {
        <div>
            @if value == 1 {
                <span>"One"</span>
            } else {
                @if value == 2 {
                    <span>"Two"</span>
                } else {
                    <span>"Other"</span>
                }
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Two"));
}

#[test]
fn test_match_simple() {
    let status = "active";
    let component = html! {
        <div>
            @match status {
                "active" => <span>"Active"</span>,
                "inactive" => <span>"Inactive"</span>,
                _ => <span>"Unknown"</span>,
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Active"));
}

#[test]
fn test_match_wildcard() {
    let status = "pending";
    let component = html! {
        <div>
            @match status {
                "active" => <span>"Active"</span>,
                _ => <span>"Unknown"</span>,
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Unknown"));
}

#[test]
fn test_let_binding() {
    let user = ("Alice", 30);
    let component = html! {
        <div>
            @let (name, age) = user;
            <span>{name}" is "{age}</span>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Alice") && html.contains("30"));
}

#[test]
fn test_nested_loops() {
    let matrix = vec![vec![1, 2], vec![3, 4]];
    let component = html! {
        <table>
            @for row in &matrix {
                <tr>
                    @for cell in row {
                        <td>{cell}</td>
                    }
                </tr>
            }
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("1") && html.contains("4"));
}

#[test]
fn test_triple_nested_if() {
    let a = true;
    let b = true;
    let c = true;
    let component = html! {
        <div>
            @if a {
                @if b {
                    @if c {
                        <span>"All three"</span>
                    }
                }
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("All three"));
}

#[test]
fn test_loop_with_enumerate() {
    let items = vec!["first", "second", "third"];
    let component = html! {
        <ul>
            @for (i, item) in items.iter().enumerate() {
                <li>{i}": "{item}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("0") && html.contains("first"));
}

#[test]
fn test_loop_with_filter() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let component = html! {
        <ul>
            @for n in numbers.iter().filter(|x| *x % 2 == 0) {
                <li>{n}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("2") && html.contains("4") && html.contains("6"));
    assert!(!html.contains(">1<"));
}

#[test]
fn test_loop_with_take() {
    let items = vec!["a", "b", "c", "d", "e"];
    let component = html! {
        <ul>
            @for item in items.iter().take(2) {
                <li>{item}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("a") && html.contains("b"));
    assert!(!html.contains(">c<"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 3: Dynamic Attributes (15 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_dynamic_class() {
    let component = html! { 
        <div class={active}>"Content"</div>
        <style>
            .active { font-weight: "bold"; }
        </style>
    };
    let html = test::render(&component);
    assert!(html.contains("class=\"active\""));
}

#[test]
fn test_dynamic_id() {
    let item_id = "item-123";
    let component = html! { <div id={item_id}>"Content"</div> };
    let html = test::render(&component);
    assert!(html.contains("item-123"));
}

#[test]
fn test_dynamic_href() {
    let url = "/page/1";
    let component = html! { <a href={url}>"Link"</a> };
    let html = test::render(&component);
    assert!(html.contains("href=\"/page/1\""));
}

#[test]
fn test_dynamic_src() {
    let img_url = "/images/photo.jpg";
    let component = html! { <img src={img_url} alt="Photo" /> };
    let html = test::render(&component);
    assert!(html.contains("src=") && html.contains("photo"));
}

#[test]
fn test_dynamic_placeholder() {
    let hint = "Enter your name";
    let component = html! { <input type="text" placeholder={hint} /> };
    let html = test::render(&component);
    assert!(html.contains("Enter your name"));
}

#[test]
fn test_dynamic_data_attribute() {
    let item_id = 42;
    let component = html! { <div data-item-id={item_id}>"Item"</div> };
    let html = test::render(&component);
    assert!(html.contains("data-item-id=") && html.contains("42"));
}

#[test]
fn test_conditional_class() {
    let is_active = true;
    let component = html! { 
        <div class={if is_active { active } else { inactive }}>"Toggle"</div>
        <style>
            .active { font-weight: "bold"; }
            .inactive { opacity: "0.5"; }
        </style>
    };
    let html = test::render(&component);
    assert!(html.contains("active"));
}

#[test]
fn test_computed_attribute() {
    let base = "/api";
    let endpoint = "/users";
    let url = format!("{}{}", base, endpoint);
    let component = html! { <a href={&url}>"API"</a> };
    let html = test::render(&component);
    assert!(html.contains("/api/users"));
}

#[test]
fn test_multiple_dynamic_attrs() {
    let cls = "card";
    let item_id = "card-1";
    let title = "Card Title";
    let component = html! { <div class={cls} id={item_id} title={title}>"Content"</div> };
    let html = test::render(&component);
    assert!(html.contains("card") && html.contains("card-1"));
}

#[test]
fn test_dynamic_disabled() {
    let is_disabled = true;
    let disabled_val = if is_disabled { "true" } else { "false" };
    let component = html! { <button disabled={disabled_val}>"Button"</button> };
    let html = test::render(&component);
    assert!(html.contains("disabled"));
}

#[test]
fn test_dynamic_aria() {
    let expanded = "true";
    let component = html! { <button aria-expanded={expanded}>"Toggle"</button> };
    let html = test::render(&component);
    assert!(html.contains("aria-expanded=\"true\""));
}

#[test]
fn test_numeric_attribute() {
    let rows = 5;
    let component = html! { <textarea rows={rows}>"Text"</textarea> };
    let html = test::render(&component);
    assert!(html.contains("rows=") && html.contains("5"));
}

#[test]
fn test_style_dynamic() {
    let color = "red";
    let fontsize = "16px";
    let component =
        html! { <div style={format!("color: {}; font-size: {}", color, fontsize)}>"Styled"</div> };
    let html = test::render(&component);
    assert!(html.contains("style="));
}

#[test]
fn test_method_in_attribute() {
    let text = "hello";
    let component = html! { <div title={text.to_uppercase()}>"Title"</div> };
    let html = test::render(&component);
    assert!(html.contains("HELLO"));
}

#[test]
fn test_formatted_attribute() {
    let id = 42;
    let component = html! { <div data-ref={format!("item-{}", id)}>"Ref"</div> };
    let html = test::render(&component);
    assert!(html.contains("item-42"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 4: Text Content Variations (10 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_static_text() {
    let component = html! { <p>"Static text"</p> };
    let html = test::render(&component);
    assert!(html.contains("Static text"));
}

#[test]
fn test_dynamic_text() {
    let text = "Dynamic";
    let component = html! { <p>{text}</p> };
    let html = test::render(&component);
    assert!(html.contains("Dynamic"));
}

#[test]
fn test_mixed_static_dynamic() {
    let name = "Alice";
    let component = html! { <p>"Hello, "{name}"!"</p> };
    let html = test::render(&component);
    assert!(html.contains("Hello,") && html.contains("Alice") && html.contains("!"));
}

#[test]
fn test_multiple_dynamic() {
    let first = "John";
    let last = "Doe";
    let component = html! { <p>{first}" "{last}</p> };
    let html = test::render(&component);
    assert!(html.contains("John") && html.contains("Doe"));
}

#[test]
fn test_expression_text() {
    let a = 10;
    let b = 20;
    let component = html! { <p>{a + b}</p> };
    let html = test::render(&component);
    assert!(html.contains("30"));
}

#[test]
fn test_formatted_text() {
    let price = 19.99;
    let component = html! { <p>{format!("${:.2}", price)}</p> };
    let html = test::render(&component);
    assert!(html.contains("$19.99"));
}

#[test]
fn test_multiline_static() {
    let component = html! {
        <pre>
            "Line 1"
            "Line 2"
        </pre>
    };
    let html = test::render(&component);
    assert!(html.contains("Line 1") && html.contains("Line 2"));
}

#[test]
fn test_html_entities_in_text() {
    let text = "5 > 3 & 3 < 5";
    let component = html! { <p>{text}</p> };
    let html = test::render(&component);
    assert!(html.contains("5") && html.contains("3"));
}

#[test]
fn test_quotes_in_text() {
    let text = r#"She said "hello""#;
    let component = html! { <p>{text}</p> };
    let html = test::render(&component);
    assert!(html.contains("hello"));
}

#[test]
fn test_newlines_preserved() {
    let text = "Line1\nLine2";
    let component = html! { <pre>{text}</pre> };
    let html = test::render(&component);
    assert!(html.contains("Line1") && html.contains("Line2"));
}
