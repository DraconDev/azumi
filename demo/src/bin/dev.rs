use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};
use syn::visit::Visit;

fn main() {
    println!("🚀 Azumi Smart Dev Server");
    println!("   Compiling...");

    let mut server = start_server();
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();
    
    watcher.watch(Path::new("src"), RecursiveMode::Recursive).unwrap();
    println!("👀 Watching for changes in demo/src...");

    let mut last_run = Instant::now();

    loop {
        match rx.recv() {
            Ok(Ok(event)) => {
                if last_run.elapsed() < Duration::from_millis(200) {
                    continue;
                }
                last_run = Instant::now();

                let is_rs = event.paths.iter().any(|p| p.extension().map_or(false, |e| e == "rs"));
                if !is_rs { continue; }

                println!("📝 Change detected.");
                
                let mut hot_patched = false;
                // Attempt hot patch for the first file in event
                if let Some(path) = event.paths.first() {
                    // For MVP: We restart by default unless parse logic is implemented
                    // To enable hot patching, implement parse_simple_template fully
                    if let Ok(true) = try_hot_patch(path) {
                        hot_patched = true;
                    }
                }

                if hot_patched {
                    println!("⚡ Sub-second patch sent!");
                } else {
                    println!("🔄 Logic change detected. Restarting server...");
                    let _ = server.kill();
                    let _ = server.wait();
                    server = start_server();
                }
            }
            _ => {}
        }
    }
}

fn start_server() -> Child {
    Command::new("cargo")
        .args(&["run", "--bin", "azumi-demo"])
        .spawn()
        .expect("Failed to start server")
}

fn try_hot_patch(path: &Path) -> Result<bool, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    // If parse fails, we restart
    let ast = match syn::parse_file(&content) {
        Ok(a) => a,
        Err(_) => return Ok(false),
    };

    let mut visitor = HtmlVisitor::new(content.clone());
    visitor.visit_file(&ast);

    if visitor.found_templates.is_empty() {
        return Ok(false);
    }

    // Send patches
    let mut patched_any = false;
    for (id, parts) in visitor.found_templates {
        let client = reqwest::blocking::Client::new();
        let payload = serde_json::json!({
            "id": id,
            "parts": parts
        });

        match client.post("http://localhost:3000/_azumi/update_template")
            .json(&payload)
            .timeout(Duration::from_millis(100))
            .send() 
        {
            Ok(_) => patched_any = true,
            Err(_) => return Ok(false),
        }
    }

    Ok(patched_any)
}

struct HtmlVisitor {
    source_code: String,
    found_templates: HashMap<String, Vec<String>>,
}

impl HtmlVisitor {
    fn new(source_code: String) -> Self {
        Self { source_code, found_templates: HashMap::new() }
    }
}

impl<'ast> Visit<'ast> for HtmlVisitor {
    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        if i.path.is_ident("html") {
            let start = i.bang_token.span.start();
            // TODO: This ID generation needs to match exactly what macro uses
            // For now, restarting is safer until this aligns perfectly
            let id = format!("{}:{}:{}", "src/bin/dev.rs", start.line, start.column); 
            
            let tokens = &i.tokens;
            let s = tokens.to_string(); 
            
            let (statics, _dynamics) = parse_simple_template(&s);
            if !statics.is_empty() {
                // self.found_templates.insert(id, statics);
            }
        }
        syn::visit::visit_macro(self, i);
    }
}

fn parse_simple_template(input: &str) -> (Vec<String>, usize) {
    let mut statics = Vec::new();
    let mut dynamics = 0;
    (statics, dynamics)
}
