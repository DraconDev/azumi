# Azumi Phase 3: Production Infrastructure & Stability

**Goal:** Transform Azumi from a "proven prototype" into a "production-grade framework" capable of powering real-world, high-traffic applications.

This phase focuses on the "boring but critical" infrastructure: Assets, Testing, and Security.

---

## 🏗️ 1. The Asset Pipeline (Asset Optimization)

**Current State:**
Basic static file serving. Assets are requested by name (e.g., `/static/logo.png`).
**Problem:**

-   **Caching**: No cache busting. If you update `logo.png`, users with cached versions won't see it.
-   **Performance**: CSS files are served raw (whitespace included). Images aren't optimized.
-   **Network**: Multiple small CSS files cause waterfalls (though HTTP/2 mitigates this).

**The Solution: `azumi-assets` Crate**

We will introduce a build-time and runtime asset management system.

### A. Content-Addressable Hashing (Cache Busting)

Instead of serving `style.css`, we serve `style.a8b3c9.css`.

-   **Compile Time**: A `build.rs` script or macro scans the `/assets` folder.
-   **Hashing**: Computes BLAKE3/SHA256 hashes of file contents.
-   **Manifest**: Generates a static map: `STATIC_ASSETS: Map<&str, &str>`.
-   **Usage**:
    ```rust
    html! {
        <link rel="stylesheet" href={asset!("/css/main.css")} />
        // Expands to: href="/static/main.a8b3c9.css"
    }
    ```
-   **Caching Headers**: `Immutable` caching headers (1 year) for hashed files.

### B. CSS Processing & Minification

Since Azumi already parses CSS in `html!` macros, we have a unique opportunity:

-   **Extraction**: Extract critical CSS from components into a single optimized bundle? (Optional, maybe stick to runtime injection for simplicity first).
-   **Minification**: At compile time, strip whitespace/comments from the `style` blocks before embedding them in the binary. This reduces binary size and transfer size.

### C. Image Optimization

-   **Optimization**: `build.rs` task to compress images (WebP/AVIF generation) automatically.
-   **Lazy Loading**: (Already implemented natively by HTML, but we can enforce `loading="lazy"` via the macro).

---

## 🧪 2. The Testing Harness (`azumi::test`)

**Current State:**
No standard way to test components. Reliability relies on manual verification.

**The Solution: Integrated Unit Testing**

We need a testing framework that operates _without_ a browser but validates the DOM logic.

### A. Component Rendering Tests

Test that a component renders the expected initial HTML given a specific state.

```rust
#[test]
fn test_counter_render() {
    let state = Counter { count: 10 };
    let html = azumi::test::render(&state, counter_view);

    assert!(html.contains("Count: 10"));
    assert_selector!(html, ".value", "10"); // jQuery-like assertions
}
```

### B. Live Logic Tests (The "Simulator")

Test that calling an action results in the correct State mutation AND the correct optimistic prediction.

```rust
#[tokio::test]
async fn test_increment_logic() {
    let mut state = Counter { count: 0 };

    // Simulate Action
    state.increment();

    // Assert State Change
    assert_eq!(state.count, 1);

    // Assert Prediction Generation (Advanced)
    let prediction = azumi::test::analyze_prediction(Counter::increment);
    assert_eq!(prediction.js, "this.count += 1"); // Verify generated JS matches intent
}
```

---

## 🔒 3. Security & Middleware

**Current State:**
No standard auth pattern. `az-scope` exposes raw state to the client.

**The Solution: Secure Session Integration**

### A. Signed/Encrypted State

The `az-scope` payload (the state sent to client) should be integrity-protected.

-   **Signature**: HMAC-SHA256 signature to prevent client-side tampering of state before "action" submission.
-   **Encryption**: Option to encrypt state if it contains sensitive data (though we recommend not sending sensitive data to client at all).

### B. `#[azumi::middleware]`

Standardize how to wrap `live` handlers with Axum middleware (Auth guards).

---

## 📊 Summary of Phase 3 Tasks

1.  **Create `azumi-assets`**: Implementing the hashing and manifest generation.
2.  **Macro Update**: Add `asset!()` macro to `azumi-macros`.
3.  **Create `azumi-test`**: Test helpers and standard `assert_selector!` macros.
4.  **Minifier**: Implement simple regex-based CSS minification in the `style!` parser.
5.  **Documentation**: Guide on "Deploying Azumi to Production" (Nginx setup, caching rules).
