use crate::token_parser::{parse_nodes, Node};
use quote::quote;
use syn::parse::{ParseStream, Parser};
use syn::Result;

fn parse_nodes_wrapper(input: ParseStream) -> Result<Vec<Node>> {
    parse_nodes(input)
}

#[test]
fn test_mismatched_closing_tag() {
    let input = quote! {
        <div></span>
    };
    let parser = parse_nodes_wrapper;
    let result = parser.parse2(input);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Mismatched closing tag: expected </div>, found </span>"
    );
}

#[test]
fn test_component_block_in_div() {
    let input = quote! {
        <div>
            @content
        </div>
    };
    let parser = parse_nodes_wrapper;
    let result = parser.parse2(input);
    assert!(result.is_ok());
}

#[test]
fn test_namespaced_attributes() {
    let input = quote! {
        <button hx-on:click="alert('clicked')" v-bind:class="active">"Click me"</button>
    };
    let parser = parse_nodes_wrapper;
    let result = parser.parse2(input);
    match result {
        Ok(nodes) => {
            if let Node::Element(elem) = &nodes[0] {
                assert_eq!(elem.name, "button");
                assert_eq!(elem.attrs.len(), 2);
                assert_eq!(elem.attrs[0].name, "hx-on:click");
                assert_eq!(elem.attrs[1].name, "v-bind:class");
            } else {
                panic!("Expected Element node");
            }
        }
        Err(e) => panic!("Parse failed: {}", e),
    }
}
