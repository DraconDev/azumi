# 🔥 Hot Reload in Azumi

Azumi provides a dual-layer hot reload system to maximize development speed.

1.  **CSS Hot Reload (Built-in)**: Instant updates for styles without reloading.
2.  **Smart Dev Server (Optional)**: Handles HTML updates and auto-restarts the server.

---

## 🚀 1. CSS Hot Reload (Instant)

This is built into the Azumi server. It works automatically with standard `cargo run`.

-   **What it does:** Watches for changes to `<style>` blocks in your Rust code.
-   **Speed:** Instant (< 50ms). No recompile required.
-   **Setup:** Ensure you have added the watcher in your `main.rs`:

```rust
// main.rs
#[tokio::main]
async fn main() {
    // ⚡ Start the subsecond CSS watcher
    azumi::devtools::subsecond_watch();
    
    // ... rest of your app
}
```

---

## ⚡ 2. Smart Dev Server (Quick Mode)

For a complete experience (HTML patching + Auto-restart), use the **Smart Dev Server**. 
Since Azumi doesn't ship a CLI tool yet, you can add this runner to your project easily.

### Setup (One-Time)

1.  Create a file `src/bin/dev.rs` in your project.
2.  Copy the content of the [reference dev runner](https://github.com/DraconDev/azumi/blob/main/demo/src/bin/dev.rs).
3.  Add `reqwest`, `notify`, and `serde_json` to your `Cargo.toml` dev-dependencies.

### Usage

Run the dev server, passing the name of your application binary:

```bash
# Replace 'my-app' with the name of your bin in Cargo.toml
cargo run --bin dev -- my-app
```

-   **HTML Changes**: Patched in sub-second time (no reload).
-   **Logic Changes**: Triggers an automatic server restart.
-   **CSS Changes**: Ignored by this runner (handled instantly by layer 1).

---

## 🐌 3. Classic Mode (Fallback)

If you don't want to set up the dev runner, you can use `cargo-watch`. This is slower (5-15s) as it restarts on every change.

```bash
cargo install cargo-watch
cargo watch -x run
```

---

## 🛠️ Troubleshooting

-   **"WebSocket connection failed"**: Ensure you are running the server and `azumi::hot_reload::router()` is merged in your Axum app.
-   **Styles not updating**: Check console logs. If you see "Style updated" but no change, ensure your CSS selectors are correct.
-   **Slow restarts**: Use a faster linker like `mold` or `lld` to improve iteration times during logic changes.
