# 🚀 Azumi: The Compiler-Driven Web Framework

> **Zero JavaScript. 100% Rust. Instant UI.**

Azumi is a revolutionary web framework that brings **compile-time safety** to the entire frontend stack. It validates your HTML, scopes your CSS, and generates optimistic UI updates from your Rust code—all before you even run the app.

[Start Learning (interactive demo)](http://localhost:3000) | [Documentation (docs.rs)](https://docs.rs/azumi)

---

## ⚡ Why Azumi?

### 1. Compiler-Driven Optimistic UI (Zero Latency)

Write standard Rust struct methods. The compiler analyzes your mutations (`self.count += 1`) and generates the equivalent JavaScript for you.

-   **Result**: Instant UI updates (0ms latency) without writing a single line of JS.
-   **Fallback**: The server always executes the logic. If the client prediction was wrong, it is corrected automatically.

### 2. The "Impossible Bug" Guarantee

Azumi catches errors at compile time that most frameworks miss until runtime:

-   ✅ **HTML Validation**: `<img>` missing `alt`? Compile Error.
-   ✅ **CSS Co-Validation**: Used `.btn_primary` in HTML but forgot it in `<style>`? Compile Error.
-   ✅ **Type-Safe Forms**: Field names checked against Rust structs.

### 3. Production Ready (Phase 3 Complete)

-   🔒 **Signed State**: All client state is HMAC-SHA256 signed. Tampering is impossible.
-   📦 **Asset Pipeline**: Automatic content-hashing (`logo.a8b9c7d6.png`) for cache busting.
-   🔑 **Type-Safe Auth**: Extractors (`fn handler(user: AdminUser)`) enforce security at the type level.

---

## 🛠️ Quick Start

### 1. Define Live State

```rust
#[azumi::live]
pub struct Counter {
    pub count: i32,
}

#[azumi::live_impl(component = "counter_view")]
impl Counter {
    // This Rust code runs on the server AND allows the compiler
    // to generate a client-side prediction!
    pub fn increment(&mut self) {
        self.count += 1;
    }
}
```

### 2. Build the UI

```rust
#[azumi::component]
fn counter_view(state: &Counter) -> impl Component {
    html! {
        <style>
            .btn { @apply bg-blue-500 text-white p-2 rounded; }
        </style>

        <div>
            <h1>"Count: " {state.count}</h1>
            // Declarative Event Binding
            <button on:click={state.increment} class={btn}>
                "Increment"
            </button>
        </div>
    }
}
```

---

## 📚 Project Structure

-   **`azumi`**: The Core Framework (Macros + Runtime).
-   **`azumi-starter`**: A Microservice-ready template with generic Auth/Payment gRPC clients.
-   **`demo`**: The Interactive Learning Platform.
    -   Run `cd demo && cargo run` to access **20 Interactive Lessons**.

## 🗺️ Roadmap (v0.2)

-   **Refined CLI**: `cargo azumi new`
-   **Testing Suite**: Enhanced network simulation.
-   **Granular Performance**: Advanced diffing algorithms.

---

## 📄 License

MIT License. Built with ❤️ in Rust.
