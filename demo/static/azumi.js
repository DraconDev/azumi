/**
 * Azumi Live - Client Runtime
 *
 * Features:
 * - Event delegation for az-on attributes
 * - DOM morphing via Idiomorph
 * - Optimistic UI via data-predict attributes (Azumi Live)
 */
class Azumi {
    constructor() {
        this.scopes = new WeakMap(); // Element -> state cache
        this.delegate();
        this.connectHotReload();
    }

    // Hot Reload Logic
    connectHotReload() {
        const protocol = window.location.protocol === "https:" ? "wss:" : "ws:";
        const wsUrl = `${protocol}//${window.location.host}/_azumi/live_reload`;

        try {
            const ws = new WebSocket(wsUrl);
            let connected = false;

            ws.onopen = () => {
                connected = true;
                console.log("🔥 Hot Reload: Connected");
            };

            ws.onclose = () => {
                if (connected) {
                    console.log(
                        "🔥 Hot Reload: Connection lost, polling for restart..."
                    );
                    this.pollForReload();
                }
            };
        } catch (e) {
            // Hot reload likely not enabled on server
        }
    }

    pollForReload() {
        const interval = setInterval(() => {
            fetch(window.location.href, { method: "HEAD" })
                .then((res) => {
                    if (res.ok) {
                        clearInterval(interval);
                        window.location.reload();
                    }
                })
                .catch(() => {
                    /* keep polling */
                });
        }, 200);
    }

    // Event delegation
    delegate() {
        ["click", "submit", "change", "input"].forEach((event) => {
            document.addEventListener(event, (e) => this.handleEvent(e));
        });
    }

    // Parse az-on attribute
    handleEvent(e) {
        const target = e.target.closest(`[az-on]`);
        if (!target) return;

        // Check if the event type matches the trigger (e.g. "click ...")
        // Simple parsing: "click call foo" or "submit call bar"
        const attr = target.getAttribute("az-on");
        const parts = attr.split(" ");
        const trigger = parts[0];

        if (trigger !== e.type) return;

        e.preventDefault(); // Default prevent for handled events

        // Parse the rest: "call toggle_like -> #box"
        // This is a very basic parser for the prototype
        const action = this.parseAction(parts.slice(1).join(" "), target);
        if (action) this.execute(action, target);
    }

    parseAction(cmd, element) {
        // Format: "{event} call {action} -> {target} {swap}"
        // or "{event} set {key} = {value}"
        // NOTE: TokenStream adds spaces around punctuation, so "-> #id" becomes "- > # id"

        // Remove extra spaces and reconstruct operators
        // Ensure arrow has spaces around it to be a separate token
        cmd = cmd.replace(/\s*-\s*>\s*/g, " -> ");
        // Ensure ID selector has no internal spaces (e.g. "# myid" -> "#myid")
        cmd = cmd.replace(/#\s+/g, "#");

        const tokens = cmd.split(" ").filter((t) => t.trim() !== "");
        const actionType = tokens[0]; // "call" or "set"

        if (actionType === "call") {
            let actionName = tokens[1]; // The actual action function name
            let url = `/_azumi/action/${actionName}`;
            let targetSelector = null;
            let swap = "morph";

            const arrowIndex = tokens.indexOf("->");
            if (arrowIndex !== -1) {
                targetSelector = tokens[arrowIndex + 1];
                if (tokens[arrowIndex + 2]) {
                    swap = tokens[arrowIndex + 2];
                }
            }

            return {
                type: "call",
                actionName,
                url,
                target: targetSelector,
                swap,
            };
        }

        // Parse 'set' for local state
        if (actionType === "set") {
            // Format: "set field = value"
            // tokens: ["set", "field", "=", "value"]
            const field = tokens[1];
            const value = tokens.slice(3).join(" "); // everything after "="

            return {
                type: "set",
                field,
                value,
            };
        }

        return null;
    }

    // Execute: "call toggle_like -> #box" or "set open = true"
    async execute(action, element) {
        if (action.type === "call") {
            await this.callAction(action, element);
        } else if (action.type === "set") {
            this.setState(action, element);
        }
    }

    /**
     * Azumi Live: Execute optimistic prediction
     *
     * Prediction DSL format: "field = expression"
     * Expressions:
     *   - "!field" -> toggle boolean
     *   - "field + value" -> increment
     *   - "field - value" -> decrement
     *   - literal -> direct assignment
     *
     * Multiple predictions separated by ";"
     */
    executePrediction(scopeElement, prediction) {
        if (!prediction || !scopeElement) return null;

        const scopeAttr = scopeElement.getAttribute("az-scope");
        if (!scopeAttr) return null;

        try {
            const state = JSON.parse(scopeAttr);
            const originalState = JSON.parse(scopeAttr); // Keep copy for rollback

            // Parse multiple predictions separated by ;
            const predictions = prediction
                .split(";")
                .map((p) => p.trim())
                .filter((p) => p);

            for (const pred of predictions) {
                this.applyPrediction(state, pred);
            }

            // Update the scope attribute with new state
            scopeElement.setAttribute("az-scope", JSON.stringify(state));

            // Update any bound elements
            this.updateBindings(scopeElement, state);

            console.log("🚀 Prediction executed:", prediction, state);

            return { originalState, newState: state };
        } catch (err) {
            console.warn("Prediction execution failed:", err);
            return null;
        }
    }

    /**
     * Apply a single prediction to state
     * Format: "field = expression"
     */
    applyPrediction(state, pred) {
        // Parse: "field = expr"
        const match = pred.match(/^(\w+)\s*=\s*(.+)$/);
        if (!match) return;

        const [, field, expr] = match;
        const trimmedExpr = expr.trim();

        // Toggle: "!field"
        if (trimmedExpr.startsWith("!")) {
            const toggleField = trimmedExpr.slice(1).trim();
            if (toggleField === field) {
                state[field] = !state[field];
                return;
            }
        }

        // Increment: "field + value"
        const addMatch = trimmedExpr.match(/^(\w+)\s*\+\s*(\d+)$/);
        if (addMatch && addMatch[1] === field) {
            state[field] = (state[field] || 0) + parseInt(addMatch[2], 10);
            return;
        }

        // Decrement: "field - value"
        const subMatch = trimmedExpr.match(/^(\w+)\s*-\s*(\d+)$/);
        if (subMatch && subMatch[1] === field) {
            state[field] = (state[field] || 0) - parseInt(subMatch[2], 10);
            return;
        }

        // Literal assignment
        if (trimmedExpr === "true") {
            state[field] = true;
        } else if (trimmedExpr === "false") {
            state[field] = false;
        } else if (/^-?\d+$/.test(trimmedExpr)) {
            state[field] = parseInt(trimmedExpr, 10);
        } else if (/^-?\d+\.\d+$/.test(trimmedExpr)) {
            state[field] = parseFloat(trimmedExpr);
        } else if (trimmedExpr.startsWith('"') && trimmedExpr.endsWith('"')) {
            state[field] = trimmedExpr.slice(1, -1);
        } else {
            // Fallback: treat as string
            state[field] = trimmedExpr;
        }
    }

    /**
     * Update DOM elements that display state values
     * Looks for elements with data-bind="fieldName" attribute
     */
    updateBindings(scopeElement, state) {
        // Find all elements with data-bind within the scope
        const bindings = scopeElement.querySelectorAll("[data-bind]");
        bindings.forEach((el) => {
            const field = el.getAttribute("data-bind");
            if (field && state.hasOwnProperty(field)) {
                el.textContent = state[field];
            }
        });

        // Also update text content that might be interpolated
        // This is a simple approach - for complex cases, re-render from server
    }

    /**
     * Rollback prediction if server response differs
     */
    rollbackPrediction(scopeElement, originalState) {
        if (!scopeElement || !originalState) return;
        scopeElement.setAttribute("az-scope", JSON.stringify(originalState));
        this.updateBindings(scopeElement, originalState);
        console.log("⏪ Prediction rolled back");
    }

    // Server action with optimistic prediction
    async callAction(action, element) {
        // Find scope element
        const scopeElement = element.closest("[az-scope]");

        // IMPORTANT: Capture original state BEFORE prediction
        // We send original state to server, not predicted state
        let body = null;
        if (element.tagName === "FORM") {
            body = new FormData(element);
            const data = Object.fromEntries(body.entries());
            body = JSON.stringify(data);
        } else {
            if (scopeElement) {
                let scopeData = scopeElement.getAttribute("az-scope");
                body = scopeData || "{}";
            } else {
                body = "{}";
            }
        }

        // Check for prediction attribute (Azumi Live)
        const prediction = element.getAttribute("data-predict");
        let predictionResult = null;

        if (prediction && scopeElement) {
            // Execute prediction AFTER capturing state (0ms latency!)
            predictionResult = this.executePrediction(scopeElement, prediction);
        }

        try {
            const res = await fetch(action.url, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body,
            });

            if (!res.ok) throw new Error(`Action failed: ${res.status}`);

            const html = await res.text();

            // OPTIMIZATION: Check if server state matches prediction
            // If prediction was correct, skip morphing to prevent flicker
            /* 
            // DISABLED: This optimization prevents structural updates (e.g. @if blocks) from rendering
            if (predictionResult && scopeElement) {
                // ... (omitted for brevity)
            }
            */

            // FIXED: Default target to scopeElement (component root), then element
            let target = scopeElement || element;
            if (action.target) {
                target = document.querySelector(action.target);
            }

            if (target && window.Idiomorph) {
                // Morph will reconcile prediction with server truth
                // Use outerHTML to replace component wrapper
                window.Idiomorph.morph(target, html, {
                    morphStyle: "outerHTML",
                });
            } else if (target) {
                console.warn(
                    "Idiomorph not loaded, falling back to outerHTML replacement"
                );
                target.outerHTML = html;
            }
        } catch (err) {
            console.error("Azumi action error:", err);
            // Rollback prediction on error
            if (predictionResult) {
                this.rollbackPrediction(
                    scopeElement,
                    predictionResult.originalState
                );
            }
        }
    }

    // Local state change (no server roundtrip)
    setState(action, element) {
        const scopeElement = element.closest("[az-scope]");
        if (!scopeElement) {
            console.warn("setState: No az-scope found");
            return;
        }

        const scopeAttr = scopeElement.getAttribute("az-scope");
        if (!scopeAttr) return;

        try {
            const state = JSON.parse(scopeAttr);

            // Apply the prediction DSL (reuse existing logic)
            const prediction = `${action.field} = ${action.value}`;
            this.applyPrediction(state, prediction);

            // Update the scope attribute
            scopeElement.setAttribute("az-scope", JSON.stringify(state));

            // Update bound elements
            this.updateBindings(scopeElement, state);

            console.log(
                "🎯 Client set:",
                action.field,
                "=",
                action.value,
                state
            );
        } catch (err) {
            console.warn("setState failed:", err);
        }
    }
}

// Initialize
window.azumi = new Azumi();
console.log("Azumi Live Client Initialized 🚀");
