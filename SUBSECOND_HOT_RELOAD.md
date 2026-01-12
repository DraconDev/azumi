# ⚡ Sub-second Template Hot Reloading

Azumi now supports **Sub-second Hot Reloading**, allowing you to change your HTML structure and CSS classes without restarting the server or waiting for a full Rust recompilation.

## 🚀 How It Works

1.  **Macro Instrumentation**: When you compile your app in debug mode, the `html!` macro analyzes your template. If it's a 'simple' template (linear structure), it generates a hook that checks a runtime registry.
2.  **Runtime Registry**: The Azumi server maintains a global map of template IDs to `RuntimeTemplate` objects.
3.  **Update Endpoint**: The server exposes a `POST /_azumi/update_template` endpoint that accepts new template structures.
4.  **Instant Patching**: When the app renders, it checks if a hot-reloaded version exists for the current file/line. If so, it uses the new structure while injecting the existing dynamic variables.

## 🛠️ Setting Up the Watcher

To use this feature, you need an external watcher tool (e.g., written in Python, Node, or Rust) that monitors your source files.

### 1. Watcher Responsibilities
The watcher should:
- Watch `src/**/*.rs` files for changes.
- Parse the files to find `html!` macro blocks.
- Extract the 'static parts' (the HTML strings between dynamic holes `{ ... }`).
- Generate a Template ID matching the one used by the macro: `concat!(file!(), ':', line!(), ':', column!())`.
- Send a JSON payload to the running Azumi server.

### 2. Payload Format
```json
{
  'id': 'src/pages/home.rs:42:10',
  'parts': [
    '<div class="my_new_class">',
    '<span>',
    '</span></div>'
  ]
}
```
*The `parts` array should have +1$ elements for $ dynamic expressions.*

## ⚠️ Current Limitations

- **Structural Complexity**: Currently, hot reload only works for 'simple' templates. Templates using complex control flow (`@if`, `@for`, `@match`) will fall back to standard recompilation reload.
- **Expression Stability**: You cannot add or remove dynamic expressions (`{variable}`) or change their order without a full recompile. The watcher can only change the *static HTML* around them.
- **Debug Mode Only**: This feature is automatically disabled in `--release` builds for maximum performance.

## 💡 Example Workflow

1.  Start your Azumi server: `cargo run`.
2.  Start your watcher tool.
3.  Modify an HTML tag in your `.rs` file (e.g., change `<div>` to `<section>`).
4.  The watcher sends the update.
5.  The browser refreshes instantly with the new structure, bypassing the 5-10s Rust compile time.
