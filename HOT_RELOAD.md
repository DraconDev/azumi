# 🔥 Hot Reload in Azumi

Azumi supports two modes of hot reload to maximize your development speed.

## 🚀 1. Quick Mode (Recommended)

Quick mode uses a native Rust runner to watch your files. It can patch HTML structure and CSS classes **instantly** without restarting the server or re-running the Rust compiler.

### Usage
Run the `dev` runner provided in the demo:

```bash
cargo run --bin dev
```

- **HTML Changes**: Patched in sub-second time.
- **Logic Changes**: Triggers an automatic server restart.

---

## 🐌 2. Classic Mode (Fallback)

Classic mode uses `cargo-watch` to restart the server on every change. This is reliable but slower (5-15s per change).

### Prerequisites nice
Install `cargo-watch`:
```bash
cargo install cargo-watch
```

### Usage
```bash
cargo watch -x run
```

---

## 🛠️ How It Works

1.  **Server Side**: The Azumi server exposes a WebSocket endpoint at `/_azumi/live_reload` and a patching endpoint at `/_azumi/update_template`.
2.  **Dev Runner**: The `dev` binary (`demo/src/bin/dev.rs`) watches your source code.
3.  **Patching**: When you save a file, the runner extracts the new `html!` structure and sends it to the running server.
4.  **Browser**: The client script (`azumi.js`) receives a reload signal and refreshes the page.

## Troubleshooting

- **"WebSocket connection failed"**: Ensure you are running the server and `azumi::hot_reload::router()` is merged in your Axum app.
- **Slow restarts**: Use a faster linker like `mold` or `lld` to improve Iteration times during logic changes.