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
    assert!(html.contains("<title>Manual Title"));
    assert!(html.contains("content=\"Manual Desc\""));
    assert!(html.contains("property=\"og:url\" content=\"https://ex.com\""));
}

#[azumi::component]
fn TraceLayout(children: impl azumi::Component) -> impl azumi::Component {
    html! {
        <div id={"trace-id"}>
            {children}
        </div>
    }
}

#[azumi::page]
fn trace_page() -> impl azumi::Component {
    html! {
        @TraceLayout() {
            "TraceContent"
        }
    }
}

#[test]
fn test_trace_layout() {
    init_test_seo();
    let comp = trace_page();
    let html = test::render(&comp);
    println!("DEBUG TRACE: {}", html);
    assert!(html.contains("trace-id"), "HTML was: {}", html);
}

// Keep Schema tests as they were passing
#[cfg(feature = "schema")]
#[derive(Schema)]
#[schema(type = "BlogPosting")]
struct Post {
    headline: String,
    date_published: String,
}

#[cfg(feature = "schema")]
#[test]
fn test_schema_works() {
    let post = Post {
        headline: "Hi".into(),
        date_published: "2024".into(),
    };
    assert!(post.to_schema_script().contains("BlogPosting"));
}
