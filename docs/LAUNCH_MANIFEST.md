# Azumi v0.1: Launch Manifest

> "Ready for Showtime"

Azumi has successfully completed its initial 3-Phase Development Roadmap. The framework is now functionally complete, secure, and ready for building production-grade applications.

## 🚀 Core Features (Delivered)

### 1. The "Zero-JS" Promise

-   **Optimistic UI**: Write Rust struct methods, get instant client-side updates.
-   **Compiler Predictions**: The compiler analyzes Rust code (`self.count += 1`) and generates the equivalent JavaScript automatically.
-   **Live State**: State is persisted between renders using secure, signed tokens.

### 2. Security (Phase 3 Complete)

-   **Signed State**: All state sent to the client is HMAC-SHA256 signed. It is impossible for users to tamper with `is_admin` or other fields.
-   **Type-Safe Auth**: The new **Extractor Pattern** allows handlers to strictly enforce authentication at the type level (`fn handler(user: AdminUser)`).
-   **CSRF Protection**: Native double-submit cookie patterns.

### 3. Production Assets (Phase 3 Complete)

-   **Hashed Assets**: Files in `/static` are automatically hashed (`logo.a8b9c7d6.png`) for immutable caching.
-   **Minification**: CSS in `<style>` blocks is minified at compile time.
-   **Rewriting**: The `html!` macro automatically rewrites paths (`src="/static/logo.png"`) to their hashed versions.

### 4. Developer Experience

-   **Macros**: `#[azumi::component]`, `html!`, `#[azumi::live]` make usage ergonomic.
-   **Error Messages**: Strict compile-time validation for CSS and Attributes prevents runtime crashes.
-   **Snippets**: Visual Studio Code snippets (implied) and comprehensive docs.

---

## 📦 Project Structure

### `azumi` (The Framework)

-   **Core**: The Runtime and Traits.
-   **Macros**: The Compiler magic (`syn`-based parsing).

### `azumi-starter` (The Template)

-   **Microservice Ready**: Pre-configured with gRPC clients (Auth/Payment).
-   **Database**: SQLx + Postgres setup.
-   **Authentication**: Now uses the new **Phase 3 Extractor Pattern**.

### `demo` (The Learnings)

-   **Lessons 0-19**: A complete interactive tutorial from "Hello World" to "Complex Auth".

---

## 🔮 What's Next? (Post-Launch)

While v0.1 is feature complete, the roadmap for v0.2 includes:

1.  **WASM Option**: Offloading complex predictions to WebAssembly.
2.  **Advanced CLI**: `cargo azumi new` generator.
3.  **LSP Server**: An official Language Server for even better IDE feedback.

## ✅ Conclusion

**Azumi is ready.** You can ship `azumi-starter` to production today.
