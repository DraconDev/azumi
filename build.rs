use std::fs;
use std::path::Path;

fn main() {
    // Only run if client files change
    println!("cargo:rerun-if-changed=client/idiomorph.js");
    println!("cargo:rerun-if-changed=client/azumi.js");

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
}
