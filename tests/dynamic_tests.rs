//! Dynamic Content Tests
//!
//! Tests for dynamic content generation
//! Run with: cargo test

use azumi::{html, test};

// ════════════════════════════════════════════════════════════════════════════
// SECTION 1: Conditional Content (20 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_if_true() {
    let show = true;
    let component = html! {
        <div>
            @if show {
                <span>"Visible"</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Visible"));
}

#[test]
fn test_if_false() {
    let show = false;
    let component = html! {
        <div>
            @if show {
                <span>"Hidden"</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(!html.contains("Hidden"));
}

#[test]
fn test_if_else_true() {
    let logged_in = true;
    let component = html! {
        <div>
            @if logged_in {
                <span>"Welcome back!"</span>
            } else {
                <span>"Please sign in"</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Welcome back!") && !html.contains("Please sign in"));
}

#[test]
fn test_if_else_false() {
    let logged_in = false;
    let component = html! {
        <div>
            @if logged_in {
                <span>"Welcome back!"</span>
            } else {
                <span>"Please sign in"</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Please sign in") && !html.contains("Welcome back!"));
}

#[test]
fn test_if_let_some() {
    let user: Option<&str> = Some("Alice");
    let component = html! {
        <div>
            @if let Some(name) = user {
                <span>"Hello, "{name}</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Alice"));
}

#[test]
fn test_if_let_none() {
    let user: Option<&str> = None;
    let component = html! {
        <div>
            @if let Some(name) = user {
                <span>"Hello, "{name}</span>
            } else {
                <span>"Guest"</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Guest"));
}

#[test]
fn test_nested_if() {
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
fn test_if_comparison_greater() {
    let count = 5;
    let component = html! {
        <div>
            @if count > 3 {
                <span>"High"</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("High"));
}

#[test]
fn test_if_comparison_less() {
    let count = 2;
    let component = html! {
        <div>
            @if count < 3 {
                <span>"Low"</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Low"));
}

#[test]
fn test_if_equals() {
    let status = "active";
    let component = html! {
        <div>
            @if status == "active" {
                <span>"Active"</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Active"));
}

#[test]
fn test_if_not_equals() {
    let status = "inactive";
    let component = html! {
        <div>
            @if status != "active" {
                <span>"Not Active"</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Not Active"));
}

#[test]
fn test_if_and() {
    let a = true;
    let b = true;
    let component = html! {
        <div>
            @if a && b {
                <span>"Both"</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Both"));
}

#[test]
fn test_if_or() {
    let a = false;
    let b = true;
    let component = html! {
        <div>
            @if a || b {
                <span>"Either"</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Either"));
}

#[test]
fn test_if_not() {
    let hidden = false;
    let component = html! {
        <div>
            @if !hidden {
                <span>"Shown"</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Shown"));
}

#[test]
fn test_conditional_class() {
    let is_error = true;
    let class_name = if is_error { "error" } else { "success" };
    let component = html! { <div data-class={class_name}>"Message"</div> };
    let html = test::render(&component);
    assert!(html.contains("error"));
}

#[test]
fn test_conditional_attribute() {
    let disabled = true;
    let component = html! { <button disabled={disabled.to_string()}>"Click"</button> };
    let html = test::render(&component);
    assert!(html.contains("disabled"));
}

#[test]
fn test_empty_condition() {
    let items: Vec<&str> = vec![];
    let component = html! {
        <div>
            @if items.is_empty() {
                <p>"No items"</p>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("No items"));
}

#[test]
fn test_len_condition() {
    let items = vec!["a", "b", "c"];
    let component = html! {
        <div>
            @if items.len() >= 3 {
                <p>"Many items"</p>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Many items"));
}

#[test]
fn test_contains_condition() {
    let text = "hello world";
    let component = html! {
        <div>
            @if text.contains("world") {
                <span>"Found it!"</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Found it!"));
}

#[test]
fn test_starts_with_condition() {
    let text = "hello world";
    let component = html! {
        <div>
            @if text.starts_with("hello") {
                <span>"Starts correctly"</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Starts correctly"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 2: Loop Content (20 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_for_simple() {
    let items = vec!["a", "b", "c"];
    let component = html! {
        <ul>
            @for item in &items {
                <li>{item}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("<li>a</li>") && html.contains("<li>c</li>"));
}

#[test]
fn test_for_with_index() {
    let items = vec!["first", "second"];
    let component = html! {
        <ol>
            @for (i, item) in items.iter().enumerate() {
                <li>{i}": "{item}</li>
            }
        </ol>
    };
    let html = test::render(&component);
    assert!(html.contains("0: first") && html.contains("1: second"));
}

#[test]
fn test_for_tuple() {
    let pairs = vec![("key1", "val1"), ("key2", "val2")];
    let component = html! {
        <dl>
            @for (k, v) in &pairs {
                <dt>{k}</dt>
                <dd>{v}</dd>
            }
        </dl>
    };
    let html = test::render(&component);
    assert!(html.contains("key1") && html.contains("val2"));
}

#[test]
fn test_for_range() {
    let component = html! {
        <ul>
            @for i in 1..=5 {
                <li>{i}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains(">1<") && html.contains(">5<"));
}

#[test]
fn test_for_filter() {
    let nums = vec![1, 2, 3, 4, 5, 6];
    let component = html! {
        <ul>
            @for n in nums.iter().filter(|x| **x % 2 == 0) {
                <li>{n}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains(">2<") && html.contains(">4<") && html.contains(">6<"));
}

#[test]
fn test_for_map() {
    let nums = vec![1, 2, 3];
    let component = html! {
        <ul>
            @for n in nums.iter().map(|x| x * 10) {
                <li>{n}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains(">10<") && html.contains(">30<"));
}

#[test]
fn test_for_take() {
    let nums = vec![1, 2, 3, 4, 5];
    let component = html! {
        <ul>
            @for n in nums.iter().take(2) {
                <li>{n}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains(">1<") && html.contains(">2<") && !html.contains(">3<"));
}

#[test]
fn test_for_skip() {
    let nums = vec![1, 2, 3, 4, 5];
    let component = html! {
        <ul>
            @for n in nums.iter().skip(3) {
                <li>{n}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains(">4<") && html.contains(">5<") && !html.contains(">1<"));
}

#[test]
fn test_for_chain() {
    let a = vec![1, 2];
    let b = vec![3, 4];
    let component = html! {
        <ul>
            @for n in a.iter().chain(b.iter()) {
                <li>{n}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains(">1<") && html.contains(">4<"));
}

#[test]
fn test_for_rev() {
    let nums = vec![1, 2, 3];
    let component = html! {
        <ul>
            @for n in nums.iter().rev() {
                <li>{n}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains(">3<") && html.contains(">1<"));
}

#[test]
fn test_nested_for() {
    let rows = vec![vec![1, 2], vec![3, 4]];
    let component = html! {
        <table>
            @for row in &rows {
                <tr>
                    @for cell in row {
                        <td>{cell}</td>
                    }
                </tr>
            }
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains(">1<") && html.contains(">4<"));
}

#[test]
fn test_for_empty() {
    let items: Vec<&str> = vec![];
    let component = html! {
        <ul>
            @for item in &items {
                <li>{item}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("<ul>") && !html.contains("<li>"));
}

#[test]
fn test_for_single() {
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
fn test_for_with_if() {
    let items = vec![("a", true), ("b", false), ("c", true)];
    let component = html! {
        <ul>
            @for (name, show) in &items {
                @if *show {
                    <li>{name}</li>
                }
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("a") && html.contains("c") && !html.contains(">b<"));
}

#[test]
fn test_for_complex_item() {
    struct Item {
        name: String,
        price: f64,
    }
    let items = vec![
        Item {
            name: "Apple".to_string(),
            price: 1.99,
        },
        Item {
            name: "Banana".to_string(),
            price: 0.99,
        },
    ];
    let component = html! {
        <ul>
            @for item in &items {
                <li>{&item.name}": $"{item.price}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("Apple") && html.contains("0.99"));
}

#[test]
fn test_for_chars() {
    let text = "abc";
    let component = html! {
        <ul>
            @for c in text.chars() {
                <li>{c}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains(">a<") && html.contains(">c<"));
}

#[test]
fn test_for_split() {
    let text = "one,two,three";
    let component = html! {
        <ul>
            @for part in text.split(',') {
                <li>{part}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("one") && html.contains("three"));
}

#[test]
fn test_for_lines() {
    let text = "line1\nline2\nline3";
    let component = html! {
        <ul>
            @for line in text.lines() {
                <li>{line}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("line1") && html.contains("line3"));
}

#[test]
fn test_for_bytes() {
    let data = vec![65u8, 66u8, 67u8]; // ABC
    let component = html! {
        <ul>
            @for b in &data {
                <li>{b}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("65") && html.contains("67"));
}

#[test]
fn test_for_step_by() {
    let component = html! {
        <ul>
            @for i in (0..10).step_by(2) {
                <li>{i}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains(">0<") && html.contains(">8<") && !html.contains(">1<"));
}
