# Azumi + Tauri Integration Guide

> A comprehensive technical document exploring how to wrap an Azumi web application in Tauri for native desktop distribution.

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Architectural Overview](#architectural-overview)
3. [How It Works](#how-it-works)
4. [Implementation Strategy](#implementation-strategy)
5. [Known Difficulties & Solutions](#known-difficulties--solutions)
6. [Comparison with Alternatives](#comparison-with-alternatives)
7. [Proof of Concept Outline](#proof-of-concept-outline)
8. [Open Questions](#open-questions)

---

## Executive Summary

**Goal**: Package an Azumi SSR application as a native desktop app using Tauri.

**Core Idea**: Run the Azumi Axum server as a **Tauri sidecar process**, and point the Tauri WebView to `http://localhost:<PORT>`. The app feels native, but internally uses the same server-rendered architecture as the web version.

**Why This Makes Sense**:

-   Single Rust codebase for web and desktop
-   Lighter than Electron (~10MB vs ~150MB+)
-   No need for WASM hydration complexity (unlike Leptos/Dioxus)
-   Direct access to SQLite, filesystem, and OS APIs
-   Works offline by default

---

## Architectural Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                        TAURI APPLICATION                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                    WEBVIEW (System)                       │  │
│  │                                                           │  │
│  │   ┌─────────────────────────────────────────────────────┐ │  │
│  │   │  http://localhost:8080                              │ │  │
│  │   │                                                     │ │  │
│  │   │  ┌─────────────────┐    ┌────────────────────────┐  │ │  │
│  │   │  │ Azumi HTML Page │◄───│ azumi.js (12kb runtime)│  │ │  │
│  │   │  └─────────────────┘    └────────────────────────┘  │ │  │
│  │   │            │                        │               │ │  │
│  │   │            │  Optimistic Updates    │               │ │  │
│  │   │            ▼                        │               │ │  │
│  │   │     User clicks button ──────────►  POST /action    │ │  │
│  │   └─────────────────────────────────────────────────────┘ │  │
│  └───────────────────────────────────────────────────────────┘  │
│                              │                                  │
│                              │ HTTP (localhost)                 │
│                              ▼                                  │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                  SIDECAR PROCESS                          │  │
│  │                                                           │  │
│  │   ┌─────────────────────────────────────────────────────┐ │  │
│  │   │            Azumi Axum Server                        │ │  │
│  │   │                                                     │ │  │
│  │   │   • Renders HTML pages                              │ │  │
│  │   │   • Handles optimistic actions                      │ │  │
│  │   │   • Serves static assets                            │ │  │
│  │   │   • Connects to SQLite (local file)                 │ │  │
│  │   └─────────────────────────────────────────────────────┘ │  │
│  └───────────────────────────────────────────────────────────┘  │
│                              │                                  │
│                              ▼                                  │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                  LOCAL RESOURCES                          │  │
│  │                                                           │  │
│  │   📁 ~/AppData/azumi-app/data.db                          │  │
│  │   📁 ~/Documents/exports/                                 │  │
│  │   🔐 OS Keychain (via Tauri plugin)                       │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

### Key Components

| Component                  | Role                                                  |
| -------------------------- | ----------------------------------------------------- |
| **Tauri Shell**            | Native window, menu bar, system tray, OS integration  |
| **WebView**                | Renders Azumi HTML, runs `azumi.js` for interactivity |
| **Sidecar (Azumi Server)** | Axum-based HTTP server handling SSR and actions       |
| **Local DB**               | SQLite file in app data directory                     |

---

## How It Works

### Request Flow

1. **App Launch**:

    - Tauri starts the sidecar (Azumi Axum binary)
    - Sidecar binds to `127.0.0.1:8080`
    - Tauri opens WebView pointing to `http://localhost:8080`

2. **Page Load**:

    - WebView requests `/`
    - Sidecar renders HTML using Azumi macros
    - Response includes `<script src="azumi.js" />` for interactivity

3. **User Interaction** (e.g., clicking a "like" button):

    - `azumi.js` intercepts the click
    - Client-side prediction updates DOM immediately (optimistic)
    - XHR POST sent to sidecar's action endpoint
    - Sidecar executes Rust logic (may query SQLite)
    - Sidecar returns new HTML fragment
    - `azumi.js` patches any differences

4. **App Close**:
    - Tauri signals sidecar to gracefully shutdown
    - Database connections closed

### State Management

```
┌──────────────────────────────────────────────────────┐
│                    STATE FLOW                        │
├──────────────────────────────────────────────────────┤
│                                                      │
│   CLIENT (WebView)           SERVER (Sidecar)        │
│   ─────────────────          ────────────────        │
│                                                      │
│   [az-scope JSON]  ◄─────►   [Struct Instance]       │
│        │                          │                  │
│        │ Prediction               │ Truth            │
│        │ (optimistic)             │ (authoritative)  │
│        ▼                          ▼                  │
│   [DOM Update]              [DB Transaction]         │
│                                   │                  │
│   ◄───────────────────────────────┘                  │
│        Reconciliation (HTML diff)                    │
│                                                      │
└──────────────────────────────────────────────────────┘
```

---

## Implementation Strategy

### Phase 1: Project Setup

```bash
# Create Tauri app with Rust frontend
npm create tauri-app@latest azumi-desktop -- --template vanilla

# Project structure
azumi-desktop/
├── src-tauri/          # Tauri Rust code
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── src/
│       └── main.rs
├── src/                # (Unused - we serve from sidecar)
└── azumi-server/       # Your existing Azumi crate (symlink or subtree)
```

### Phase 2: Sidecar Configuration

**tauri.conf.json**:

```json
{
    "bundle": {
        "externalBin": ["azumi-server"]
    },
    "app": {
        "windows": [
            {
                "url": "http://localhost:8080",
                "title": "Azumi Desktop"
            }
        ]
    }
}
```

**src-tauri/src/main.rs**:

```rust
use tauri::api::process::Command;
use std::sync::Mutex;

struct AppState {
    server_child: Mutex<Option<tauri::api::process::CommandChild>>,
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Spawn Azumi server as sidecar
            let (mut rx, child) = Command::new_sidecar("azumi-server")
                .expect("Failed to locate sidecar")
                .spawn()
                .expect("Failed to spawn sidecar");

            // Store child handle for cleanup
            app.manage(AppState {
                server_child: Mutex::new(Some(child)),
            });

            // Optional: log server output
            tauri::async_runtime::spawn(async move {
                while let Some(event) = rx.recv().await {
                    match event {
                        tauri::api::process::CommandEvent::Stdout(line) => {
                            println!("[azumi] {}", line);
                        }
                        tauri::api::process::CommandEvent::Stderr(line) => {
                            eprintln!("[azumi] {}", line);
                        }
                        _ => {}
                    }
                }
            });

            Ok(())
        })
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event.event() {
                // Graceful shutdown of sidecar
                if let Some(state) = event.window().try_state::<AppState>() {
                    if let Some(child) = state.server_child.lock().unwrap().take() {
                        let _ = child.kill();
                    }
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Phase 3: Port Coordination

The sidecar needs to tell Tauri which port it bound to (in case 8080 is taken).

**Option A: Fixed Port with Retry**

```rust
// In Azumi server
let port = std::env::var("AZUMI_PORT").unwrap_or("8080".to_string());
let addr = format!("127.0.0.1:{}", port);
```

**Option B: Dynamic Port via Stdout**

```rust
// In Azumi server - print port for Tauri to read
let listener = TcpListener::bind("127.0.0.1:0").await?; // OS assigns port
let port = listener.local_addr()?.port();
println!("AZUMI_READY:{}", port); // Tauri parses this

// In Tauri - wait for ready signal
while let Some(event) = rx.recv().await {
    if let CommandEvent::Stdout(line) = event {
        if line.starts_with("AZUMI_READY:") {
            let port: u16 = line[12..].parse().unwrap();
            // Navigate WebView to this port
            window.eval(&format!("window.location = 'http://localhost:{}'", port))?;
            break;
        }
    }
}
```

### Phase 4: Database Path Resolution

The Azumi server needs to know where to store the SQLite database.

```rust
// In Azumi server main.rs
fn get_db_path() -> PathBuf {
    if let Ok(path) = std::env::var("AZUMI_DATA_DIR") {
        PathBuf::from(path).join("data.db")
    } else {
        // Fallback for development
        PathBuf::from("./data.db")
    }
}

// Tauri passes the path via environment variable
Command::new_sidecar("azumi-server")
    .env("AZUMI_DATA_DIR", app.path_resolver().app_data_dir().unwrap())
    .spawn()
```

---

## Known Difficulties & Solutions

### 1. **Port Conflicts**

**Problem**: Port 8080 may already be in use.

**Solutions**:

-   Use dynamic port allocation (Option B above)
-   Implement retry logic with port range
-   Show error dialog if no ports available

### 2. **Startup Latency**

**Problem**: Sidecar takes time to start; WebView shows blank/error.

**Solutions**:

-   Show a native splash screen (Tauri supports this)
-   WebView starts with a loading page, JS polls `/health` until server ready
-   Use `AZUMI_READY` signal pattern

```rust
// Loading page approach
app.get_window("main").unwrap().eval(r#"
    document.body.innerHTML = '<div style="display:flex;height:100vh;align-items:center;justify-content:center;">Loading...</div>';
    const check = setInterval(async () => {
        try {
            const res = await fetch('http://localhost:8080/health');
            if (res.ok) {
                clearInterval(check);
                window.location.reload();
            }
        } catch {}
    }, 100);
"#)?;
```

### 3. **CSP and CORS Issues**

**Problem**: WebView may block requests to `localhost`.

**Solutions**:

-   Configure Tauri's `dangerousRemoteDomainIpcAccess` (not recommended)
-   Better: Use Tauri's `protocol::asset` for static files, keep dynamic in sidecar
-   Add proper CORS headers in Azumi:

```rust
// In Axum
use tower_http::cors::{CorsLayer, Any};

let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any);

let app = Router::new()
    // ...routes
    .layer(cors);
```

### 4. **Hot Reload in Development**

**Problem**: Need to restart sidecar on code changes.

**Solutions**:

-   Use `cargo-watch` or `systemfd` for the sidecar
-   Tauri's dev mode can watch and restart sidecar
-   Alternative: Run sidecar manually during dev, only bundle for release

```bash
# Development workflow
# Terminal 1: Azumi server with watch
cargo watch -x run -w src

# Terminal 2: Tauri dev (WebView only)
npm run tauri dev -- --no-watch
```

### 5. **Binary Size**

**Problem**: Bundling two Rust binaries (Tauri + Azumi) increases app size.

**Analysis**:

-   Tauri shell: ~5-10MB
-   Azumi server (release, stripped): ~5-15MB depending on dependencies
-   Total: ~15-25MB (still much smaller than Electron's 150MB+)

**Mitigations**:

-   Use `strip = true` and `lto = true` in release profile
-   Consider `upx` compression for sidecar
-   Exclude unnecessary dependencies

### 6. **Graceful Shutdown**

**Problem**: Killing sidecar abruptly may corrupt SQLite or lose data.

**Solution**: Signal-based shutdown

```rust
// In Azumi server
use tokio::signal;

let shutdown_signal = async {
    signal::ctrl_c().await.ok();
};

axum::serve(listener, app)
    .with_graceful_shutdown(shutdown_signal)
    .await?;
```

```rust
// In Tauri - send SIGTERM instead of kill
#[cfg(unix)]
child.signal(nix::sys::signal::Signal::SIGTERM)?;
#[cfg(windows)]
child.kill()?; // Windows doesn't have SIGTERM
```

### 7. **Security: Open Port on localhost**

**Problem**: Other apps on the machine could theoretically access `localhost:8080`.

**Mitigations**:

-   Use a random high port
-   Implement a shared secret (passed via env var, checked on each request)
-   Bind to `127.0.0.1` only (never `0.0.0.0`)

```rust
// Shared secret approach
let secret = std::env::var("AZUMI_SECRET").expect("Secret required");

async fn verify_secret(
    headers: HeaderMap,
    next: Next,
) -> Response {
    if headers.get("X-Azumi-Secret").map(|v| v.to_str().ok()) != Some(Some(&secret)) {
        return StatusCode::FORBIDDEN.into_response();
    }
    next.run(req).await
}

// Tauri injects secret as env var and header
Command::new_sidecar("azumi-server")
    .env("AZUMI_SECRET", generate_random_secret())
    .spawn()
```

### 8. **Updates and Distribution**

**Problem**: How to update the app?

**Solutions**:

-   Use Tauri's built-in updater (checks GitHub releases)
-   Sidecar is bundled with the app, updates together
-   Consider delta updates for faster downloads

---

## Comparison with Alternatives

| Approach                    | Bundle Size | Complexity | Offline | Rust % | Web Parity     |
| --------------------------- | ----------- | ---------- | ------- | ------ | -------------- |
| **Azumi + Tauri (Sidecar)** | ~20MB       | Medium     | ✅      | 100%   | ✅ Perfect     |
| **Leptos + Tauri (WASM)**   | ~15MB       | High       | ✅      | 100%   | ⚠️ WASM quirks |
| **Dioxus Desktop**          | ~10MB       | Medium     | ✅      | 100%   | ❌ Not web     |
| **Electron + Any**          | ~150MB+     | Low        | ✅      | 0-50%  | ✅ Perfect     |
| **Tauri + Svelte**          | ~15MB       | Medium     | ✅      | ~30%   | ✅ Good        |

### When to Choose Azumi + Tauri

✅ **Good fit if**:

-   You already have an Azumi web app
-   You want identical behavior on web and desktop
-   You prefer server-rendering mental model
-   You want to share DB/logic between web API and desktop

❌ **Not ideal if**:

-   You need deep native UI (custom window chrome, complex menus)
-   You want zero HTTP overhead (direct memory access)
-   Your app is purely offline-first with no server concept

---

## Proof of Concept Outline

### Step 1: Minimal Azumi App

Create a simple counter app with SQLite persistence.

### Step 2: Add Health Endpoint

```rust
.route("/health", get(|| async { "OK" }))
```

### Step 3: Create Tauri Wrapper

Use the sidecar pattern from Phase 2.

### Step 4: Test Locally

-   `cargo build --release -p azumi-server`
-   `npm run tauri dev`

### Step 5: Bundle for Distribution

-   `npm run tauri build`
-   Test on Windows/macOS/Linux

---

## Open Questions

1. **Should we support Tauri Commands for power-user features?**

    - E.g., file dialogs, notifications via Tauri API instead of HTTP

2. **How to handle multiple windows?**

    - Each window could share the same sidecar, or spawn its own

3. **Can we eliminate the sidecar entirely?**

    - Theoretically, Axum could run _inside_ Tauri's Rust code, responding to a custom protocol
    - This would require significant refactoring of Azumi's runtime

4. **Mobile support?**
    - Tauri 2.0 supports iOS/Android
    - Same sidecar approach should work, but needs testing

---

## Conclusion

Integrating Azumi with Tauri is **architecturally sound** and **practically feasible**. The sidecar pattern preserves Azumi's SSR model while gaining native distribution benefits.

The main challenges are operational (port coordination, startup timing, graceful shutdown) rather than fundamental. These are well-understood problems with established solutions.

**Recommended next step**: Build a minimal PoC to validate the architecture before committing to a full integration.
