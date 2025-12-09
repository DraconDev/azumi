# 🚀 Azumi: The Compiler-Driven Web Framework

> **Zero JavaScript. 100% Rust. Compile-Time Safe.**

Azumi is a paradigm-shifting web framework for Rust that moves the entire frontend runtime responsibility to the **compiler**. By analyzing your Rust code at compile time, Azumi validates your HTML, scopes your CSS, and generates sophisticated optimistic UI updates without you writing a single line of client-side JavaScript.

---

## � Why Azumi?

Most frameworks force you to choose: **Safety** (Server-Side) or **Interactivity** (Client-Side SPA). Azumi gives you both.

| Feature             | Logic               | Latency | Validation       |
| :------------------ | :------------------ | :------ | :--------------- |
| **Traditional SSR** | Server              | ~100ms  | Runtime          |
| **React / SPA**     | Client (JS)         | ~10ms   | Runtime          |
| **Azumi**           | **Compiler (Rust)** | **0ms** | **Compile-Time** |

### Core Pillars

1.  **Compiler-Driven Optimistic UI**: Write mutable Rust methods (`self.count += 1`). The compiler analyzes these mutations and generates exact JavaScript instructions to update the DOM instantly. If the server disagrees, the client rolls back automatically.
2.  **Impossible Bugs**:
    -   **HTML**: `<img>` missing `alt`? **Compile Error.**
    -   **CSS**: Used class `.btn-primry` but defined `.btn-primary`? **Compile Error.**
    -   **Forms**: Input name `usr_name` doesn't match struct `user_name`? **Compile Error.**
3.  **Signed State Security**: All component state sent to the client is signed with **HMAC-SHA256**. Users cannot tamper with `is_admin` flags in the DOM.

---

## 📦 The Ecosystem

Azumi is designed as a complete stack.

-   **`azumi`** (The Crate): The core framework, macros, and standard library.
-   **`azumi-starter`** (The Template): A production-ready **Microservice Template** with:
    -   Pre-configured **Postgres** (SQLx) & **Redis**.
    -   **gRPC Clients** for Auth/Payment services.
    -   **Phase 3 Auth** patterns (Extractors).
-   **`demo`** (The School): An interactive application with **20 Lessons** teaching you Azumi from scratch.

---

## 📚 The Manual

### 1. Components & Syntax

Azumi uses the `html!` macro (similar to JSX but stricter).

```rust
#[azumi::component]
fn UserCard(name: &str, is_pro: bool) -> impl Component {
    html! {
        // 1. Control Flow with @if, @for, @match
        @if is_pro {
            <span class="badge">"PRO"</span>
        }

        // 2. Variable Injection (Curly Braces)
        <h1>"Hello, " {name}</h1>

        // 3. Component Composition (@Syntax)
        @Avatar(url="/me.png")
    }
}
```

### 2. The Styling System

Azumi's styling system is **Co-Validated**. It ensures your HTML classes actually exist in your styles.

#### Automatic Scoping

Styles are strictly isolated. A generic class `.card` becomes `.card-s7f2` automatically.

```rust
html! {
    <style>
        .card { background: "white"; } // Defines 'card' variable scope
    </style>

    // ✅ Uses variable 'card' (Verified at compile time)
    <div class={card}>...</div>

    // ❌ Error: "Class 'container' not found in style block"
    // <div class={container}>...</div>
}
```

#### Dynamic Variables

Pass Rust state directly into CSS Custom Properties.

```rust
<div class={progress} style="--width: {state.percent}%; --color: {state.color}">
```

_(Note: Inline static style strings `style="color: red"` are BANNED to enforce consistency.)_

### 3. Live State (Optimistic UI)

This is the magic. Define a struct with `#[azumi::live]` and methods to mutate it.

```rust
// 1. Define State
#[azumi::live]
pub struct Counter {
    pub count: i32,
}

// 2. Define Actions (Server + Compiler Analysis)
#[azumi::live_impl(component = "counter_view")]
impl Counter {
    pub fn increment(&mut self) {
        // The compiler sees this mutation and generates JS:
        // `node.textContent = Number(node.textContent) + 1`
        self.count += 1;
    }
}

// 3. Bind Events
#[azumi::component]
fn counter_view(state: &Counter) -> impl Component {
    html! {
        // on:event declarative binding
        <button on:click={state.increment}>"Click"</button>
        <span data-bind="count">{state.count}</span>
    }
}
```

### 4. Forms & Data Binding

Bind generic HTML forms directly to Rust structs.

```rust
#[derive(Schema)]
struct Login {
    email: String,
}

html! {
    // bind={Struct} validates names at compile time
    <form bind={Login}>
        // Name must match struct field "email"
        <input name="email" type="text" />
    </form>
}
```

### 5. Authentication (The Extractor Pattern)

Azumi (via Axum) promotes Type-Driven Security. Never manually check `if user.is_admin`.

```rust
// ❌ Dangerous (Easy to forget)
pub async fn delete_db(Extension(user): Extension<User>) {
    if !user.admin { return; }
    // ...
}

// ✅ Safe (Impossible to misuse)
// The handler WON'T RUN if the extractor fails.
pub async fn delete_db(admin: AdminUser) {
     // ...
}
```

### 6. Asset Pipeline (Phase 3)

Azumi includes a Vite-like asset pipeline built into `cargo build`.

-   **Hashing**: `static/logo.png` -> `assets/logo.a8b9...png`.
-   **Rewriting**: `html! { <img src="/static/logo.png"> }` is automatically rewritten.
-   **Minification**: CSS in `<style>` blocks is minified.

---

## 🛠️ Getting Started

### Option A: The Learning Path (Recommended)

Building the `demo` app is the best way to learn.

```bash
git clone https://github.com/DraconDev/azumi
cd azumi/demo
cargo run
# Open http://localhost:3000
```

### Option B: The Production Path

Start a new real-world project.

```bash
git clone https://github.com/DraconDev/azumi-starter my-app
cd my-app
# Configure database in .env
cargo run
```

---

## 🔮 Roadmap v0.2

-   **CLI**: `cargo azumi new`
-   **Testing**: Advanced network simulation suite.
-   **Performance**: Improved DOM diffing.

---

**License**: MIT
