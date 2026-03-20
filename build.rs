use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;

fn main() {
    // Only run if client files change
    println!("cargo:rerun-if-changed=client/idiomorph.js");
    println!("cargo:rerun-if-changed=client/azumi.js");
    println!("cargo:rerun-if-changed=AI_RULES_HASH");

    let client_dir = Path::new("client");
    let src_dir = Path::new("src");

    // Read files
    let idiomorph =
        fs::read_to_string(client_dir.join("idiomorph.js")).expect("Failed to read idiomorph.js");
    let azumi = fs::read_to_string(client_dir.join("azumi.js")).expect("Failed to read azumi.js");

    // Concatenate
    // In a real scenario, we might want to minify here
    let combined = format!("{}\n\n{}", idiomorph, azumi);

    // Write to src/client.min.js so it can be included with include_str!
    fs::write(src_dir.join("client.min.js"), combined).expect("Failed to write client.min.js");

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

    let mut hasher = DefaultHasher::new();
    version.hash(&mut hasher);
    for rule in rules {
        rule.hash(&mut hasher);
    }
    format!("{:x}", hasher.finish())
}
