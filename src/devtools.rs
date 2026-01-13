use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    Router,
};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::{Child, Command};
use std::time::{Duration, Instant};

/// Returns the router for Azumi development tools
/// currently includes the hot reload websocket endpoint
pub fn router() -> Router {
    crate::hot_reload::router()
}

/// The "Easiest Solution" for developers.
///
/// Call this at the very beginning of your `main()` function.
/// It automatically manages sub-second patching and server restarts
/// during development (debug mode).
pub fn auto_reload() {
    auto_reload_if(cfg!(debug_assertions));
}

/// Start hot-reload only if the provided condition is true.
///
/// # Usage
/// ```rust,no_run
/// fn main() {
///     let is_dev = true; // or your own config check
///     azumi::devtools::auto_reload_if(is_dev);
///     // ...
/// }
/// ```
pub fn auto_reload_if(enabled: bool) {
    if !enabled {
        return;
    }

    // If we are already the worker, just start the internal CSS watcher and return to main()
    if std::env::var("AZUMI_IS_WORKER").is_ok() {
        subsecond_watch();
        return;
    }

    // If we aren't in a terminal or something went wrong, don't trap the user
    // unless they explicitly forced it.
    if !std::io::stdin().is_terminal() && std::env::var("AZUMI_FORCE_WATCH").is_err() {
        return;
    }

    println!("🔥 Azumi Smart Watcher Active");
    run_master_loop();
    std::process::exit(0);
}

trait IsTerminal {
    fn is_terminal(&self) -> bool;
}

impl IsTerminal for std::io::Stdin {
    fn is_terminal(&self) -> bool {
        #[cfg(unix)]
        {
            use std::os::fd::AsRawFd;
            unsafe { libc::isatty(self.as_raw_fd()) != 0 }
        }
        #[cfg(not(unix))]
        {
            false
        }
    }
}

fn run_master_loop() {
    use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
    use std::sync::mpsc::channel;

    // Detect the binary name to ensure we restart the correct target
    let exe = std::env::current_exe().expect("Failed to get current exe path");
    let bin_name = exe.file_name().and_then(|s| s.to_str()).unwrap_or("app");

    let mut server = start_worker(bin_name);
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();

    // Watch common Rust directories
    for dir in ["src", "demo/src", "apps", "libs"] {
        if Path::new(dir).exists() {
            let _ = watcher.watch(Path::new(dir), RecursiveMode::Recursive).unwrap();
        }
    }

    let mut last_run = Instant::now();
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    loop {
        match rx.recv() {
            Ok(Ok(event)) => {
                if last_run.elapsed() < Duration::from_millis(200) {
                    continue;
                }
                last_run = Instant::now();

                let is_rs = event
                    .paths
                    .iter()
                    .any(|p| p.extension().map_or(false, |e| e == "rs"));
                if !is_rs {
                    continue;
                }

                if let Some(path) = event.paths.first() {
                    if let Ok(true) = try_hot_patch_internal(path, &port) {
                        println!("⚡ Sub-second patch sent!");
                        continue;
                    }
                }

                println!("🔄 Logic change detected. Rebuilding & Restarting...");
                let _ = server.kill();
                let _ = server.wait();
                server = start_worker(bin_name);
            }
            _ => {}
        }
    }
}

fn start_worker(bin_name: &str) -> Child {
    let mut cmd = Command::new("cargo");
    // Use --bin to ensure we run the same target even in workspaces
    cmd.args(&["run", "--bin", bin_name, "--"]);
    
    // Forward original CLI arguments to the worker
    let args: Vec<String> = std::env::args().skip(1).collect();
    cmd.args(&args);

    cmd.env("AZUMI_IS_WORKER", "1")
        .spawn()
        .expect("Failed to start azumi worker")
}

fn try_hot_patch_internal(path: &Path, port: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let templates = extract_templates_internal(&content, path.to_string_lossy().as_ref());

    if templates.is_empty() {
        return Ok(false);
    }

    let mut success = false;
    for (id, parts) in templates {
        let payload = serde_json::json!({ "id": id, "parts": parts }).to_string();
        if send_raw_post(port, "/_azumi/update_template", &payload) {
            success = true;
        } else {
            return Ok(false);
        }
    }
    Ok(success)
}

fn send_raw_post(port: &str, path: &str, body: &str) -> bool {
    use std::io::Write;
    use std::net::TcpStream;

    let addr = format!("127.0.0.1:{}", port);
    if let Ok(mut stream) = TcpStream::connect_timeout(
        &addr.parse().unwrap(),
        Duration::from_millis(100),
    ) {
        let request = format!(
            "POST {} HTTP/1.1\r\n\
             Host: localhost:{}\r\n\
             Content-Type: application/json\r\n\
             Content-Length: {}\r\n\
             Connection: close\r\n\r\n\
             {}",
            path,
            port,
            body.len(),
            body
        );
        return stream.write_all(request.as_bytes()).is_ok();
    }
    false
}

fn extract_templates_internal(content: &str, file_path: &str) -> HashMap<String, Vec<String>> {
    let mut templates = HashMap::new();
    let mut current_idx = 0;
    while let Some(idx) = content[current_idx..].find("html!") {
        let start = current_idx + idx;
        let open_brace = match content[start..].find('{') {
            Some(i) => start + i,
            None => {
                current_idx = start + 5;
                continue;
            }
        };

        let pre = &content[..start];
        let line = pre.lines().count();
        let line = if pre.ends_with('\n') {
            line + 1
        } else {
            std::cmp::max(1, line)
        };

        let last_line = pre.lines().last().unwrap_or("");
        let col = last_line.len() + 1;

        let mut depth = 1;
        let mut inner_end = 0;
        let mut chars = content[open_brace + 1..].char_indices();

        while let Some((i, c)) = chars.next() {
            if c == '{' {
                depth += 1;
            } else if c == '}' {
                depth -= 1;
            }

            if depth == 0 {
                inner_end = open_brace + 1 + i;
                break;
            }
        }

        if depth == 0 {
            let body = &content[open_brace + 1..inner_end];
            let mut parts = Vec::new();
            let mut last = 0;
            let mut d = 0;

            for (i, c) in body.char_indices() {
                if c == '{' {
                    if d == 0 {
                        parts.push(body[last..i].to_string());
                    }
                    d += 1;
                } else if c == '}' {
                    d -= 1;
                    if d == 0 {
                        last = i + 1;
                    }
                }
            }
            parts.push(body[last..].to_string());

            let id = format!("{}:{}:{}", file_path, line, col);
            templates.insert(id, parts);
            current_idx = inner_end;
        } else {
            break;
        }
    }
    templates
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
            let first_node_rel = content[macro_content_start..]
                .find(|c: char| !c.is_whitespace())
                .unwrap_or(0);
            let first_node_abs = macro_content_start + first_node_rel;

            // Find <style> and </style>
            if let Some(style_start) = content[macro_content_start..].find("<style") {
                let style_tag_rel_end = content[macro_content_start + style_start..]
                    .find(">")
                    .unwrap_or(0);
                let css_start = macro_content_start + style_start + style_tag_rel_end + 1;

                if let Some(style_end) = content[css_start..].find("</style>") {
                    let css_content = &content[css_start..css_start + style_end];

                    // Line (1-indexed)
                    let line = content[..first_node_abs].lines().count();
                    // Column (0-indexed in proc-macro2)
                    let line_start = content[..first_node_abs]
                        .rfind('\n')
                        .map(|i| i + 1)
                        .unwrap_or(0);
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
