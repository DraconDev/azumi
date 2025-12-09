# Pattern: Manual Extensions vs. Custom Extractors

In Azumi (and Axum), there are two main ways to get data (like a User) from middleware into your page handler. Both are correct, but one is cleaner.

## 1. The "Manual" Way (Azumi Starter)

This is the default Axum way. It is **Explicit**. You can see exactly what is happening.

### How it works:

1.  **Middleware** inserts data into `req.extensions_mut()`.
2.  **Handler** asks for `Extension<T>`.

### Code Example:

```rust
// 1. Setup in Middleware
req.extensions_mut().insert(User { name: "Dracon" });

// 2. Usage in Handler
// ⚠️ You MUST wrap your type in `Extension<>`
pub async fn profile_page(
    Extension(user): Extension<User>
) -> Html<String> {
    // Access data
    format!("Hello {}", user.name)
}
```

### Pros & Cons

-   ✅ **Explicit**: You know exactly where data comes from.
-   ✅ **Zero Boilerplate**: No extra structs needed.
-   ❌ **Verbose**: You have to type `Extension(...)` in _every single handler_.

---

## 2. The "Extractor" Way (Lesson 19)

This is the "Azumi Pro" way. It uses Rust traits to hide the plumbing.

### How it works:

1.  **Middleware** inserts data (same as above).
2.  **We define a Helper Struct** (`CurrentUser`) that implements `FromRequestParts`.
3.  **Handler** asks for `CurrentUser`.

### Code Example:

```rust
// 1. The Setup (Once per project)
pub struct CurrentUser(pub User);

impl FromRequestParts<S> for CurrentUser {
    async fn from_request_parts(...) {
        // The "Plumbing" is hidden here!
        let Extension(user) = ...;
        Ok(CurrentUser(user))
    }
}

// 2. Usage in Handler
// ✨ Clean! No wrapper needed.
pub async fn profile_page(
    CurrentUser(user): CurrentUser
) -> Html<String> {
    format!("Hello {}", user.name)
}
```

### Pros & Cons

-   ✅ **Clean**: Handlers look meaningful (`CurrentUser` vs `Extension`).
-   ✅ **Refactor Safe**: If you change how you fetch users (e.g., from DB instead of Extension), you only change the _Extractor_, not 50 handlers.
-   ❌ **Setup Cost**: Requires ~10 lines of boilerplate code (once).

## Recommendation

-   **Start with Manual**. It's easier to debug.
-   **Switch to Extractors** when you have 5+ handlers using the same data.
