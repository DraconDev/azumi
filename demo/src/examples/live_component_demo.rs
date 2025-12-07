use azumi::prelude::*;

#[azumi::live]
pub struct UnifiedCounter {
    pub count: i32,
    pub active: bool,
}

#[azumi::live_impl]
impl UnifiedCounter {
    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn toggle(&mut self) {
        self.active = !self.active;
    }
}

// Unified component macro auto-detects live state!
#[azumi::component]
pub fn unified_counter_view<'a>(state: &'a UnifiedCounter) -> impl Component + 'a {
    html! {
        <div class={counter_box}>
            <h3>"Unified Component Demo"</h3>
            <p>"Count: " <span data-bind="count">{state.count}</span></p>
            <p>"Active: " <span data-bind="active">{state.active}</span></p>

            // New syntax: on:click={state.method}
            // Auto-generates az-on and data-predict!
            <button on:click={state.increment}>
                "Increment (+1)"
            </button>

            <button on:click={state.toggle}>
                "Toggle Active"
            </button>
            <style>
                .counter_box {
                    border: "1px solid #ccc";
                    padding: "1em";
                    border-radius: "8px";
                }
            </style>
        </div>
    }
}

pub async fn unified_demo_handler() -> axum::response::Html<String> {
    let state = UnifiedCounter {
        count: 0,
        active: true,
    };

    // Use the component module pattern for components with props
    use unified_counter_view_component::*;
    let component_html = azumi::render_to_string(&render(
        Props::builder().state(&state).build().expect("props"),
    ));

    // Wrap with full HTML document including azumi.js
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Azumi Live Demo</title>
    <style>
        body {{ font-family: system-ui, sans-serif; padding: 2rem; }}
    </style>
</head>
<body>
    {}
    <script src="/static/idiomorph.js"></script>
    <script src="/static/azumi.js"></script>
</body>
</html>"#,
        component_html
    );

    axum::response::Html(html)
}

azumi::inventory::submit! {
    azumi::action::ActionEntry {
        path: "/unified_demo",
        handler: unified_demo_router,
    }
}

#[allow(non_snake_case)]
pub fn unified_demo_router() -> axum::routing::MethodRouter<()> {
    axum::routing::get(unified_demo_handler)
}
