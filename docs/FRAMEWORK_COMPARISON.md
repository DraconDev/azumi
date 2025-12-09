# ⚔️ Azumi vs. The World: Framework Comparison

Azumi occupies a unique niche in the web development landscape: **Compiler-Driven Optimistic UI**.

Most frameworks force you to choose between:

1.  **SPA / Hydration** (Leptos, Dioxus, Next.js) - Heavy client bundles, complex state sync, "loading..." spinners.
2.  **SSR / Templates** (Maud, Askama, Templ) - Fast initial load, but "dumb" clients that need manual JS or HTMX for interactivity.

**Azumi creates a third category:** It writes "dumb" server-side code that the **compiler** transforms into a smart, optimistic client app—without you writing a single line of client logic.

---

## 🆚 Rust Ecosystem Comparison

| Feature            | **Azumi**                           | **Maud**              | **Leptos / Dioxus**              | **Askama / Tera**       |
| :----------------- | :---------------------------------- | :-------------------- | :------------------------------- | :---------------------- |
| **Primary Model**  | SSR + Compiler-Driven Interactivity | Pure SSR (HTML Macro) | WASM (Client-Side Rendering)     | SSR (String Templating) |
| **Client Logic**   | **Compiler Generated**              | Manual JS / HTMX      | WASM Bundle                      | Manual JS / HTMX        |
| **Bundle Size**    | **Minimal (<20kb)**                 | N/A (0kb)             | Heavy (>300kb WASM)              | N/A (0kb)               |
| **Optimistic UI**  | **Automatic (Local + CSS)**         | Manual JS Required    | Yes (Client-Side State)          | Manual JS Required      |
| **CSS Validation** | **Co-validated with HTML**          | None                  | Limited / External               | None                    |
| **Learning Curve** | Low (Write struct, get UI)          | Low (Write HTML)      | High (Async, Signals, lifetimes) | Low                     |

### vs. Maud

-   **Maud** is brilliant for static HTML generation but stops there. If you want a button to update a counter, you must write a `<script>` tag or wire up HTMX yourself.
-   **Azumi** feels like Maud (just writing Rust macros), but when you add `#[azumi::live]`, the compiler _generates_ the necessary client-side code for you.

### vs. Leptos / Dioxus (WASM)

-   **WASM Frameworks** treat the browser as an operating system. They ship a runtime (WASM blob) that takes over the page. This is powerful for heavy apps (like Photoshop in the browser) but overkill for 99% of websites.
-   **Azumi** treats the browser as a document viewer. It enhances standard HTML with tiny, surgical JavaScript. No heavy WASM download, instant interactive (TTI).

---

## 🆚 JavaScript Ecosystem Comparison

| Feature         | **Azumi**               | **Next.js / React**            | **Svelte**         | **HTMX**        |
| :-------------- | :---------------------- | :----------------------------- | :----------------- | :-------------- |
| **Language**    | Rust                    | TypeScript / JS                | TypeScript / JS    | HTML Attributes |
| **Type Safety** | **Total (Full Stack)**  | Partial (API boundary is weak) | Partial            | None            |
| **State Sync**  | **Compiler Guaranteed** | Manual / Hydration             | Manual / Hydration | Server-Driven   |
| **Performance** | Native Code Speed       | V8 Runtime Overhead            | Fast DOM Updates   | DOM Swapping    |

### vs. Next.js (React)

-   **Next.js** relies on "Hydration"—re-running your server code on the client to attach event listeners. This causes the "Uncanny Valley" where a button is visible but doesn't work yet.
-   **Azumi** has no hydration. The HTML is interactive immediately, and state updates are instant (optimistic) while the server confirms in the background.

### vs. Svelte

-   **Svelte** is the closest philosophical cousin. Both use a compiler to optimize code.
-   **Difference:** Svelte compiles to JS to run in the browser. Azumi compiles to native machine code on the server and generates a "protocol" for the client. Azumi avoids the "API Layer" entirely—your UI _is_ your API.

---

## 🆚 Other Ecosystems (Go, etc.)

### vs. Templ (Go)

-   **Templ** is "Maud for Go". Great for type-safe HTML, but has no story for interactivity. You're back to writing `document.querySelector` or pulling in Alpine.js.
-   **Azumi** integrates the interactivity into the component definition itself, keeping state and UI co-located and type-checked together.

---

## 🏆 Summary: Why Choose Azumi?

Choose **Azumi** if you want:

1.  **The speed of SSR** (Maud/Templ)
2.  **The interactivity of an SPA** (React/Leptos)
3.  **The safety of Rust**
4.  **Without the complexity** of WASM, JSON APIs, or Hydration.

It is the **"Have Your Cake and Eat It Too"** framework.
