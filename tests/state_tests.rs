//! State Management Tests
//!
//! Comprehensive tests for state handling and data binding
//! Run with: cargo test

use azumi::{html, test};

// ════════════════════════════════════════════════════════════════════════════
// SECTION 1: Primitive Types (20 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_i8() {
    let n: i8 = -128;
    let component = html! { <span>{n}</span> };
    let html = test::render(&component);
    assert!(html.contains("-128"));
}

#[test]
fn test_i16() {
    let n: i16 = -32768;
    let component = html! { <span>{n}</span> };
    let html = test::render(&component);
    assert!(html.contains("-32768"));
}

#[test]
fn test_i32_positive() {
    let n: i32 = 2147483647;
    let component = html! { <span>{n}</span> };
    let html = test::render(&component);
    assert!(html.contains("2147483647"));
}

#[test]
fn test_i64_large() {
    let n: i64 = 9223372036854775807;
    let component = html! { <span>{n}</span> };
    let html = test::render(&component);
    assert!(html.contains("9223372036854775807"));
}

#[test]
fn test_u8() {
    let n: u8 = 255;
    let component = html! { <span>{n}</span> };
    let html = test::render(&component);
    assert!(html.contains("255"));
}

#[test]
fn test_u16() {
    let n: u16 = 65535;
    let component = html! { <span>{n}</span> };
    let html = test::render(&component);
    assert!(html.contains("65535"));
}

#[test]
fn test_u32() {
    let n: u32 = 4294967295;
    let component = html! { <span>{n}</span> };
    let html = test::render(&component);
    assert!(html.contains("4294967295"));
}

#[test]
fn test_u64() {
    let n: u64 = 18446744073709551615;
    let component = html! { <span>{n}</span> };
    let html = test::render(&component);
    assert!(html.contains("18446744073709551615"));
}

#[test]
fn test_f32_decimal() {
    let n: f32 = 3.14159;
    let component = html! { <span>{n}</span> };
    let html = test::render(&component);
    assert!(html.contains("3.14"));
}

#[test]
fn test_f64_precision() {
    let n: f64 = 3.141592653589793;
    let component = html! { <span>{n}</span> };
    let html = test::render(&component);
    assert!(html.contains("3.14159"));
}

#[test]
fn test_bool_true() {
    let b = true;
    let component = html! { <span>{b}</span> };
    let html = test::render(&component);
    assert!(html.contains("true"));
}

#[test]
fn test_bool_false() {
    let b = false;
    let component = html! { <span>{b}</span> };
    let html = test::render(&component);
    assert!(html.contains("false"));
}

#[test]
fn test_char() {
    let c = 'X';
    let component = html! { <span>{c}</span> };
    let html = test::render(&component);
    assert!(html.contains("X"));
}

#[test]
fn test_char_unicode() {
    let c = '日';
    let component = html! { <span>{c}</span> };
    let html = test::render(&component);
    assert!(html.contains("日"));
}

#[test]
fn test_string_owned() {
    let s = String::from("Hello");
    let component = html! { <span>{&s}</span> };
    let html = test::render(&component);
    assert!(html.contains("Hello"));
}

#[test]
fn test_string_slice() {
    let s = "World";
    let component = html! { <span>{s}</span> };
    let html = test::render(&component);
    assert!(html.contains("World"));
}

#[test]
fn test_usize() {
    let n: usize = 12345;
    let component = html! { <span>{n}</span> };
    let html = test::render(&component);
    assert!(html.contains("12345"));
}

#[test]
fn test_isize() {
    let n: isize = -12345;
    let component = html! { <span>{n}</span> };
    let html = test::render(&component);
    assert!(html.contains("-12345"));
}

#[test]
fn test_unit_type() {
    // Unit type can't be displayed, just test no panic
    let _unit = ();
    let component = html! { <span>"Unit test"</span> };
    let html = test::render(&component);
    assert!(html.contains("Unit test"));
}

#[test]
fn test_zero_values() {
    let i = 0i32;
    let component = html! { <span>{i}</span> };
    let html = test::render(&component);
    assert!(html.contains("0"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 2: Collections (20 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_vec_empty() {
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
fn test_vec_single() {
    let items = vec!["one"];
    let component = html! {
        <ul>
            @for item in &items {
                <li>{item}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("one"));
}

#[test]
fn test_vec_multiple() {
    let items = vec!["a", "b", "c"];
    let component = html! {
        <ul>
            @for item in &items {
                <li>{item}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("a") && html.contains("b") && html.contains("c"));
}

#[test]
fn test_vec_integers() {
    let nums = vec![1, 2, 3, 4, 5];
    let component = html! {
        <ul>
            @for n in &nums {
                <li>{n}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("1") && html.contains("5"));
}

#[test]
fn test_vec_tuples() {
    let pairs = vec![("a", 1), ("b", 2)];
    let component = html! {
        <ul>
            @for (k, v) in &pairs {
                <li>{k}": "{v}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("a: 1") && html.contains("b: 2"));
}

#[test]
fn test_vec_enumerate() {
    let items = vec!["first", "second", "third"];
    let component = html! {
        <ol>
            @for (i, item) in items.iter().enumerate() {
                <li>{i}". "{item}</li>
            }
        </ol>
    };
    let html = test::render(&component);
    assert!(html.contains("0. first") && html.contains("2. third"));
}

#[test]
fn test_vec_filter() {
    let nums = vec![1, 2, 3, 4, 5, 6];
    let component = html! {
        <ul>
            @for n in nums.iter().filter(|x| *x % 2 == 0) {
                <li>{n}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("2") && html.contains("4") && html.contains("6"));
    assert!(!html.contains(">1<") && !html.contains(">3<"));
}

#[test]
fn test_vec_map() {
    let nums = vec![1, 2, 3];
    let component = html! {
        <ul>
            @for n in nums.iter().map(|x| x * 2) {
                <li>{n}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("2") && html.contains("4") && html.contains("6"));
}

#[test]
fn test_vec_take() {
    let nums = vec![1, 2, 3, 4, 5];
    let component = html! {
        <ul>
            @for n in nums.iter().take(3) {
                <li>{n}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("1") && html.contains("3"));
    assert!(!html.contains(">4<"));
}

#[test]
fn test_vec_skip() {
    let nums = vec![1, 2, 3, 4, 5];
    let component = html! {
        <ul>
            @for n in nums.iter().skip(2) {
                <li>{n}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("3") && html.contains("5"));
    assert!(!html.contains(">1<"));
}

#[test]
fn test_vec_rev() {
    let nums = vec![1, 2, 3];
    let component = html! {
        <ul>
            @for n in nums.iter().rev() {
                <li>{n}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("3") && html.contains("1"));
}

#[test]
fn test_range() {
    let component = html! {
        <ul>
            @for i in 1..=5 {
                <li>{i}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("1") && html.contains("5"));
}

#[test]
fn test_range_exclusive() {
    let component = html! {
        <ul>
            @for i in 0..3 {
                <li>{i}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("0") && html.contains("2"));
    assert!(!html.contains(">3<"));
}

#[test]
fn test_option_some() {
    let val: Option<&str> = Some("present");
    let component = html! {
        <div>
            @if let Some(v) = val {
                <span>{v}</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("present"));
}

#[test]
fn test_option_none() {
    let val: Option<&str> = None;
    let component = html! {
        <div>
            @if let Some(v) = val {
                <span>{v}</span>
            } else {
                <span>"None"</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("None"));
}

#[test]
fn test_result_ok() {
    let res: Result<&str, &str> = Ok("success");
    let component = html! {
        <div>
            @if let Ok(v) = res {
                <span>{v}</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("success"));
}

#[test]
fn test_result_err() {
    let res: Result<&str, &str> = Err("error");
    let component = html! {
        <div>
            @if let Ok(v) = res {
                <span>{v}</span>
            } else {
                <span>"Error"</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Error"));
}

#[test]
fn test_nested_vec() {
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
fn test_slice() {
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];
    let component = html! {
        <ul>
            @for n in slice {
                <li>{n}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("2") && html.contains("4"));
}

#[test]
fn test_array() {
    let arr = [10, 20, 30];
    let component = html! {
        <ul>
            @for n in &arr {
                <li>{n}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("10") && html.contains("30"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 3: Structs (15 tests)
// ════════════════════════════════════════════════════════════════════════════

struct User {
    name: String,
    age: u32,
    email: String,
}

#[test]
fn test_struct_field_access() {
    let user = User {
        name: "Alice".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
    };
    let component = html! { <div>{&user.name}</div> };
    let html = test::render(&component);
    assert!(html.contains("Alice"));
}

#[test]
fn test_struct_multiple_fields() {
    let user = User {
        name: "Bob".to_string(),
        age: 25,
        email: "bob@example.com".to_string(),
    };
    let component = html! {
        <div>
            <span>{&user.name}</span>
            <span>{user.age}</span>
            <span>{&user.email}</span>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Bob") && html.contains("25") && html.contains("bob@example.com"));
}

struct Point {
    x: i32,
    y: i32,
}

#[test]
fn test_simple_struct() {
    let p = Point { x: 10, y: 20 };
    let component = html! { <span>"("{p.x}", "{p.y}")"</span> };
    let html = test::render(&component);
    assert!(html.contains("10") && html.contains("20"));
}

struct Config {
    enabled: bool,
    max_size: usize,
    name: String,
}

#[test]
fn test_config_struct() {
    let cfg = Config {
        enabled: true,
        max_size: 1024,
        name: "Main".to_string(),
    };
    let component = html! {
        <div>
            @if cfg.enabled {
                <span>{&cfg.name}": "{cfg.max_size}</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Main") && html.contains("1024"));
}

struct Product {
    title: String,
    price: f64,
    in_stock: bool,
}

#[test]
fn test_product_struct() {
    let p = Product {
        title: "Widget".to_string(),
        price: 19.99,
        in_stock: true,
    };
    let component = html! {
        <article>
            <h3>{&p.title}</h3>
            <span>{"$"}{p.price}</span>
            @if p.in_stock {
                <span>"In Stock"</span>
            }
        </article>
    };
    let html = test::render(&component);
    assert!(html.contains("Widget") && html.contains("19.99") && html.contains("In Stock"));
}

struct Address {
    street: String,
    city: String,
    zip: String,
}

#[test]
fn test_address_struct() {
    let addr = Address {
        street: "123 Main St".to_string(),
        city: "Springfield".to_string(),
        zip: "12345".to_string(),
    };
    let component = html! {
        <address>
            <p>{&addr.street}</p>
            <p>{&addr.city}", "{&addr.zip}</p>
        </address>
    };
    let html = test::render(&component);
    assert!(html.contains("123 Main St") && html.contains("Springfield") && html.contains("12345"));
}

struct Article {
    title: String,
    content: String,
    tags: Vec<String>,
}

#[test]
fn test_struct_with_vec() {
    let article = Article {
        title: "Hello World".to_string(),
        content: "This is content".to_string(),
        tags: vec!["rust".to_string(), "web".to_string()],
    };
    let component = html! {
        <article>
            <h1>{&article.title}</h1>
            <p>{&article.content}</p>
            <ul>
                @for tag in &article.tags {
                    <li>{tag}</li>
                }
            </ul>
        </article>
    };
    let html = test::render(&component);
    assert!(html.contains("Hello World") && html.contains("rust") && html.contains("web"));
}

struct OptionalFields {
    name: String,
    nickname: Option<String>,
}

#[test]
fn test_struct_optional_some() {
    let person = OptionalFields {
        name: "John".to_string(),
        nickname: Some("Johnny".to_string()),
    };
    let component = html! {
        <div>
            <span>{&person.name}</span>
            @if let Some(nick) = &person.nickname {
                <span>" ("{nick}")"</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("John") && html.contains("Johnny"));
}

#[test]
fn test_struct_optional_none() {
    let person = OptionalFields {
        name: "Jane".to_string(),
        nickname: None,
    };
    let component = html! {
        <div>
            <span>{&person.name}</span>
            @if let Some(nick) = &person.nickname {
                <span>" ("{nick}")"</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Jane"));
    assert!(!html.contains("("));
}

struct Nested {
    inner: Point,
    label: String,
}

#[test]
fn test_nested_struct() {
    let n = Nested {
        inner: Point { x: 5, y: 10 },
        label: "Origin".to_string(),
    };
    let component = html! {
        <div>
            <span>{&n.label}</span>
            <span>{n.inner.x}", "{n.inner.y}</span>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Origin") && html.contains("5") && html.contains("10"));
}

#[derive(Clone)]
struct Item {
    id: u32,
    name: String,
}

#[test]
fn test_vec_of_structs() {
    let items = vec![
        Item {
            id: 1,
            name: "One".to_string(),
        },
        Item {
            id: 2,
            name: "Two".to_string(),
        },
    ];
    let component = html! {
        <ul>
            @for item in &items {
                <li data-id={item.id}>{&item.name}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("One") && html.contains("Two"));
}

struct Counter {
    count: i32,
}

impl Counter {
    fn is_zero(&self) -> bool {
        self.count == 0
    }

    fn display(&self) -> String {
        format!("Count: {}", self.count)
    }
}

#[test]
fn test_struct_method() {
    let c = Counter { count: 5 };
    let component = html! { <span>{c.display()}</span> };
    let html = test::render(&component);
    assert!(html.contains("Count: 5"));
}

#[test]
fn test_struct_bool_method() {
    let c = Counter { count: 0 };
    let component = html! {
        <div>
            @if c.is_zero() {
                <span>"Zero!"</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Zero!"));
}

#[test]
fn test_struct_computed() {
    let p = Point { x: 3, y: 4 };
    let dist = ((p.x * p.x + p.y * p.y) as f64).sqrt();
    let component = html! { <span>"Distance: "{dist}</span> };
    let html = test::render(&component);
    assert!(html.contains("5"));
}
