# Azumi: Technical Analysis & Thoughts

Azumi is a "Compiler-First" full-stack web framework for Rust that fundamentally rethinks the relationship between server-side logic and client-side interactivity.

## 🏗️ Core Architecture: The "Prediction" Engine

The most impressive feat of Azumi is its ability to perform **Static Analysis on Rust mutations**. 
- By parsing `&mut self` methods in `#[azumi::live_impl]`, the framework generates a "mini-DSL" for client-side predictions.
- This turns a standard Rust backend into an **Optimistic UI engine** without the developer having to write a single line of JavaScript for simple state transitions (toggles, increments, etc.).

## 🛡️ Correctness as a First-Class Citizen

Azumi enforces a level of "HTML/CSS Type Safety" that is rare in the industry:
- **CSS-HTML Co-validation:** Catching undefined CSS classes at compile time eliminates a massive category of "silent" UI bugs.
- **Strict Syntax Rules:** The requirement for quoted CSS values, `snake_case` class names, and mandatory bracket syntax for attributes isn't just a stylistic choice—it's what allows the proc-macros to reliably analyze and optimize the code.
- **Signed State (HMAC):** Storing component state in the DOM while cryptographically signing it on the server is a brilliant way to keep the server stateless without sacrificing security.

## ⚡ Performance: The Anti-Hydration Model

Azumi rejects the "Hydration" pattern (where the server renders HTML and the client re-runs the same logic to attach listeners).
- **Static by Default:** If you don't include `azumi.js`, you get zero JavaScript.
- **Micro-Runtime (~5kb):** The client runtime is just a global event delegator and a DOM morphing engine (`Idiomorph`).
- **Instant TTI:** Because there is no heavy JS bundle to parse or "hydrate," the Time to Interactive is virtually identical to the First Contentful Paint.

## 🧠 AI Guidance & Development Workflow

The `AI_GUIDE_FOR_WRITING_AZUMI.md` highlights a very disciplined development workflow:
1. **Quote Everything:** Text content, CSS values, and attribute values must be quoted.
2. **HTML Structure First:** Style blocks must come *after* the HTML structure.
3. **Component Linking:** Live state *must* be explicitly linked to a view component.
4. **Manual Opt-In:** Interactivity requires manual injection of the runtime, forcing developers to be intentional about JS usage.

## 🚀 Final Assessment

Azumi is a "Hardcore" framework that leverages Rust's greatest strengths—macros, type safety, and ownership—to solve the most annoying problems of modern web development (API boilerplate, state sync, and silent CSS failures).

It is not just a library; it's a **language extension** for the web.
