#[cfg(feature = "schema")]
use azumi::Schema;
use azumi::{html, seo, test};

// Initialize SEO once for all tests in this file
fn init_test_seo() {
    let config = seo::SeoConfig::new("Test Site")
        .with_description("Test Description")
        .with_image("/default-og.jpg");
    seo::init_seo(config);
}

// ════════════════════════════════════════════════════════════════════════════
// SEO Tests Matrix
// ════════════════════════════════════════════════════════════════════════════

#[azumi::page]
fn seo_page_simple() -> impl azumi::Component {
    html! { <h1>"Simple"</h1> }
}

#[test]
fn test_seo_inference_simple() {
    init_test_seo();
    let _view = seo_page_simple();
    let head = seo::render_automatic_seo();
    let html = test::render(&head);
    println!("DEBUG SEO SIMPLE: {}", html);
    assert!(html.contains("<title>Seo Page Simple"), "Got: {}", html);
}

/// My Page Description
#[azumi::page]
fn seo_page_with_desc() -> impl azumi::Component {
    html! { <h1>"Desc"</h1> }
}

#[test]
fn test_seo_inference_desc() {
    init_test_seo();
    let _view = seo_page_with_desc();
    let head = seo::render_automatic_seo();
    let html = test::render(&head);
    println!("DEBUG SEO DESC: {}", html);
    assert!(html.contains("content=\"My Page Description\""));
}

#[test]
fn test_manual_head_macro() {
    init_test_seo();
    let head = azumi::head! {
        title: "Manual Title",
        description: "Manual Desc",
        image: "/img.jpg",
        url: "https://ex.com",
        type: "website"
    };
    let html = test::render(&head);
    println!("DEBUG HEAD MACRO: {}", html);
    assert!(html.contains("<title>Manual Title"));
    assert!(html.contains("content=\"Manual Desc\""));
    // Note: OpenGraph tags might use 'property' instead of 'name'
    assert!(html.contains("/img.jpg"));
    assert!(html.contains("https://ex.com"));
}

// ════════════════════════════════════════════════════════════════════════════
// Schema.org Matrix
// ════════════════════════════════════════════════════════════════════════════

#[cfg(feature = "schema")]
#[derive(Schema)]
#[schema(type = "BlogPosting")]
struct Post {
    headline: String,
    date_published: String,
}

#[cfg(feature = "schema")]
#[test]
fn test_schema_blog_posting() {
    let post = Post {
        headline: "News".into(),
        date_published: "2024-01-01".into(),
    };
    let script = post.to_schema_script();
    println!("DEBUG SCHEMA BLOG: {}", script);
    assert!(script.contains("BlogPosting"));
    assert!(script.contains("News"));
}

#[cfg(feature = "schema")]
#[derive(Schema)]
#[schema(type = "Product")]
struct Product {
    name: String,
    sku: String,
    price: f64,
}

#[cfg(feature = "schema")]
#[test]
fn test_schema_product() {
    let p = Product {
        name: "Gear".into(),
        sku: "G1".into(),
        price: 99.0,
    };
    let script = p.to_schema_script();
    println!("DEBUG SCHEMA PRODUCT: {}", script);
    assert!(script.contains("Product"));
    assert!(script.contains("G1"));
}

#[cfg(feature = "schema")]
#[derive(Schema)]
#[schema(type = "Organization")]
struct Org {
    name: String,
    url: String,
}

#[cfg(feature = "schema")]
#[test]
fn test_schema_org() {
    let o = Org {
        name: "Acme".into(),
        url: "https://acme.org".into(),
    };
    let script = o.to_schema_script();
    println!("DEBUG SCHEMA ORG: {}", script);
    assert!(script.contains("Organization"));
}

#[cfg(feature = "schema")]
#[derive(Schema)]
struct Person {
    name: String,
    job_title: String,
}

#[cfg(feature = "schema")]
#[test]
fn test_schema_person() {
    let p = Person {
        name: "Bob".into(),
        job_title: "Dev".into(),
    };
    let script = p.to_schema_script();
    println!("DEBUG SCHEMA PERSON: {}", script);
    assert!(script.contains("Person"));
}

// ════════════════════════════════════════════════════════════════════════════
// Layout Interactions
// ════════════════════════════════════════════════════════════════════════════

#[azumi::component]
fn SeoLayout(children: impl azumi::Component) -> impl azumi::Component {
    html! {
        <html>
            <head>
                {seo::render_automatic_seo()}
            </head>
            <body>{children}</body>
        </html>
    }
}

/// Nested SEO Page
#[azumi::page]
fn nested_page() -> impl azumi::Component {
    html! {
        @SeoLayout {
            "Content"
        }
    }
}

#[test]
fn test_layout_seo_propagation() {
    init_test_seo();
    let comp = nested_page();
    let html = test::render(&comp);
    println!("DEBUG LAYOUT SEO: {}", html);
    assert!(html.contains("<title>Nested Page"));
}
