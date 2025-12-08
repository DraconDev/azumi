# Azumi Project Roadmap

## 🏁 Phase 1: Foundation (Completed)

-   [x] **Core Macro System**: `#[azumi::component]`, `html!`, `#[azumi::live]`.
-   [x] **Strict Syntax**: Compiler-enforced rules for classes, IDs, and styles.
-   [x] **Live State Analysis**: Auto-generation of optimistic UI predictions (`data-predict`).
-   [x] **Documentation**: Comprehensive guide and 15 interactive lessons.
-   [x] **Demo**: Functional server with client-side runtime script injection.

---

## 🚧 Phase 2: Production Readiness (Proposed)

While the current version works well for demos and in-memory apps, real-world production usage requires addressing three key gaps:

### 1. Async & Database Integration

**The Gap:**
Currently, `#[azumi::live_impl]` generates synchronous code. Real applications need to query databases (e.g., `sqlx`, `tokio-postgres`) or call external APIs, which requires `async/await`.

**Remediation Plan:**

-   **Update Macros**: Modify `macros/src/live.rs` to support `async fn` in live implementation blocks.
-   **Handler Generation**: Ensure generated Axum handlers properly `.await` the user's methods.
-   **Dependency Injection**: Establish a pattern for injecting database pools (e.g., `PgPool`) into the component state or context, likely via Axum's `FromRef` state extractor.

### 2. Testing Story

**The Gap:**
There is currently no standard way to unit test Azumi components or Verify that optimistic predictions match server logic.

**Remediation Plan:**

-   **Unit Testing Pattern**: Document strict patterns for testing `LiveState` structs (which are just Rust structs).
-   **Component Testing**: Create a utility (e.g., `azumi::test::render`) to assert on the generated HTML output of a component without a full browser.
-   **New Lesson**: Add "Lesson 16: Testing" to demonstrate TDD with Azumi.

### 3. Middleware & Authentication

**The Gap:**
The current demo has no authentication. Users need to know how to protect `live` routes.

**Remediation Plan:**

-   **Middleware Integration**: Verify and document how standard Axum middleware (e.g., `tower-http`) interacts with Azumi's auto-generated routes.
-   **Secure State**: Ensure sensitive state (like User IDs) is handled securely in the `az-scope` serialization (encryption or session-based lookups).

---

## 🔮 Phase 3: Ecosystem (Future)

-   **Component Library**: Built-in accessible implementations of Modals, Dropdowns, and Toast notifications.
-   **DevTools**: A browser extension or overlay to visualize the "Live" WebSocket/Action traffic and predictions.
-   **WASM fallback**: Option to run local interaction logic via WASM instead of raw DOM operations for complex interaction logic.
