#[cfg(feature = "schema")]
use azumi::Schema;
use azumi::{html, seo, test};

// ════════════════════════════════════════════════════════════════════════════
// SEO Tests Matrix
// ════════════════════════════════════════════════════════════════════════════

#[azumi::page]
fn seo_page_simple() -> impl azumi::Component {
    html! { <h1>"Simple"</h1> }
}

#[test]
fn test_seo_inference_simple() {
    let _view = seo_page_simple();
    let head = seo::render_automatic_seo();
    let html = test::render(&head);
    assert!(html.contains("<title>Seo Page Simple"), "Got: {}", html);
}

/// My Page Description
#[azumi::page]
fn seo_page_with_desc() -> impl azumi::Component {
    html! { <h1>"Desc"</h1> }
}

#[test]
fn test_seo_inference_desc() {
    let _view = seo_page_with_desc();
    let head = seo::render_automatic_seo();
    let html = test::render(&head);
    assert!(html.contains("content=\"My Page Description\""));
}

#[test]
fn test_manual_head_macro() {
    let head = azumi::head! {
        title: "Manual Title",
        description: "Manual Desc",
        image: "/img.jpg",
        url: "https://ex.com",
        type: "website"
    };
    let html = test::render(&head);
    assert!(html.contains("<title>Manual Title</title>"));
    assert!(html.contains("content=\"Manual Desc\""));
    assert!(html.contains("content=\"/img.jpg\""));
    assert!(html.contains("content=\"https://ex.com\""));
    assert!(html.contains("content=\"website\""));
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
    assert!(script.contains("\"@type\":\"BlogPosting\""));
    assert!(script.contains("\"headline\":\"News\""));
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
    assert!(script.contains("\"@type\":\"Product\""));
    assert!(script.contains("\"sku\":\"G1\""));
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
    assert!(script.contains("\"@type\":\"Organization\""));
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
    assert!(script.contains("\"@type\":\"Person\""));
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
    let comp = nested_page();
    let html = test::render(&comp);
    assert!(html.contains("<title>Nested Page"));
    assert!(html.contains("content=\"Nested SEO Page\""));
}
