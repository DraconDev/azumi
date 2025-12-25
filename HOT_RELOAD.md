# 🔥 Hot Reload in Azumi

Azumi supports hot reload to improve your development experience. Since Azumi uses Rust macros for compile-time safety, "hot reload" effectively means **automatic server restart and browser refresh**.

## Prerequisites

You need `cargo-watch` to detect file changes and restart the server.

```bash
cargo install cargo-watch
```

## How It Works

1.  **Server Side**: The Azumi server (in development mode) exposes a WebSocket endpoint at `/_azumi/live_reload`.
2.  **Client Side**: Use `cargo watch` to recompiles and restarts the server on file change.
3.  **Browser**: The client script (`azumi.js`) connects to this WebSocket.
    -   When the WebSocket connection is lost, it knows the server is restarting.
    -   It polls the server until it's back online.
    -   Once online, it triggers a full page reload.

## Enabling Hot Reload

### 1. Cargo.toml

Ensure `azumi` is enabled, and your `axum` dependency has the `ws` feature (Azumi handles the internal logic, but you need `tokio` and `axum` setup correctly).

### 2. Server Setup

Mount the hot reload router in your application.

```rust
use axum::Router;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home_handler))
        // 👇 Add this line!
        .merge(azumi::hot_reload::router());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### 3. Run with cargo-watch

Instead of `cargo run`, use:

```bash
cargo watch -x run
```

Or if you have a specific binary:

```bash
cargo watch -x "run --bin my-app"
```

## Troubleshooting

-   **"WebSocket connection failed"**: Ensure you are running the server and the route is mounted.
-   **Page doesn't reload**: Check the browser console. You should see "🔥 Hot Reload: Connected".
-   **Slow restarts**: Rust compilation speed can be a bottleneck. Use a faster linker like `mold` or `lld` to improve iteration times.
