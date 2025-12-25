//! Control Flow Tests
//!
//! Tests for Azumi's control flow constructs: @if, @for, @match, @let
//! Run with: cargo test --features test-utils

use azumi::{html, test};

// ════════════════════════════════════════════════════════════════════════════
// SECTION 1: @if Conditionals (10 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_if_true_branch() {
    let show = true;
    let component = html! {
        <div>
            {if show { "Visible" } else { "Hidden" }}
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Visible"));
    assert!(!html.contains("Hidden"));
}

#[test]
fn test_if_false_branch() {
    let show = false;
    let component = html! {
        <div>
            {if show { "Visible" } else { "Hidden" }}
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Hidden"));
    assert!(!html.contains("Visible"));
}

#[test]
fn test_if_without_else() {
    let show = true;
    let component = html! {
        <div>
            @if show {
                <span>"Shown"</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Shown"));
}

#[test]
fn test_if_false_no_else_empty() {
    let show = false;
    let component = html! {
        <div>
            @if show {
                <span>"Shown"</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(!html.contains("Shown"));
}

#[test]
fn test_if_with_complex_condition() {
    let count = 5;
    let component = html! {
        <div>
            {if count > 3 { "Large" } else { "Small" }}
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Large"));
}

#[test]
fn test_if_with_and_condition() {
    let a = true;
    let b = true;
    let component = html! {
        <div>
            {if a && b { "Both true" } else { "Not both" }}
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Both true"));
}

#[test]
fn test_if_with_or_condition() {
    let a = false;
    let b = true;
    let component = html! {
        <div>
            {if a || b { "At least one" } else { "Neither" }}
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("At least one"));
}

#[test]
fn test_nested_if() {
    let level = 2;
    let component = html! {
        <div>
            {if level > 0 {
                if level > 1 { "Level 2+" } else { "Level 1" }
            } else {
                "Level 0"
            }}
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Level 2+"));
}

#[test]
fn test_if_with_element_branches() {
    let is_error = true;
    let component = html! {
        <div>
            @if is_error {
                <span>"Error!"</span>
            } else {
                <span>"Success"</span>
            }
        </div>
    };
    let html = test::render(&component);
    test::assert_selector(&html, "span", Some("Error!"));
}

#[test]
fn test_if_equality_check() {
    let status = "active";
    let component = html! {
        <div>
            {if status == "active" { "Active" } else { "Inactive" }}
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Active"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 2: @if let Pattern Matching (8 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_if_let_some() {
    let value: Option<&str> = Some("Hello");
    let component = html! {
        <div>
            @if let Some(v) = value {
                <span>{v}</span>
            }
        </div>
    };
    let html = test::render(&component);
    test::assert_selector(&html, "span", Some("Hello"));
}

#[test]
fn test_if_let_none() {
    let value: Option<&str> = None;
    let component = html! {
        <div>
            @if let Some(v) = value {
                <span>{v}</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(!html.contains("<span>"));
}

#[test]
fn test_if_let_with_else() {
    let value: Option<i32> = None;
    let component = html! {
        <div>
            @if let Some(n) = value {
                <span>"Number: "{n}</span>
            } else {
                <span>"No number"</span>
            }
        </div>
    };
    let html = test::render(&component);
    test::assert_selector(&html, "span", Some("No number"));
}

#[test]
fn test_if_let_result_ok() {
    let result: Result<&str, &str> = Ok("Success");
    let component = html! {
        <div>
            @if let Ok(msg) = result {
                <span>{msg}</span>
            }
        </div>
    };
    let html = test::render(&component);
    test::assert_selector(&html, "span", Some("Success"));
}

#[test]
fn test_if_let_result_err() {
    let result: Result<&str, &str> = Err("Failed");
    let component = html! {
        <div>
            @if let Err(e) = result {
                <span>"Error: "{e}</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Error:") && html.contains("Failed"));
}

#[test]
fn test_if_let_tuple() {
    let pair: Option<(i32, &str)> = Some((42, "answer"));
    let component = html! {
        <div>
            @if let Some((num, text)) = pair {
                <span>{num}": "{text}</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("42") && html.contains("answer"));
}

#[test]
fn test_if_let_nested_option() {
    let nested: Option<Option<&str>> = Some(Some("deep"));
    let component = html! {
        <div>
            @if let Some(Some(val)) = nested {
                <span>{val}</span>
            }
        </div>
    };
    let html = test::render(&component);
    test::assert_selector(&html, "span", Some("deep"));
}

#[test]
fn test_if_let_struct_destructure() {
    struct Point {
        x: i32,
        y: i32,
    }
    let point: Option<Point> = Some(Point { x: 10, y: 20 });
    let component = html! {
        <div>
            @if let Some(Point { x, y }) = point {
                <span>"("{x}", "{y}")"</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("10") && html.contains("20"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 3: @for Loops (10 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_for_vec_strings() {
    let items = vec!["Apple", "Banana", "Cherry"];
    let component = html! {
        <ul>
            @for item in &items {
                <li>{item}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("Apple") && html.contains("Banana") && html.contains("Cherry"));
}

#[test]
fn test_for_empty_vec() {
    let items: Vec<&str> = vec![];
    let component = html! {
        <ul>
            @for item in &items {
                <li>{item}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    // Should have ul but no li
    assert!(html.contains("<ul>") && !html.contains("<li>"));
}

#[test]
fn test_for_range() {
    let component = html! {
        <ul>
            @for i in 1..=3 {
                <li>"Item "{i}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("1") && html.contains("2") && html.contains("3"));
}

#[test]
fn test_for_with_index() {
    let items = vec!["A", "B", "C"];
    let component = html! {
        <ul>
            @for (idx, item) in items.iter().enumerate() {
                <li>{idx}": "{item}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("0") && html.contains("A"));
}

#[test]
fn test_for_numbers() {
    let numbers = vec![1, 2, 3, 4, 5];
    let component = html! {
        <div>
            @for n in &numbers {
                <span>{n}</span>
            }
        </div>
    };
    let html = test::render(&component);
    for n in 1..=5 {
        assert!(html.contains(&n.to_string()));
    }
}

#[test]
fn test_for_struct_items() {
    struct User {
        name: String,
    }
    let users = vec![
        User {
            name: "Alice".into(),
        },
        User { name: "Bob".into() },
    ];
    let component = html! {
        <ul>
            @for user in &users {
                <li>{&user.name}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("Alice") && html.contains("Bob"));
}

#[test]
fn test_for_nested() {
    let matrix = vec![vec![1, 2], vec![3, 4]];
    let component = html! {
        <div>
            @for row in &matrix {
                <div>
                    @for cell in row {
                        <span>{cell}</span>
                    }
                </div>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("1") && html.contains("2") && html.contains("3") && html.contains("4"));
}

#[test]
fn test_for_with_conditionals() {
    let numbers = vec![1, 2, 3, 4, 5];
    let component = html! {
        <ul>
            @for n in &numbers {
                <li>
                    {if *n % 2 == 0 { "Even" } else { "Odd" }}
                </li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("Even") && html.contains("Odd"));
}

#[test]
fn test_for_filter_iterator() {
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
}

#[test]
fn test_for_map_iterator() {
    let numbers = vec![1, 2, 3];
    let component = html! {
        <ul>
            @for n in numbers.iter().map(|x| x * 2) {
                <li>{n}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("2") && html.contains("4") && html.contains("6"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 4: @match Pattern Matching (8 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_match_string() {
    let status = "active";
    let component = html! {
        <div>
            @match status {
                "active" => { <span>"✓ Active"</span> }
                "pending" => { <span>"⏳ Pending"</span> }
                _ => { <span>"Unknown"</span> }
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("✓ Active"));
}

#[test]
fn test_match_number() {
    let level = 2;
    let component = html! {
        <div>
            @match level {
                1 => { "Beginner" }
                2 => { "Intermediate" }
                3 => { "Advanced" }
                _ => { "Unknown" }
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Intermediate"));
}

#[test]
fn test_match_option() {
    let value: Option<&str> = Some("Hello");
    let component = html! {
        <div>
            @match value {
                Some(v) => { <span>{v}</span> }
                None => { <span>"Empty"</span> }
            }
        </div>
    };
    let html = test::render(&component);
    test::assert_selector(&html, "span", Some("Hello"));
}

#[test]
fn test_match_option_none() {
    let value: Option<&str> = None;
    let component = html! {
        <div>
            @match value {
                Some(v) => { <span>{v}</span> }
                None => { <span>"Empty"</span> }
            }
        </div>
    };
    let html = test::render(&component);
    test::assert_selector(&html, "span", Some("Empty"));
}

#[test]
fn test_match_result() {
    let result: Result<i32, &str> = Ok(42);
    let component = html! {
        <div>
            @match result {
                Ok(n) => { <span>"Success: "{n}</span> }
                Err(e) => { <span>"Error: "{e}</span> }
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Success:") && html.contains("42"));
}

#[test]
fn test_match_enum() {
    #[derive(Clone)]
    #[allow(dead_code)]
    enum Status {
        Loading,
        Success,
        Error,
    }
    let status = Status::Success;
    let component = html! {
        <div>
            @match status {
                Status::Loading => { "Loading..." }
                Status::Success => { "Done!" }
                Status::Error => { "Failed" }
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Done!"));
}

#[test]
fn test_match_with_guard() {
    let score = 85;
    let component = html! {
        <div>
            @match score {
                n if n >= 90 => { "A" }
                n if n >= 80 => { "B" }
                n if n >= 70 => { "C" }
                _ => { "F" }
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("B"));
}

#[test]
fn test_match_tuple() {
    let coords = (0, 1);
    let component = html! {
        <div>
            @match coords {
                (0, 0) => { "Origin" }
                (0, _) => { "Y-axis" }
                (_, 0) => { "X-axis" }
                _ => { "Somewhere" }
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Y-axis"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 5: @let Local Variables (6 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_let_simple() {
    let component = html! {
        <div>
            @let greeting = "Hello World";
            <span>{greeting}</span>
        </div>
    };
    let html = test::render(&component);
    test::assert_selector(&html, "span", Some("Hello World"));
}

#[test]
fn test_let_expression() {
    let price = 100;
    let tax_rate = 0.1;
    let component = html! {
        <div>
            @let total = (price as f64) * (1.0 + tax_rate);
            <span>"Total: "{total}</span>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("110"));
}

#[test]
fn test_let_method_chain() {
    let text = "  hello world  ";
    let component = html! {
        <div>
            @let cleaned = text.trim().to_uppercase();
            <span>{cleaned}</span>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("HELLO WORLD"));
}

#[test]
fn test_let_multiple() {
    let component = html! {
        <div>
            @let first = "Hello";
            @let second = "World";
            <span>{first}" "{second}</span>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Hello") && html.contains("World"));
}

#[test]
fn test_let_used_in_condition() {
    let items = vec![1, 2, 3];
    let component = html! {
        <div>
            @let count = items.len();
            @if count > 0 {
                <span>{count}</span><span>" items"</span>
            } else {
                <span>"No items"</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("3") && html.contains("items"));
}

#[test]
fn test_let_in_loop() {
    let prices = vec![10, 20, 30];
    let component = html! {
        <ul>
            @for price in &prices {
                @let discounted = price * 9 / 10;
                <li>{discounted}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("9") && html.contains("18") && html.contains("27"));
}
