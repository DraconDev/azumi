use azumi::head;

#[test]
fn test_minimal_head() {
    let meta = head! {
        title: "Minimal Page",
        description: "Just a title and description"
    };

    assert!(meta.0.contains("<title>Minimal Page</title>"));
    assert!(meta
        .0
        .contains("<meta name=\"description\" content=\"Just a title and description\">"));
    assert!(meta
        .0
        .contains("<meta property=\"og:title\" content=\"Minimal Page\">"));
    // assert!(meta.0.contains("<meta name=\"twitter:card\" content=\"summary\">")); // Twitter card default might confirm this
}

#[test]
fn test_full_head() {
    let meta = head! {
        title: "Full Page",
        description: "Everything included",
        image: "/static/preview.jpg"
        // url: "https://example.com", // macro parser needs to support these new keys if they aren't there
        // type: "article"
    };

    assert!(meta
        .0
        .contains("<meta property=\"og:image\" content=\"/static/preview.jpg\">"));
    assert!(meta
        .0
        .contains("<meta name=\"twitter:image\" content=\"/static/preview.jpg\">"));
    // assert!(meta.0.contains("<meta name=\"twitter:card\" content=\"summary_large_image\">"));
}

#[test]
fn test_dynamic_values() {
    let page_title = "Dynamic Title";
    let meta = head! {
        title: page_title,
        description: format!("Description for {}", page_title)
    };

    assert!(meta.0.contains("<title>Dynamic Title</title>"));
    assert!(meta.0.contains("Description for Dynamic Title"));
}
