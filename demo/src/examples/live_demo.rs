//! Azumi Live Demo - Compiler-Driven Optimistic UI
//!
//! This example demonstrates the new #[azumi::live] and #[azumi::live_impl] macros
//! that automatically analyze mutations and generate prediction metadata.

use azumi::prelude::*;

// The #[azumi::live] macro:
// - Adds Serialize/Deserialize derives automatically
// - Generates to_scope() helper method
#[azumi::live]
pub struct LiveCounter {
    pub count: i32,
    pub active: bool,
}

// The #[azumi::live_impl] macro:
// - Analyzes each method for predictable mutations
// - Generates prediction DSL automatically
// - Auto-registers Axum action handlers
#[azumi::live_impl]
impl LiveCounter {
    /// Toggle the active state
    /// Prediction: "active = !active"
    pub fn toggle(&mut self) {
        self.active = !self.active;
    }

    /// Set to inactive
    /// Prediction: "active = false"
    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

/// Demo view component
#[allow(dead_code)]
pub fn live_counter_view(state: &LiveCounter) -> impl azumi::Component + '_ {
    html! {
        <div id={live_demo} class={live_container} az-scope={state.to_scope()}>
        <h2 class={live_title}>"Azumi Live Demo"</h2>
        <div class={live_count}>{state.count}</div>
            <div class={live_status}>
                "Status: "
                {if state.active { "Active ✅" } else { "Inactive ❌" }}
                </div>
                <div>
                <button class={live_button toggle_btn}
                az-on={click call toggle -> #live_demo}
                data-predict="active = !active">
                "Toggle Status"
                </button>
                <button class={live_button deactivate_btn}
                az-on={click call deactivate -> #live_demo}
                data-predict="active = false">
                "Deactivate"
                </button>
                </div>
                <p class={live_status}>
                "🚀 This demo uses #[azumi::live] for automatic mutation analysis!"
                </p>
                </div>
        <style>
            .live_container { padding: "2rem"; text-align: "center"; border: "2px solid #4caf50"; border-radius: "8px"; }
            .live_title { font-size: "1.5rem"; color: "#333"; margin-bottom: "1rem"; }
            .live_count { font-size: "3rem"; font-weight: "bold"; color: "#4caf50"; }
            .live_status { font-size: "1rem"; color: "#666"; margin: "1rem 0"; }
            .live_button { padding: "0.75rem 1.5rem"; margin: "0.5rem"; border: "none"; border-radius: "4px"; cursor: "pointer"; font-size: "1rem"; }
            .toggle_btn { background: "#2196f3"; color: "white"; }
            .deactivate_btn { background: "#ff5722"; color: "white"; }
            #live_demo { display: "block"; }
        </style>
    }
}

/// Main lesson component
#[azumi::component]
pub fn live_demo() -> impl azumi::Component {
    let initial_state = LiveCounter {
        count: 42,
        active: true,
    };

    html! {
        <script src="/static/azumi.js"></script>

        <div class={page_container}>
            <h1 class={page_title}>"Azumi Live: Compiler-Driven Optimistic UI"</h1>
            <p class={page_intro}>
                "Using #[azumi::live] to automatically generate predictions from Rust mutations."
            </p>

            { live_counter_view(&initial_state) }

            <div class={code_block}>
                <pre>
                    "#[azumi::live]\n"
                    "pub struct LiveCounter {\n"
                    "    pub count: i32,\n"
                    "    pub active: bool,\n"
                    "}\n\n"
                    "#[azumi::live_impl]\n"
                    "impl LiveCounter {\n"
                    "    pub fn toggle(&mut self) {\n"
                    "        self.active = !self.active;  // → Prediction: \"active = !active\"\n"
                    "    }\n"
                    "}"
                </pre>
            </div>
        </div>
        <style>
            .page_container { max-width: "800px"; margin: "0 auto"; padding: "2rem"; }
            .page_title { font-size: "2rem"; color: "#333"; text-align: "center"; }
            .page_intro { color: "#666"; text-align: "center"; margin-bottom: "2rem"; }
            .code_block { background: "#f5f5f5"; padding: "1rem"; border-radius: "4px"; font-family: "monospace"; overflow-x: "auto"; }
        </style>
    }
}

#[allow(dead_code)]
pub async fn live_demo_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&live_demo()))
}
