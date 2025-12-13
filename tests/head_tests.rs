// use azumi::head;

// #[test]
// fn test_minimal_head() {
//     let meta = head! {
//         title: "Minimal Page",
//         description: "Just a title and description"
//     };

//     assert!(meta.contains("<title>Minimal Page</title>"));
//     assert!(meta.contains("<meta name=\"description\" content=\"Just a title and description\">"));
//     assert!(meta.contains("<meta property=\"og:title\" content=\"Minimal Page\">"));
//     assert!(meta.contains("<meta name=\"twitter:card\" content=\"summary\">"));
// }

// #[test]
// fn test_full_head() {
//     let meta = head! {
//         title: "Full Page",
//         description: "Everything included",
//         image: "/static/preview.jpg",
//         url: "https://example.com",
//         type: "article"
//     };

//     assert!(meta.contains("<meta property=\"og:image\" content=\"/static/preview.jpg\">"));
//     assert!(meta.contains("<meta name=\"twitter:image\" content=\"/static/preview.jpg\">"));
//     assert!(meta.contains("<meta name=\"twitter:card\" content=\"summary_large_image\">"));
//     assert!(meta.contains("<meta property=\"og:url\" content=\"https://example.com\">"));
//     assert!(meta.contains("<meta property=\"og:type\" content=\"article\">"));
// }

// #[test]
// fn test_dynamic_values() {
//     let page_title = "Dynamic Title";
//     let meta = head! {
//         title: page_title,
//         description: format!("Description for {}", page_title)
//     };

//     assert!(meta.contains("<title>Dynamic Title</title>"));
//     assert!(meta.contains("Description for Dynamic Title"));
// }
