use std::fs;
use std::path::Path;

/// Compute a stable hash using FNV-1a algorithm.
/// Unlike DefaultHasher (SipHash), this is deterministic across Rust versions.
fn fnv_hash(data: &str) -> u64 {
    const INITIAL: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 0x100000001b3;
    let mut hash = INITIAL;
    for byte in data.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(PRIME);
    }
    hash
}

fn main() {
    // Only run if client files change
    println!("cargo:rerun-if-changed=client/idiomorph.js");
    println!("cargo:rerun-if-changed=client/azumi.js");
    println!("cargo:rerun-if-changed=AI_RULES_HASH");

    let client_dir = Path::new("client");
    let src_dir = Path::new("src");

    // Read files - graceful handling if files are missing
    let idiomorph = match fs::read_to_string(client_dir.join("idiomorph.js")) {
        Ok(content) => content,
        Err(e) => {
            eprintln!(
                "warning: Failed to read client/idiomorph.js: {}. Using empty content.",
                e
            );
            String::new()
        }
    };
    let azumi = match fs::read_to_string(client_dir.join("azumi.js")) {
        Ok(content) => content,
        Err(e) => {
            eprintln!(
                "warning: Failed to read client/azumi.js: {}. Using empty content.",
                e
            );
            String::new()
        }
    };

    // Concatenate
    // In a real scenario, we might want to minify here
    let combined = format!("{}\n\n{}", idiomorph, azumi);

    // Write to src/client.min.js so it can be included with include_str!
    if let Err(e) = fs::write(src_dir.join("client.min.js"), combined) {
        eprintln!("warning: Failed to write src/client.min.js: {}", e);
    }

    // ── AI Framework Fingerprint ──────────────────────────────────────────
    // Compute a deterministic hash from the version + strict rules.
    // AI assistants read AZUMI_AI_HASH to verify they're targeting the
    // correct framework version and rule set.
    //
    // If the env var AZUMI_AI_HASH is already set (e.g. in CI or .env),
    // use that value directly so teams can pin to a known-good hash.
    //
    // Otherwise, compute from the canonical rules list.
    let ai_hash = std::env::var("AZUMI_AI_HASH").unwrap_or_else(|_| compute_ai_hash());

    println!("cargo:rustc-env=AZUMI_AI_HASH={}", ai_hash);
}

/// Compute a hash from the framework version and strict AI rules.
/// This hash changes whenever the rules change, giving AI assistants
/// a way to verify they're generating code for the correct rule set.
fn compute_ai_hash() -> String {
    let version = std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".into());

    let rules: &[&str] = &[
        // Version is read from Cargo.toml at build time
        "version=dynamic",
        // Syntax rules
        "text_must_be_quoted=true",
        "css_values_must_be_quoted=true",
        "class_must_be_snake_case=true",
        "static_class_attr_banned=true",
        "static_style_attr_banned=true",
        "static_id_attr_banned=true",
        "dashes_in_css_banned=true",
        // Macro rules
        "style_block_after_html=true",
        "let_class_anti_pattern=true",
        "on_event_syntax=call",
        "component_builder_pattern=true",
        // Security rules
        "hmac_signed_state=true",
        "xss_escaping_seo=true",
        "secret_env_var=AZUMI_SECRET",
    ];

    let mut combined = version.to_string();
    for rule in rules {
        combined.push_str(rule);
    }
    format!("{:x}", fnv_hash(&combined))
}
