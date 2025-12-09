# 🚀 Azumi: The Compiler-Driven Web Framework

> **Zero JavaScript. 100% Rust. Compile-Time Safe.**

Azumi is a paradigm-shifting web framework that eliminates entire classes of runtime errors by moving them to compile-time. It validates your HTML structure, scopes your CSS automatically, and generates optimistic client-side predictions from your Rust code—all without you writing a single line of JavaScript.

---

## 📦 The Azumi Ecosystem

Azumi is more than just a crate. It is a complete ecosystem for building production-grade web applications.

### 1. `azumi` (The Core Framework)

The engine powering everything. It includes the `html!` macro, the component system, local state management, and the `syn`-based compiler that generates client-side logic.

-   [Crate Documentation](https://docs.rs/azumi)
-   [Source Code](https://github.com/DraconDev/azumi)

### 2. `azumi-starter` (The Production Template)

**Start here for new apps.** A pre-configured, microservice-ready boilerplate.

-   **Stack**: Azumi + Axum + SQLx (Postgres) + Redis.
-   **Architecture**: gRPC-ready (includes clients for Auth/Payment services).
-   **Security**: Pre-configured with the **Phase 3 Auth Extractor Pattern**.
-   **Usage**:
    ```bash
    git clone https://github.com/azumi/azumi-starter my-app
    cd my-app
    cargo run
    ```

### 3. `demo` (The Learning Platform)

An interactive educational platform built _with_ Azumi, _for_ learning Azumi.

-   **Content**: 20 Interactive Lessons (Hello World → Advanced Auth).
-   **Usage**:
    ```bash
    cd demo
    cargo run
    # Visit http://localhost:3000 to start learning.
    ```

---

## 🛡️ Core Pillars

### 1. CSS-HTML Co-Validation

Azumi is the only framework that validates the relationship between your styles and your markup at compile time.

```rust
html! {
    <style>
        .btn_primary { ... }
    </style>

    // ✅ Compiles: Class exists in scope
    <button class={btn_primary}>"Click Me"</button>

    // ❌ Compile Error: 'btn_prmary' not found in style block!
    // <button class={btn_prmary}>"Typo"</button>
}
```

### 2. Compiler-Driven Optimistic UI (Zero-Latency)

Write standard Rust methods. The compiler analyzes your code and generates instant client-side predictions.

```rust
#[azumi::live]
pub struct Counter { count: i32 }

#[azumi::live_impl(component = "counter_view")]
impl Counter {
    pub fn increment(&mut self) {
        // compiler generates: `state.count = state.count + 1` (JS)
        self.count += 1;
    }
}
```

**Result**: 0ms latency for the user. The server runs the real logic asynchronously and corrects the client if it was wrong (Smart Reconciliation).

### 3. Signed State Security (HMAC-SHA256)

Client-side state is cryptographically signed.

-   Consumers cannot tamper with `is_admin` flags.
-   Replay attacks are prevented via nonces.
-   **Zero Config**: It works automatically out of the box.

---

## 🎨 Styling System

### Automatic Scoping

Forget BEM. Forget CSS Modules.

```rust
// In Rust: .card { ... }
// Output:  .card-s7a9f2 { ... }
```

Styles defined in `<style>` blocks are automatically scoped to the component.

### Dynamic Styles

Pass Rust variables directly into CSS variables.

```rust
<div class={progress_bar} style="--width: {state.percent}%; --color: {state.color}">
```

_Note_: Inline `style="..."` strings are BANNED. You must use the typed `--var` syntax.

---

## 🔒 Authentication Patterns

Azumi promotes the **Extractor Pattern** for type-safe security.

**Old Way (Middleware manual check):**
❌ `handler(Extension(user))` -> `if !user.is_admin { return Error }`

**Azumi Way (Type-Driven):**
✅ `handler(user: AdminUser)`

The handler _cannot run_ unless the user is authenticated and authorized. This logic is centralized in reusable Extractors found in `azumi-starter`.

---

## 🚀 Production Features (Phase 3)

### Asset Pipeline

-   **Auto-Hashing**: `logo.png` -> `logo.a8b9c7d6.png` for immutable caching.
-   **Auto-Rewriting**: `html! { <img src="/logo.png"> }` rewrites the path automatically.
-   **Minification**: CSS in `<style>` blocks is stripped of whitespace at compile time.

### Forms

-   **Data Binding**: `bind={StructName}` ensures your form fields match your Rust server structs.
-   **CSRF**: Built-in double-submit protection.

---

## 🔮 Roadmap (v0.2)

1.  **CLI Tool**: `cargo azumi new` for instant scaffolding.
2.  **Testing**: Expanded simulation suite (`azumi::test`) for slow-network rehearsal.
3.  **Performance**: Granular DOM diffing algorithms.

---

## 📄 License

MIT License.

---

**Ready to build?**

-   **New App**: Clone [`azumi-starter`](./azumi-starter)
-   **Learn**: Run [`demo`](./demo)
