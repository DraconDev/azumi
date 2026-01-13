# 🔥 Hot Reload in Azumi

Azumi provides a built-in, self-hosting hot reload system that requires zero configuration and zero external binaries.

---

## 🚀 The "Gold Standard" (Recommended)

Just add one line to your `main.rs`. Azumi will automatically detect development mode, watch your files, and patch your UI in sub-second time.

### Setup

```rust
// src/main.rs
#[tokio::main]
async fn main() {
    // ⚡ Add this line at the VERY BEGINNING of main()
    azumi::devtools::auto_reload();
    
    // OR tie it to your own condition:
    // azumi::devtools::auto_reload_if(my_config.is_dev);

    // ... your normal Axum setup
}
```

### How it works:
1.  **CSS Changes**: Patched instantly (< 50ms) without page reload.
2.  **HTML Changes**: Patched in sub-second time via WebSocket reload.
3.  **Logic Changes**: Triggers an automatic server restart.

---

## 🐌 Fallback: Classic Mode

If you prefer not to use the built-in watcher, you can use `cargo-watch`. This is slower (5-15s) as it restarts the entire compiler on every change.

```bash
cargo install cargo-watch
cargo watch -x run
```

---

## 🛠️ Requirements & Troubleshooting

-   **Debug Mode**: Hot reload is only active in `debug` builds (not `--release`).
-   **Terminal**: The master watcher only starts when running in an interactive terminal.
-   **WebSocket**: Ensure `azumi::devtools::router()` is merged into your Axum app so the browser can receive signals.
-   **Port**: By default, it expects the server on port `8080`. If you use a different port, set the `PORT` environment variable:
    ```bash
    PORT=3000 cargo run
    ```