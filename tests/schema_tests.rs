#![cfg(feature = "schema")]

use azumi::Schema;
use serde::Serialize;

#[derive(Schema, Serialize)]
struct Product {
    name: String,
    description: String,
    sku: String,
    offers: Offer,
}

#[derive(Schema, Serialize)]
struct Offer {
    price: f32,
    price_currency: String,
}

#[test]
fn test_basic_product_schema() {
    let product = Product {
        name: "Example Product".to_string(),
        description: "An example description.".to_string(),
        sku: "SKU-123".to_string(),
        offers: Offer {
            price: 29.99,
            price_currency: "USD".to_string(),
        },
    };

    let script = product.to_schema_script();
    println!("{}", script);

    assert!(script.contains("<script type=\"application/ld+json\">"));
    assert!(script.contains("\"@context\": \"https://schema.org\""));
    assert!(script.contains("\"@type\": \"Product\""));
    assert!(script.contains("\"name\": \"Example Product\""));
    assert!(script.contains("\"priceCurrency\": \"USD\""));
    assert!(script.contains("\"offers\": {"));
    assert!(script.contains("\"@type\": \"Offer\""));
}

#[derive(Schema, Serialize)]
#[schema(type = "BlogPosting")]
struct Article {
    headline: String,
    #[schema(name = "datePublished")]
    published_at: String,
    #[schema(skip)]
    internal_id: i32,
}

#[test]
fn test_attributes_and_overrides() {
    let article = Article {
        headline: "My First Post".to_string(),
        published_at: "2023-01-01".to_string(),
        internal_id: 999,
    };

    let json = article.to_schema_json_value();

    assert_eq!(json["@type"], "BlogPosting");
    assert_eq!(json["headline"], "My First Post");
    assert_eq!(json["datePublished"], "2023-01-01");
    assert!(json.get("internal_id").is_none());
    assert!(json.get("published_at").is_none());
}

#[derive(Schema, Serialize)]
struct Collection {
    items: Vec<String>,
    meta: Option<String>,
    empty_meta: Option<String>,
}

#[test]
fn test_vec_and_option() {
    let collection = Collection {
        items: vec!["Item 1".to_string(), "Item 2".to_string()],
        meta: Some("Metadata".to_string()),
        empty_meta: None,
    };

    let json = collection.to_schema_json_value();

    assert!(json["items"].is_array());
    assert_eq!(json["items"][0], "Item 1");
    assert_eq!(json["items"][1], "Item 2");
    assert_eq!(json["meta"], "Metadata");
    assert!(json.get("emptyMeta").is_none()); // Should be skipped
}

#[derive(Schema, Serialize)]
struct NestedVec {
    products: Vec<Product>,
}

#[test]
fn test_nested_vec_schema() {
    let container = NestedVec {
        products: vec![Product {
            name: "P1".to_string(),
            description: "D1".to_string(),
            sku: "S1".to_string(),
            offers: Offer {
                price: 10.0,
                price_currency: "USD".to_string(),
            },
        }],
    };

    let json = container.to_schema_json_value();
    let products = json["products"].as_array().unwrap();

    assert_eq!(products.len(), 1);
    assert_eq!(products[0]["@type"], "Product");
    assert_eq!(products[0]["name"], "P1");
}
