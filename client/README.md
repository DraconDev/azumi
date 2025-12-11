# Azumi Client (azumi.js)

The `azumi.js` client is a lightweight, framework-agnostic runtime that powers Azumi's interactivity. It provides declarative event delegation, optimistic UI updates, and intelligent DOM morphing without requiring a heavy frontend framework.

While designed to work seamlessly with `azumi-rs`, it can be used independently with any backend that follows the Azumi protocol.

## Features

-   **Event Delegation**: Declarative `az-on` attributes for click, submit, change, and input events.
-   **Optimistic UI**: Instant state updates via `data-predict` with zero network latency.
-   **DOM Morphing**: Uses [Idiomorph](https://github.com/bigskysoftware/idiomorph) for smooth DOM transitions, preserving focus and input state.
-   **Hot Reload**: Built-in WebSocket connection for instant development feedback.
-   **Micro-State Management**: embed JSON state directly in the DOM with `az-scope`.

## Installation

The client expects `Idiomorph` to be available globally (optional but recommended for morphing).

```html
<!-- 1. Include Idiomorph (recommended) -->
<script src="https://unpkg.com/idiomorph"></script>

<!-- 2. Include Azumi Client -->
<!-- You can use the raw file or the minified version from the release -->
<script src="/path/to/azumi.js"></script>
```

The client initializes automatically as `window.azumi`.

## Core Concepts

### 1. Scopes (`az-scope`)

Components store their state as a JSON string in the `az-scope` attribute. This serves as the single source of truth for the client.

```html
<div id="counter" az-scope='{"count": 0}'>
    <span data-bind="count">0</span>
    ...
</div>
```

-   **`data-bind="field"`**: Automatically updates text content when the scope state changes (optimistically or via local set).

### 2. Events (`az-on`)

Azumi uses a declarative syntax for event handling.

**Syntax**: `az-on="{trigger} {command} {args...}"`

| Command  | Syntax                        | Description                                                      |
| :------- | :---------------------------- | :--------------------------------------------------------------- |
| **Call** | `call {action} -> {selector}` | Calls a server endpoint and morphs the response into the target. |
| **Set**  | `set {field} = {value}`       | Updates local state immediately without a server roundtrip.      |

**Examples**:

```html
<!-- Server Action -->
<button az-on="click call increment -> #counter">Increment</button>

<!-- Local State Set -->
<button az-on="click set isOpen = true">Open Modal</button>

<!-- Form Submission -->
<form az-on="submit call login -> #auth-box">...</form>
```

### 3. Optimistic UI (`data-predict`)

Predict the outcome of a server action to update the UI immediately (0ms latency). Azumi captures the state, applies the prediction, and rolls it back if the server request fails.

**Syntax**: `data-predict="{field} {operation} {value?}"`

| Operation     | Syntax              | Example             |
| :------------ | :------------------ | :------------------ |
| **Toggle**    | `!{field}`          | `!liked`            |
| **Increment** | `{field} + {n}`     | `likes + 1`         |
| **Decrement** | `{field} - {n}`     | `likes - 1`         |
| **Assign**    | `{field} = {value}` | `status = "active"` |

**Example**:

```html
<button
    az-on="click call toggle_like -> #card"
    data-predict="!liked; likes + 1"
>
    Like
</button>
```

### 4. Server Protocol

If you are using `azumi.js` without `azumi-rs`, your server must implement the following:

1.  **Endpoint**: `POST /_azumi/action/{action_name}`
2.  **Request Body**:
    -   **Forms**: `JSON` object of form fields.
    -   **Others**: `JSON` object of the current `az-scope` state.
3.  **Response**:
    -   **Success (200)**: HTML fragment to swap into the target.
    -   **Error (4xx/5xx)**: Client rolls back optimistic updates.

### 5. Hot Reload

The client automatically attempts to connect to `ws://{host}/_azumi/live_reload`.
When the connection is lost (server restart), it polls the current page via `HEAD` requests and refreshes when the server is back up.

## JavaScript API

While declarative attributes cover 90% of use cases, you can access the runtime via `window.azumi`.

```javascript
// Manually refresh an element
window.azumi.execute(
    {
        type: "call",
        actionName: "refresh",
        url: "/_azumi/action/refresh",
        target: "#my-component",
    },
    elementReference
);

// Update local state programmatically
window.azumi.setState(
    {
        field: "count",
        value: 10,
    },
    elementWithinScope
);
```
