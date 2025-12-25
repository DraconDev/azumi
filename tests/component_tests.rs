//! Component System Tests
//!
//! Tests for Azumi's component props, children, and composition
//! Run with: cargo test --features test-utils

use azumi::{html, test};

// ════════════════════════════════════════════════════════════════════════════
// SECTION 1: Basic Component Props (10 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_string_prop() {
    let name = "World";
    let component = html! {
        <div>"Hello, "{name}"!"</div>
    };
    let html = test::render(&component);
    assert!(html.contains("Hello") && html.contains("World"));
}

#[test]
fn test_empty_string_prop() {
    let name = "";
    let component = html! {
        <span>"Name: "{name}</span>
    };
    let html = test::render(&component);
    assert!(html.contains("Name:"));
}

#[test]
fn test_number_prop() {
    let count = 42;
    let component = html! {
        <span>"Count: "{count}</span>
    };
    let html = test::render(&component);
    assert!(html.contains("42"));
}

#[test]
fn test_zero_prop() {
    let count = 0;
    let component = html! {
        <span>"Count: "{count}</span>
    };
    let html = test::render(&component);
    assert!(html.contains("0"));
}

#[test]
fn test_negative_prop() {
    let value = -10;
    let component = html! {
        <span>"Value: "{value}</span>
    };
    let html = test::render(&component);
    assert!(html.contains("-10"));
}

#[test]
fn test_bool_true_prop() {
    let active = true;
    let component = html! {
        <div>{if active { "ON" } else { "OFF" }}</div>
    };
    let html = test::render(&component);
    assert!(html.contains("ON"));
}

#[test]
fn test_bool_false_prop() {
    let active = false;
    let component = html! {
        <div>{if active { "ON" } else { "OFF" }}</div>
    };
    let html = test::render(&component);
    assert!(html.contains("OFF"));
}

#[test]
fn test_multiple_props() {
    let name = "Alice";
    let age = 30;
    let active = true;
    let component = html! {
        <div>
            <span>{name}</span>
            <span>" - Age: "{age}</span>
            <span>{if active { " (Active)" } else { " (Inactive)" }}</span>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Alice") && html.contains("30") && html.contains("Active"));
}

#[test]
fn test_float_prop() {
    let price = 19.99;
    let component = html! {
        <span>"Price: $"{price}</span>
    };
    let html = test::render(&component);
    assert!(html.contains("19.99"));
}

#[test]
fn test_char_prop() {
    let grade = 'A';
    let component = html! {
        <span>"Grade: "{grade}</span>
    };
    let html = test::render(&component);
    assert!(html.contains("A"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 2: Optional Props (6 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_option_some() {
    let subtitle: Option<&str> = Some("Subtitle");
    let component = html! {
        <div>
            <h1>"Title"</h1>
            @if let Some(sub) = subtitle {
                <h2>{sub}</h2>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Title") && html.contains("Subtitle"));
}

#[test]
fn test_option_none() {
    let subtitle: Option<&str> = None;
    let component = html! {
        <div>
            <h1>"Title"</h1>
            @if let Some(sub) = subtitle {
                <h2>{sub}</h2>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Title") && !html.contains("Subtitle"));
}

#[test]
fn test_unwrap_or_default() {
    let count: Option<i32> = None;
    let display = count.unwrap_or(0);
    let component = html! {
        <span>"Count: "{display}</span>
    };
    let html = test::render(&component);
    assert!(html.contains("0"));
}

#[test]
fn test_unwrap_or_custom() {
    let count: Option<i32> = Some(42);
    let display = count.unwrap_or(0);
    let component = html! {
        <span>"Count: "{display}</span>
    };
    let html = test::render(&component);
    assert!(html.contains("42"));
}

#[test]
fn test_option_map() {
    let value: Option<i32> = Some(5);
    let doubled = value.map(|x| x * 2).unwrap_or(0);
    let component = html! {
        <span>"Result: "{doubled}</span>
    };
    let html = test::render(&component);
    assert!(html.contains("10"));
}

#[test]
fn test_result_handling() {
    let result: Result<i32, &str> = Ok(100);
    let component = html! {
        <div>
            @match result {
                Ok(val) => { <span>"Success: "{val}</span> }
                Err(e) => { <span>"Error: "{e}</span> }
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Success") && html.contains("100"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 3: Struct Props (8 tests)
// ════════════════════════════════════════════════════════════════════════════

struct User {
    name: String,
    email: String,
}

#[test]
fn test_struct_field_access() {
    let user = User {
        name: "Alice".into(),
        email: "alice@example.com".into(),
    };
    let component = html! {
        <div>
            <h3>{&user.name}</h3>
            <p>{&user.email}</p>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Alice") && html.contains("alice@example.com"));
}

struct Article {
    title: String,
    content: String,
    published: bool,
}

#[test]
fn test_struct_with_bool() {
    let article = Article {
        title: "Hello".into(),
        content: "World".into(),
        published: true,
    };
    let component = html! {
        <article>
            <h2>{&article.title}</h2>
            <p>{&article.content}</p>
            @if article.published {
                <span>"Published"</span>
            } else {
                <span>"Draft"</span>
            }
        </article>
    };
    let html = test::render(&component);
    assert!(html.contains("Hello") && html.contains("World") && html.contains("Published"));
}

#[test]
fn test_struct_unpublished() {
    let article = Article {
        title: "Draft Article".into(),
        content: "Content".into(),
        published: false,
    };
    let component = html! {
        <article>
            <h2>{&article.title}</h2>
            @if article.published {
                <span>"Published"</span>
            } else {
                <span>"Draft"</span>
            }
        </article>
    };
    let html = test::render(&component);
    assert!(html.contains("Draft"));
}

struct Address {
    street: String,
    city: String,
    zip: String,
}

#[test]
fn test_address_struct() {
    let addr = Address {
        street: "123 Main St".into(),
        city: "Springfield".into(),
        zip: "12345".into(),
    };
    let component = html! {
        <div>
            <p>{&addr.street}</p>
            <p>{&addr.city}", "{&addr.zip}</p>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("123 Main St") && html.contains("Springfield") && html.contains("12345"));
}

struct Product {
    name: String,
    price: f64,
    in_stock: bool,
}

#[test]
fn test_product_in_stock() {
    let product = Product {
        name: "Widget".into(),
        price: 9.99,
        in_stock: true,
    };
    let component = html! {
        <div>
            <h3>{&product.name}</h3>
            <p>"$"{product.price}</p>
            {if product.in_stock { "In Stock" } else { "Out of Stock" }}
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Widget") && html.contains("9.99") && html.contains("In Stock"));
}

#[test]
fn test_product_out_of_stock() {
    let product = Product {
        name: "Gadget".into(),
        price: 29.99,
        in_stock: false,
    };
    let component = html! {
        <div>
            <h3>{&product.name}</h3>
            {if product.in_stock { "In Stock" } else { "Out of Stock" }}
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Gadget") && html.contains("Out of Stock"));
}

struct Order {
    id: u32,
    items: Vec<String>,
}

#[test]
fn test_struct_with_vec() {
    let order = Order {
        id: 12345,
        items: vec!["Apple".into(), "Banana".into(), "Cherry".into()],
    };
    let component = html! {
        <div>
            <h3>"Order #"{order.id}</h3>
            <ul>
                @for item in &order.items {
                    <li>{item}</li>
                }
            </ul>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("12345") && html.contains("Apple") && html.contains("Banana"));
}

#[test]
fn test_empty_vec_in_struct() {
    let order = Order {
        id: 99999,
        items: vec![],
    };
    let component = html! {
        <div>
            <h3>"Order #"{order.id}</h3>
            @if order.items.is_empty() {
                <p>"No items"</p>
            } else {
                <ul>
                    @for item in &order.items {
                        <li>{item}</li>
                    }
                </ul>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("99999") && html.contains("No items"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 4: Component Composition (6 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_wrapper_with_static_children() {
    let component = html! {
        <div>
            <header>"Header"</header>
            <main>"Content"</main>
            <footer>"Footer"</footer>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Header") && html.contains("Content") && html.contains("Footer"));
}

#[test]
fn test_nav_links() {
    let links = vec![("Home", "/"), ("About", "/about"), ("Contact", "/contact")];
    let component = html! {
        <nav>
            @for (label, href) in &links {
                <a href={href}>{label}</a>
            }
        </nav>
    };
    let html = test::render(&component);
    assert!(html.contains("Home") && html.contains("About") && html.contains("Contact"));
}

#[test]
fn test_sidebar_layout() {
    let component = html! {
        <div>
            <aside>"Sidebar"</aside>
            <main>"Main content"</main>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Sidebar") && html.contains("Main content"));
}

#[test]
fn test_card_layout() {
    let title = "Card Title";
    let body = "Card body content";
    let action = "Save";
    let component = html! {
        <div>
            <header>{title}</header>
            <main>{body}</main>
            <footer><button>{action}</button></footer>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Card Title") && html.contains("Card body") && html.contains("Save"));
}

#[test]
fn test_two_column() {
    let component = html! {
        <div>
            <div>"Left column"</div>
            <div>"Right column"</div>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Left column") && html.contains("Right column"));
}

#[test]
fn test_breadcrumb() {
    let crumbs = vec!["Home", "Products", "Category", "Item"];
    let component = html! {
        <nav>
            @for (idx, crumb) in crumbs.iter().enumerate() {
                @if idx > 0 {
                    <span>" > "</span>
                }
                <span>{crumb}</span>
            }
        </nav>
    };
    let html = test::render(&component);
    assert!(html.contains("Home") && html.contains("Products") && html.contains("Item"));
}
