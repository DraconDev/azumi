use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Mutex;

// Global cache for the manifest to avoid re-reading file on every macro expansion
static MANIFEST: Lazy<Mutex<Option<HashMap<String, String>>>> =
    Lazy::new(|| Mutex::new(load_manifest()));

fn load_manifest() -> Option<HashMap<String, String>> {
    // The macro runs in the context of the user's crate build.
    // We expect assets_manifest.json to be in the crate root (current dir).
    let path = Path::new("assets_manifest.json");
    if !path.exists() {
        // If manifest doesn't exist (e.g., first build or no assets), return None
        // We might want to warn, but for now we just fail gracefully (no rewriting).
        return None;
    }

    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return None,
    };

    serde_json::from_str(&content).ok()
}

use crate::token_parser::{self, AttributeValue, Block, Node};

pub fn rewrite_nodes(nodes: &mut [Node]) {
    for node in nodes {
        rewrite_node(node);
    }
}

fn rewrite_node(node: &mut Node) {
    match node {
        Node::Element(elem) => {
            // Rewrite attributes
            for attr in &mut elem.attrs {
                if attr.name == "src" || attr.name == "href" || attr.name == "srcset" {
                    if let AttributeValue::Static(val) = &mut attr.value {
                        if let Some(new_path) = rewrite_path(val) {
                            *val = new_path;
                        }
                    }
                }
            }
            // Recurse into children
            rewrite_nodes(&mut elem.children);
        }
        Node::Fragment(frag) => {
            rewrite_nodes(&mut frag.children);
        }
        Node::Block(block) => match block {
            Block::If(if_block) => {
                rewrite_nodes(&mut if_block.then_branch);
                if let Some(else_branch) = &mut if_block.else_branch {
                    rewrite_nodes(else_branch);
                }
            }
            Block::For(for_block) => {
                rewrite_nodes(&mut for_block.body);
            }
            Block::Match(match_block) => {
                for arm in &mut match_block.arms {
                    rewrite_nodes(&mut arm.body);
                }
            }
            Block::Call(call_block) => {
                rewrite_nodes(&mut call_block.children);
            }
            _ => {}
        },
        _ => {}
    }
}

pub fn rewrite_path(original: &str) -> Option<String> {
    // Only attempt rewrite for absolute paths (starting with /)
    if !original.starts_with('/') {
        return None;
    }

    let guard = MANIFEST.lock().unwrap();
    if let Some(map) = &*guard {
        if let Some(hashed) = map.get(original) {
            return Some(hashed.clone());
        }
    }

    None
}
