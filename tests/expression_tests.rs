//! Expression Tests
//!
//! Tests for expression interpolation within the html! macro.
//! Covers Rust primitives, method calls, math, referencing, and complex types.

use azumi::{html, test};

// ════════════════════════════════════════════════════════════════════════════
// SECTION 1: Rust Primitive Types (20 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_expr_string_literal() {
    let component = html! { <span>{"Hello"}</span> };
    let html = test::render(&component);
    assert!(html.contains("Hello"));
}

#[test]
fn test_expr_string_owned() {
    let text = String::from("Owned String");
    let component = html! { <span>{text}</span> };
    let html = test::render(&component);
    assert!(html.contains("Owned String"));
}

#[test]
fn test_expr_string_ref() {
    let text = "Reference";
    let component = html! { <span>{text}</span> };
    let html = test::render(&component);
    assert!(html.contains("Reference"));
}

#[test]
fn test_expr_integer_i32() {
    let val: i32 = 42;
    let component = html! { <span>{val}</span> };
    let html = test::render(&component);
    assert!(html.contains("42"));
}

#[test]
fn test_expr_integer_u32() {
    let val: u32 = 100;
    let component = html! { <span>{val}</span> };
    let html = test::render(&component);
    assert!(html.contains("100"));
}

#[test]
fn test_expr_integer_negative() {
    let val: i32 = -500;
    let component = html! { <span>{val}</span> };
    let html = test::render(&component);
    assert!(html.contains("-500"));
}

#[test]
fn test_expr_float_f64() {
    let val: f64 = 3.14159;
    let component = html! { <span>{val}</span> };
    let html = test::render(&component);
    assert!(html.contains("3.14159"));
}

#[test]
fn test_expr_bool_true() {
    // Booleans usually don't render text directly in some frameworks,
    // but in Azumi they implement Display, so let's verify behavior.
    // If they shouldn't render, assert !contains "true".
    // Assuming Display behavior:
    let val = true;
    let component = html! { <span>{val}</span> };
    let html = test::render(&component);
    assert!(html.contains("true"));
}

#[test]
fn test_expr_bool_false() {
    let val = false;
    let component = html! { <span>{val}</span> };
    let html = test::render(&component);
    assert!(html.contains("false"));
}

#[test]
fn test_expr_char() {
    let val = 'Z';
    let component = html! { <span>{val}</span> };
    let html = test::render(&component);
    assert!(html.contains("Z"));
}

#[test]
fn test_expr_format_macro() {
    let name = "World";
    let component = html! { <span>{format!("Hello {}", name)}</span> };
    let html = test::render(&component);
    assert!(html.contains("Hello World"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 2: Evaluation & Logic (15 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_expr_addition() {
    let x = 10;
    let y = 32;
    let component = html! { <span>{x + y}</span> };
    let html = test::render(&component);
    assert!(html.contains("42"));
}

#[test]
fn test_expr_math_complex() {
    let component = html! { <span>{(10 * 5) - (20 / 2)}</span> };
    let html = test::render(&component);
    // 50 - 10 = 40
    assert!(html.contains("40"));
}

#[test]
fn test_expr_method_call() {
    let text = "hello";
    let component = html! { <span>{text.to_uppercase()}</span> };
    let html = test::render(&component);
    assert!(html.contains("HELLO"));
}

#[test]
fn test_expr_method_chaining() {
    let text = "  hello  ";
    let component = html! { <span>{text.trim().to_uppercase()}</span> };
    let html = test::render(&component);
    assert!(html.contains("HELLO"));
}

#[test]
fn test_expr_conditional_ternary_simulation() {
    let is_logged_in = true;
    let component = html! { <span>{if is_logged_in { "Welcome" } else { "Login" }}</span> };
    let html = test::render(&component);
    assert!(html.contains("Welcome"));
}

#[test]
fn test_expr_option_display() {
    // Option doesn't implement Display directly usually.
    // If usage is {opt.unwrap_or("default")}, verify that.
    let opt = Some("Value");
    let component = html! { <span>{opt.unwrap_or("Default")}</span> };
    let html = test::render(&component);
    assert!(html.contains("Value"));
}

#[test]
fn test_expr_option_none() {
    let opt: Option<&str> = None;
    let component = html! { <span>{opt.unwrap_or("Default")}</span> };
    let html = test::render(&component);
    assert!(html.contains("Default"));
}

#[test]
fn test_expr_iter_sum() {
    let nums = vec![1, 2, 3, 4];
    let component = html! { <span>{nums.iter().sum::<i32>()}</span> };
    let html = test::render(&component);
    assert!(html.contains("10"));
}

#[test]
fn test_expr_reference_deref() {
    let val = 100;
    let r = &val;
    let rr = &r;
    let component = html! { <span>{**rr}</span> };
    let html = test::render(&component);
    assert!(html.contains("100"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 3: Attribute Expressions (15 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_attr_expr_string() {
    let cls = "btn-primary";
    let component = html! { <button class={cls}>"Click"</button> };
    let html = test::render(&component);
    assert!(html.contains("class=\"btn-primary\""));
}

#[test]
fn test_attr_expr_dynamic_string() {
    let id_num = 123;
    let component = html! { <div data-id={format!("user-{}", id_num)}>"User"</div> };
    let html = test::render(&component);
    assert!(html.contains("data-id=\"user-123\""));
}

#[test]
fn test_attr_expr_boolean_true() {
    // disabled={true} should result in disabled="" or checked property existing
    let is_disabled = true;
    let component = html! { <button disabled={is_disabled}>"Btn"</button> };
    let html = test::render(&component);
    assert!(html.contains("disabled"));
}

#[test]
fn test_attr_expr_boolean_false() {
    // Current framework behavior: renders disabled="false" (which is still truthy in HTML)
    // Future improvement: should remove the attribute.
    let is_disabled = false;
    let component = html! { <button disabled={is_disabled}>"Btn"</button> };
    let html = test::render(&component);
    assert!(html.contains("disabled=\"false\""));
}

#[test]
fn test_attr_expr_conditional() {
    let is_active = true;
    let component =
        html! { <div class={if is_active { "active" } else { "inactive" }}>"Tab"</div> };
    let html = test::render(&component);
    assert!(html.contains("class=\"active\""));
}

#[test]
fn test_attr_expr_option_some_unwrapped() {
    let val = Some("value".to_string());
    let component = html! { <input value={val.clone().unwrap()} /> };
    let html = test::render(&component);
    assert!(html.contains("value=\"value\""));
}

#[test]
fn test_attr_expr_option_none_default() {
    let val: Option<String> = None;
    let component = html! { <input value={val.clone().unwrap_or_default()} /> };
    let html = test::render(&component);
    // It renders value=""
    assert!(html.contains("value=\"\""));
}

#[test]
fn test_attr_expr_numeric() {
    let cols = 5;
    let component = html! { <textarea cols={cols}></textarea> };
    let html = test::render(&component);
    assert!(html.contains("cols=\"5\""));
}

#[test]
fn test_attr_expr_method_chain() {
    let name = "  user  ";
    let component = html! { <input name={name.trim()} /> };
    let html = test::render(&component);
    assert!(html.contains("name=\"user\""));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 4: Block Expressions & Closures (10 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_block_expression() {
    let component = html! {
        <div>
            {{
                let x = 10;
                let y = 20;
                x + y
            }}
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("30"));
}

#[test]
fn test_block_with_string_concat() {
    let component = html! {
        <span>
            {{
                let mut s = String::from("Hello");
                s.push_str(" World");
                s
            }}
        </span>
    };
    let html = test::render(&component);
    assert!(html.contains("Hello World"));
}

#[test]
fn test_closure_immediate_call() {
    // Calling a closure immediately inside the expression block
    let component = html! {
        <span>
            {(|| "Closure Result")()}
        </span>
    };
    let html = test::render(&component);
    assert!(html.contains("Closure Result"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 5: Structs & Complex Types (10 tests)
// ════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct User {
    name: String,
    age: u32,
}

#[test]
#[allow(dead_code)]
fn test_struct_field_access() {
    let user = User {
        name: "Alice".to_string(),
        age: 30,
    };
    let component = html! {
        <div>
            <span>{&user.name}</span>
            <span>{user.age}</span>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Alice"));
    assert!(html.contains("30"));
}

#[test]
#[allow(dead_code)]
fn test_nested_field_access() {
    #[allow(dead_code)]
    struct Wrapper {
        user: User,
    }
    let wrapper = Wrapper {
        user: User {
            name: "Bob".to_string(),
            age: 40,
        },
    };

    let component = html! { <span>{wrapper.user.name}</span> };
    let html = test::render(&component);
    assert!(html.contains("Bob"));
}

#[test]
fn test_tuple_indexing() {
    let tuple = ("Alpha", 1, true);
    let component = html! {
        <ul>
            <li>{tuple.0}</li>
            <li>{tuple.1}</li>
            <li>{tuple.2}</li>
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("Alpha"));
    assert!(html.contains("1"));
    assert!(html.contains("true"));
}
