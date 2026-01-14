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
    assert!(html.contains("property=\"og:url\" content=\"https://ex.com\""));
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

// ════════════════════════════════════════════════════════════════════════════
// Layout Interactions
// ════════════════════════════════════════════════════════════════════════════

#[azumi::component]
fn SimpleLayout(children: impl azumi::Component) -> impl azumi::Component {
    html! {
        <div class={"layout-root"}>
            <header>"Header"</header>
            <main>{children}</main>
        </div>
    }
}

/// Simple Page using Layout
#[azumi::page]
fn simple_layout_page() -> impl azumi::Component {
    html! {
        @SimpleLayout {
            "Content"
        }
    }
}

#[test]
fn test_layout_rendering() {
    init_test_seo();
    let comp = simple_layout_page();
    let html = test::render(&comp);
    println!("DEBUG SIMPLE LAYOUT: {}", html);
    assert!(html.contains("layout-root"));
    assert!(html.contains("Header"));
}

#[test]
fn test_layout_seo_propagation() {
    init_test_seo();
    let _view = simple_layout_page();
    let head = seo::render_automatic_seo();
    let html = test::render(&head);
    println!("DEBUG PROPAGATION SEO: {}", html);
    assert!(html.contains("<title>Simple Layout Page"));
}
