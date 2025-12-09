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

## 3. Scaling to Big Projects (Why Extractors Win)

In a large project (50+ routes), **Extractors are essential**. Here is why:

### A. Centralized Logic ("Type-Driven Security")

Imagine you have an Admin section.

-   **Manual**: You must remember to check `if !user.is_admin { return 403; }` in every single admin handler. If you forget one, you have a security hole.
-   **Extractor**: You create a `AdminUser` extractor. If a non-admin tries to access the route, the _Extractor_ rejects them automatically. The handler _never even runs_.

```rust
// The Handler guarantees the user is an Admin
pub async fn delete_database(
    AdminUser(admin): AdminUser // <--- This fails automatically if not admin
) {
    // Safe to delete!
}
```

### B. Refactoring Proof

If you switch from **Cookies** to **JWT Tokens**:

-   **Manual**: You might have to find-replace `Extension<User>` with `Extension<Claims>` everywhere if the type changes.
-   **Extractor**: You just update the `impl FromRequestParts for CurrentUser` block. The 50 handlers don't even know anything changed.

### C. Testing

Extractors are easier to unit test because they are self-contained logic units.
