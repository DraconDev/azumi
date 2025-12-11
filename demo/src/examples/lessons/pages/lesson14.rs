use azumi::prelude::*;

/// Lesson 14: Composition with Live Components
///
/// Building complex UIs by composing live components

#[azumi::live]
pub struct TabState {
    pub active_index: i32,
}

#[azumi::live_impl(component = "tabs_view")]
impl TabState {
    pub fn select_0(&mut self) {
        self.active_index = 0;
    }
    pub fn select_1(&mut self) {
        self.active_index = 1;
    }
    pub fn select_2(&mut self) {
        self.active_index = 2;
    }
}

/// Tabs component
#[azumi::component]
pub fn tabs_view<'a>(state: &'a TabState) -> impl Component + 'a {
    html! {

        <div class={tabs_container}>
            <div class={tab_buttons}>
                <button
                    class={if state.active_index == 0 { format!("{} {}", tab_btn, tab_btn_active) } else { tab_btn.to_string() }}
                    on:click={state.select_0}>
                    "🏠 Overview"
                </button>
                <button
                    class={if state.active_index == 1 { format!("{} {}", tab_btn, tab_btn_active) } else { tab_btn.to_string() }}
                    on:click={state.select_1}>
                    "📊 Features"
                </button>
                <button
                    class={if state.active_index == 2 { format!("{} {}", tab_btn, tab_btn_active) } else { tab_btn.to_string() }}
                    on:click={state.select_2}>
                    "💡 Examples"
                </button>
            </div>
            <div class={tab_content}>
                @if state.active_index == 0 {
                    <div class={tab_panel}>
                        <h3>"Overview"</h3>
                        <p>"Azumi Live allows you to build interactive components with zero JavaScript. The compiler analyzes your Rust code and generates optimistic predictions."</p>
                    </div>
                }
                @if state.active_index == 1 {
                    <div class={tab_panel}>
                        <h3>"Features"</h3>
                        <ul>
                            <li>"Compiler-driven optimistic UI"</li>
                            <li>"Type-safe state management"</li>
                            <li>"Zero client-side JavaScript needed"</li>
                            <li>"Automatic DOM reconciliation"</li>
                        </ul>
                    </div>
                }
                @if state.active_index == 2 {
                    <div class={tab_panel}>
                        <h3>"Examples"</h3>
                        <p>"Counters, Like buttons, Forms, Tabs, Accordions - all built with the same pattern!"</p>
                    </div>
                }
            </div>
        </div>
        <style>
            .tabs_container {
                background: "white";
                border-radius: "12px";
                border: "1px solid #e0e0e0";
                overflow: "hidden";
            }
            .tab_buttons {
                display: "flex";
                background: "#f5f5f5";
                border-bottom: "1px solid #e0e0e0";
            }
            .tab_btn {
                flex: "1";
                padding: "1rem";
                border: "none";
                background: "transparent";
                cursor: "pointer";
                font-size: "1rem";
                transition: "background 0.2s";
            }
            .tab_btn_active {
                background: "white";
                color: "#2196f3";
                font-weight: "bold";
                border-bottom: "2px solid #2196f3";
            }
            .tab_content {
                padding: "1.5rem";
                min-height: "150px";
            }
            .tab_panel {
                animation: "fadeIn 0.3s ease";
            }
        </style>
    }
}

// Handler for Axum
pub async fn lesson14_handler() -> axum::response::Html<String> {
    let tab_state = TabState { active_index: 0 };

    use tabs_view_component::Props;
    let tabs_html = azumi::render_to_string(&tabs_view_component::render(
        Props::builder().state(&tab_state).build().expect("props"),
    ));

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Lesson 14: Component Composition</title>
    <style>
        body {{ 
            font-family: system-ui, sans-serif; 
            margin: 0;
            padding: 2rem;
            background: #fafafa;
        }}
        .container {{ max-width: 800px; margin: 0 auto; }}
        .header {{ text-align: center; margin-bottom: 2rem; }}
        .main_title {{ font-size: 2rem; color: #333; }}
        .subtitle {{ color: #666; }}
        .demo_area {{ margin: 2rem 0; }}
        .explanation {{
            background: #fff3e0;
            padding: 1.5rem;
            border-radius: 8px;
            margin: 2rem 0;
        }}
        .tab_btn {{ padding: 1rem; border: none; background: transparent; cursor: pointer; font-size: 1rem; flex: 1; }}
        .tab_btn_active {{ background: white; color: #2196f3; font-weight: bold; border-bottom: 2px solid #2196f3; }}
        @keyframes fadeIn {{ from {{ opacity: 0; }} to {{ opacity: 1; }} }}
    </style>
</head>
<body>
    <div class="container">
        <header class="header">
            <h1 class="main_title">Lesson 14: Component Composition</h1>
            <p class="subtitle">Building complex UIs with live components</p>
        </header>
        
        <div class="explanation">
            <h3>🧩 Composition Pattern</h3>
            <p>Each tab switch is a separate action. The compiler generates predictions for each:
            <code>select_0</code> → <code>active_index = 0</code></p>
        </div>
        
        <div class="demo_area">
            {}
        </div>
    </div>
    <script src="/static/idiomorph.js"></script>
    <script src="/static/azumi.js"></script>
</body>
</html>"#,
        tabs_html
    );

    axum::response::Html(html)
}
