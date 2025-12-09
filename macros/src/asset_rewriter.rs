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
