use axum::{extract::Request, middleware::Next, response::Response, Router};
use std::path::Path;

/// Returns the router for Azumi development tools
/// currently includes the hot reload websocket endpoint
pub fn router() -> Router {
    crate::hot_reload::router()
}

/// Starts a background thread that watches for CSS changes in .rs files
/// and pushes updates to the browser without a full reload.
pub fn subsecond_watch() {
    #[cfg(debug_assertions)]
    {
        std::thread::spawn(|| {
            if let Err(e) = watch_loop() {
                eprintln!("🔥 Azumi Watcher Error: {:?}", e);
            }
        });
    }
}

fn watch_loop() -> Result<(), Box<dyn std::error::Error>> {
    use notify::{RecursiveMode, Watcher};
    use std::sync::mpsc::channel;

    let (tx, rx) = channel();
    let mut watcher = notify::recommended_watcher(tx)?;

    // Watch src and demo/src
    if Path::new("src").exists() {
        watcher.watch(Path::new("src"), RecursiveMode::Recursive)?;
    }
    if Path::new("demo/src").exists() {
        watcher.watch(Path::new("demo/src"), RecursiveMode::Recursive)?;
    }

    println!("🔥 Azumi Subsecond Watcher: Active");

    loop {
        match rx.recv() {
            Ok(Ok(event)) => {
                if event.kind.is_modify() {
                    for path in event.paths {
                        if path.extension().map_or(false, |ext| ext == "rs") {
                            process_file_change(&path);
                        }
                    }
                }
            }
            Ok(Err(e)) => eprintln!("watch error: {:?}", e),
            Err(_) => break,
        }
    }
    Ok(())
}

fn process_file_change(path: &Path) {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return,
    };

    let mut current_pos = 0;
    while let Some(start_idx) = content[current_pos..].find("html!") {
        let absolute_start = current_pos + start_idx;
        if let Some(brace_idx) = content[absolute_start..].find('{') {
            let macro_content_start = absolute_start + brace_idx + 1;
            
            // The macro uses nodes[0].span().start()
            // Find the first non-whitespace character after the opening brace
            let first_node_rel = content[macro_content_start..].find(|c: char| !c.is_whitespace()).unwrap_or(0);
            let first_node_abs = macro_content_start + first_node_rel;

            // Find <style> and </style>
            if let Some(style_start) = content[macro_content_start..].find("<style") {
                let style_tag_rel_end = content[macro_content_start + style_start..].find(">").unwrap_or(0);
                let css_start = macro_content_start + style_start + style_tag_rel_end + 1;
                
                if let Some(style_end) = content[css_start..].find("</style>") {
                    let css_content = &content[css_start..css_start + style_end];
                    
                    // Line (1-indexed)
                    let line = content[..first_node_abs].lines().count();
                    // Column (0-indexed in proc-macro2)
                    let line_start = content[..first_node_abs].rfind('\n').map(|i| i + 1).unwrap_or(0);
                    let col = first_node_abs - line_start;

                    use std::collections::hash_map::DefaultHasher;
                    use std::hash::{Hash, Hasher};
                    let mut hasher = DefaultHasher::new();
                    line.hash(&mut hasher);
                    col.hash(&mut hasher);
                    let hash = hasher.finish();
                    let scope_id = format!("s{:x}", hash);

                    let scoped_css = crate::scope_css(css_content, &scope_id);
                    crate::hot_reload::push_style_update(&scope_id, &scoped_css);
                }
            }
        }
        current_pos = absolute_start + 5;
    }
}

/// Middleware to force no-cache headers in development mode
/// usage: .layer(axum::middleware::from_fn(azumi::devtools::no_cache_middleware))
pub async fn no_cache_middleware(req: Request, next: Next) -> Response {
    let mut response = next.run(req).await;

    // Only set headers if we are in debug mode
    #[cfg(debug_assertions)]
    {
        let headers = response.headers_mut();
        // Prevent caching for all responses
        headers.insert(
            "Cache-Control",
            "no-cache, no-store, must-revalidate".parse().unwrap(),
        );
        headers.insert("Pragma", "no-cache".parse().unwrap());
        headers.insert("Expires", "0".parse().unwrap());
    }

    response
}
