# ⚔️ The Great Framework Comparison: Azumi vs. The World

Azumi is not just "another framework"; it represents a fundamental shift in how we think about web application architecture. It pioneers the **Compiler-Driven Optimistic UI (CDO)** model.

This document provides a deep, technical comparison between Azumi and major players in the web ecosystem.

---

## 📊 The Big Feature Matrix

| Feature              | **Azumi** 🦀                      | **Maud / Templ** 📝          | **Leptos / Dioxus** 🕸️  | **Next.js (React)** ⚛️     | **Svelte 5** 🟠     | **HTMX** 🔌                  |
| :------------------- | :-------------------------------- | :--------------------------- | :---------------------- | :------------------------- | :------------------ | :--------------------------- |
| **Primary Paradigm** | **Compiler-Driven Optimistic UI** | Pure SSR (String Templating) | WASM SPA (CSR)          | Hybrid SSR/CSR (Hydration) | Compiler-Driven SPA | HTML-over-the-wire           |
| **Language**         | **Rust**                          | Rust / Go                    | Rust                    | TypeScript                 | TypeScript          | HTML Attributes              |
| **Interactivity**    | **Instant (Zero Latency)**        | None (Static)                | Varied (WASM Load Time) | Delayed (Hydration Gap)    | Fast                | Network Latency (Round Trip) |
| **Client Bundle**    | **Tiny (<20kb)**                  | 0kb                          | Heavy (>300kb)          | Heavy (>200kb)             | Small (<50kb)       | Tiny (~14kb)                 |
| **State Sync**       | **Compiler Guaranteed**           | N/A                          | Manual (Signals)        | Manual (Hooks/Context)     | Runes / Signals     | Server-Driven                |
| **Build Step**       | **Macro Expansion**               | Macro Expansion              | WASM Compilation        | Webpack / Turbopack        | Svelte Compiler     | None                         |
| **CSS Handling**     | **Co-Validated & Scoped**         | None (Global CSS)            | Component Scoped        | CSS Modules / Tailwind     | Scoped              | Global / Tailwind            |
| **Type Safety**      | **Full Stack (End-to-End)**       | Server Only                  | Full Stack              | Partial (API Boundary)     | Partial             | None                         |
| **Hydration?**       | **NO (Resumable-ish)**            | No                           | Yes                     | Yes (Costly)               | Yes                 | No                           |
| **Learning Curve**   | **Low**                           | Very Low                     | High                    | High                       | Medium              | Very Low                     |

---

## 🧠 Architectural Philosophy

### 1. The "Component-Driven Optimistic UI" (Azumi)

**Philosophy:** _"The server is the source of truth, but the user shouldn't have to wait for it."_

Azumi introduces a novel architecture where you write components as if they were running on the server, but the compiler analyzes your state mutations (`count += 1`) and automatically generates the minimal JavaScript needed to update the DOM immediately.

-   **Optimistic by Default:** Every interaction feels instant.
-   **Self-Correcting:** The server processes the real action in the background and sends back a "truth" update only if the optimistic prediction was wrong (or to sync complex state).
-   **Zero API Layer:** You don't write JSON endpoints. You call Rust functions.

### 2. The "Hydration" Model (Next.js, Leptos, Dioxus)

**Philosophy:** _"Run the application twice: once on the server for SEO, and again on the client for interactivity."_

-   **The Cost:** You pay double for everything. The browser downloads the HTML _and_ the code to generate that HTML again.
-   **The "Uncanny Valley":** Users see a button, click it, and nothing happens because the JS bundle hasn't finished downloading and executing (hydrating).
-   **State Synchronization:** Keeping the server state and client state in sync requires complex fetching logic (`useEffect`, `useQuery`, `Resources`).

### 3. The "HTML-over-the-wire" Model (HTMX, Phoenix LiveView)

**Philosophy:** _"The client is a dumb terminal. The server does everything."_

-   **Benefits:** extremely simple mental model. No state synchronization issues because there is no client state.
-   **The Latency Problem:** Every click requires a round-trip to the server. If the user has a slow connection, the UI feels sluggish.
-   **No Optimistic UI:** Implementing "instant" feedback (like a like button turning red immediately) requires manual scripting, breaking the clean model.

---

## 🆚 Comparison: Rust Ecosystem

### Azumi vs. Maud / Askama

**Maud** and **Askama** are excellent for generating static HTML. They are fast and type-safe.

-   **The Wall:** You hit a wall the moment you need a dropdown menu, a modal, or a counter.
-   **The Solution:** You have to reach for Alpine.js or vanilla JS, breaking your type safety and developer experience.
-   **Azumi's Edge:** Azumi starts like Maud (writing macros), but scales seamlessly into interactivity without leaving Rust.

### Azumi vs. Leptos / Dioxus / Yew

These frameworks compile Rust to **WebAssembly (WASM)**. They are essentially React for Rust.

-   **The WASM Tax:** Loading a WASM binary is heavy. It's often 300kb-1MB+ of compressed data. This makes initial load times (Time to Interactive) poor on mobile networks.
-   **DOM Access:** WASM cannot touch the DOM directly; it has to go through a JS bridge, which has overhead.
-   **Azumi's Edge:** Azumi keeps the main thread light. It uses standard HTML/CSS for rendering and tiny JS for logic. It loads instantly on a 3G connection.

---

## 🆚 Comparison: JavaScript Ecosystem

### Azumi vs. Next.js (React)

Next.js is the giant of the industry.

-   **Complexity:** Next.js has "Server Components" and "Client Components" with complex serialization boundaries. You have to constantly think about "where is this running?".
-   **Azumi's Edge:** In Azumi, _everything_ is a Server Component, but the interactive parts are automatically projected to the client. The mental model is unified.

### Azumi vs. Svelte

Svelte is Azumi's closest relative. Both use a **compiler** to generate optimized code rather than shipping a runtime.

-   **The Difference:** Svelte compiles to a SPA (Single Page App). Azumi compiles to a Multi-Page App (MPA) with SPA-like superpowers.
-   **Type Safety:** Svelte uses TypeScript, which is great, but Rust's type system is stricter and safer. With Azumi, your backend database types flow directly into your frontend components without any `zod` validation or generated TypeScript definitions.

---

## 📉 Performance Characteristics

### Time to First Byte (TTFB)

-   **Azumi:** ⚡⚡⚡⚡⚡ (Native Rust binary speed)
-   **Next.js:** ⚡⚡⚡ (Node.js runtime overhead)

### Time to Interactive (TTI)

-   **Azumi:** ⚡⚡⚡⚡⚡ (Zero hydration, minimal JS)
-   **WASM:** ⚡⚡ (Must download & compile WASM)
-   **Next.js:** ⚡⚡⚡ (Must hydrate huge bundles)

### Interaction Latency (Click-to-Update)

-   **Azumi:** ⚡⚡⚡⚡⚡ (Optimistic, typically <16ms)
-   **HTMX:** ⚡⚡ (Network latency dependent, 50-200ms)
-   **React:** ⚡⚡⚡⚡ (React Scheduler overhead)

---

## � Conclusion: When to use what?

| Use Case                            | Best Choice            | Why?                                                             |
| :---------------------------------- | :--------------------- | :--------------------------------------------------------------- |
| **E-commerce, Blogs, SaaS**         | **Azumi 🦀**           | Best SEO, fastest load times, great interactivity without bloat. |
| **Complex Dashboard (Data Heavy)**  | **Azumi 🦀**           | Type-safe data flow from DB to UI is unbeatable.                 |
| **Photo/Video Editor (in browser)** | **Leptos / Dioxus 🕸️** | WASM shines for heavy computation on the client.                 |
| **Static Landing Page**             | **Maud 📝**            | Simplest tool for the job (or Azumi for future-proofing).        |
| **Enterprise Standard (Hiring)**    | **Next.js ⚛️**         | Easier to hire React devs (but harder to maintain code).         |

**Azumi drives the "Middle Way":** The speed of static HTML with the soul of an interactive app.
