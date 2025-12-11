use crate::token_parser::Element;
use crate::token_parser::Node;
use proc_macro2::TokenStream;
use quote::quote_spanned;

/// Rule 10: Component Structure Enforcement
/// Enforces Script -> Content -> Style order
pub fn validate_node_order(nodes: &[Node]) -> Vec<TokenStream> {
    let mut errors = vec![];
    
    #[derive(PartialEq, PartialOrd, Copy, Clone)]
    enum Phase {
        Script,
        Content,
        Style,
    }

    let mut phase = Phase::Script;

    for node in nodes {
        match node {
            // Script Handling
            Node::Element(elem) if elem.name == "script" => {
                if phase > Phase::Script {
                    let msg = "Order Error: <script> tags must be placed at the top of the component, before any HTML content.";
                    errors.push(quote_spanned! { elem.span =>
                        compile_error!(#msg);
                    });
                }
                // Allowed in Script Phase, stays in Script Phase
            }
            // Style Handling
            Node::Element(elem) if elem.name == "style" => {
                // <style> element (e.g., inline or src) - treat as Style phase
                 phase = Phase::Style;
            }
             Node::Block(crate::token_parser::Block::Style(_)) => {
                // style! block - treat as Style phase
                phase = Phase::Style;
            }
            // Comments - Ignored, do not change phase
            Node::Comment(_) => {}
            // All other content (HTML elements, text, blocks, etc.)
            _ => {
                if phase == Phase::Style {
                    // Start of content span is tricky without a specific node match, 
                    // but we can match individual types if needed or just use a generic span if available.
                    // For now, let's try to get a span from the node if possible
                    let span = match node {
                        Node::Element(e) => e.span,
                        Node::Text(t) => t.span,
                        Node::Expression(e) => e.span,
                        Node::Doctype(d) => d.span,
                        Node::Fragment(f) => f.span,
                        Node::Block(b) => match b {
                            crate::token_parser::Block::If(i) => i.span,
                            crate::token_parser::Block::For(f) => f.span,
                            crate::token_parser::Block::Match(m) => m.span,
                             crate::token_parser::Block::Call(c) => c.span,
                            crate::token_parser::Block::Component(c) => c.span,
                            crate::token_parser::Block::Let(l) => l.span,
                            _ => proc_macro2::Span::call_site(), // Should match matches above
                        },
                        _ => proc_macro2::Span::call_site(),
                    };

                    let msg = "Order Error: HTML structure and logic must be placed BEFORE Style blocks. Move this content above the <style> block.";
                    errors.push(quote_spanned! { span =>
                        compile_error!(#msg);
                    });
                } else {
                    // Move to Content phase if we were in Script phase
                    if phase == Phase::Script {
                        phase = Phase::Content;
                    }
                }
            }
        }
    }

    errors
}

/// Rule 1: Tables can only contain specific children
pub fn validate_table_children(elem: &Element) -> Vec<TokenStream> {
