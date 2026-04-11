/// CSS Validator - Enforces Azumi's CSS rules at compile time
use crate::token_parser::{AttributeValue, Block, Node};

/// Parse all CSS files referenced in the component and validate classes
/// Returns a TokenStream of compile errors if validation fails
pub fn validate_component_css(nodes: &[Node]) -> proc_macro2::TokenStream {
    use quote::quote;

    // First, collect all CSS files referenced in the component
    let mut css_files = Vec::new();
    collect_css_files(nodes, &mut css_files);

    // Check for inline styles (banned unless internal)
    let mut inline_errors = Vec::new();
    check_inline_styles_recursive(nodes, &mut inline_errors);

    if !inline_errors.is_empty() {
        return quote! {
            #(#inline_errors)*
        };
    }

    if !css_files.is_empty() {
        return quote! {
            compile_error!("External CSS files are banned in Azumi. Use the style! macro instead.");
        };
    }

    quote! {}
}

fn check_inline_styles_recursive(nodes: &[Node], errors: &mut Vec<proc_macro2::TokenStream>) {
    use quote::quote;
    for node in nodes {
        match node {
            Node::Element(elem) => {
                if elem.name == "style" {
                    let has_src = elem.attrs.iter().any(|a| a.name == "src");
                    let is_internal = elem.attrs.iter().any(|a| a.name == "data-azumi-scope");

                    if !has_src && !is_internal {
                        errors.push(quote! {
                            compile_error!(r#"Inline <style> tags not allowed in Azumi

CSS must be external:
  ✅ <style src="components/card.css" />  (auto-scoped)
  ❌ <style>.card { padding: 2em; }</style>

For dynamic styles: use style attribute with expressions

Why? External files get full IDE support (linting, autocomplete, error checking)."#);
                        });
                    }
                }
                check_inline_styles_recursive(&elem.children, errors);
            }
            Node::Fragment(frag) => {
                check_inline_styles_recursive(&frag.children, errors);
            }
            Node::Block(block) => match block {
                Block::If(if_block) => {
                    check_inline_styles_recursive(&if_block.then_branch, errors);
                    if let Some(else_branch) = &if_block.else_branch {
                        check_inline_styles_recursive(else_branch, errors);
                    }
                }
                Block::For(for_block) => {
                    check_inline_styles_recursive(&for_block.body, errors);
                }
                Block::Match(match_block) => {
                    for arm in &match_block.arms {
                        check_inline_styles_recursive(&arm.body, errors);
                    }
                }
                Block::Call(call_block) => {
                    check_inline_styles_recursive(&call_block.children, errors);
                }
                _ => {}
            },
            _ => {}
        }
    }
}

/// Collect all CSS file paths from <style src="..."> tags
fn collect_css_files(nodes: &[Node], css_files: &mut Vec<String>) {
    for node in nodes {
        match node {
            Node::Element(elem) => {
                if elem.name.as_str() == "style" {
                    if let Some(src_attr) = elem.attrs.iter().find(|a| a.name == "src") {
                        if let AttributeValue::Static(path) = &src_attr.value {
                            // Skip global.css files - they are opt-out of validation
                            if !path.ends_with("global.css") {
                                let css_file_path = resolve_css_file_path(path);
                                css_files.push(css_file_path);
                            }
                        }
                    }
                }
                collect_css_files(&elem.children, css_files);
            }
            Node::Fragment(frag) => {
                collect_css_files(&frag.children, css_files);
            }
            Node::Block(block) => match block {
                Block::If(if_block) => {
                    collect_css_files(&if_block.then_branch, css_files);
                    if let Some(else_branch) = &if_block.else_branch {
                        collect_css_files(else_branch, css_files);
                    }
                }
                Block::For(for_block) => {
                    collect_css_files(&for_block.body, css_files);
                }
                Block::Match(match_block) => {
                    for arm in &match_block.arms {
                        collect_css_files(&arm.body, css_files);
                    }
                }
                Block::Call(call_block) => {
                    collect_css_files(&call_block.children, css_files);
                }
                _ => {}
            },
            _ => {}
        }
    }
}

/// Resolve CSS file path from CARGO_MANIFEST_DIR
pub fn resolve_css_file_path(css_path: &str) -> String {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let manifest_path = std::path::Path::new(&manifest_dir);
    let clean_path = css_path.trim_start_matches('/');

    let possible_paths = vec![
        manifest_path.join(clean_path).to_string_lossy().to_string(),
        manifest_path
            .join("static")
            .join(clean_path)
            .to_string_lossy()
            .to_string(),
        manifest_path
            .join("src")
            .join(clean_path)
            .to_string_lossy()
            .to_string(),
    ];

    for path in &possible_paths {
        if std::path::Path::new(path).exists() {
            return path.clone();
        }
    }

    manifest_path.join(clean_path).to_string_lossy().to_string()
}
