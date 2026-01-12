# ⚡ Quick Template Hot Reloading

Azumi now supports **Quick Hot Reloading**, allowing you to change your HTML structure and CSS classes without waiting for a full Rust recompilation.

## 🚀 Usage

Instead of `cargo run`, use the `dev` binary provided in the demo:

```bash
cargo run --bin dev
```

This runner:
1.  Starts your application.
2.  Watches your source code for changes.
3.  **Automatically restarts** the server when Rust logic changes.
4.  **Hot patches** the running server when HTML templates change (significantly faster than full recompile).

## 🛠️ Architecture

The `dev` runner (`demo/src/bin/dev.rs`) uses:
- `notify` to watch the file system.
- A custom parser to identify `html!` macros.
- `reqwest` to send template updates to the running server.

When a change is detected:
1.  It attempts to parse the file and extract the new HTML template.
2.  If successful (simple structure change), it POSTs the update to `/_azumi/update_template`.
3.  If parsing fails or logic changed, it kills and restarts the server.

## ⚠️ Limitations

- **Complex Macros**: Templates with complex control flow (`@if`, loops) inside the root `html!` macro might not be hot-patchable yet and will trigger a full restart.
- **Dependency**: This runner is currently part of the `demo` crate but can be moved to a standalone CLI in the future.
