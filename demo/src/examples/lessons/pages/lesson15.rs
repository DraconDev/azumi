use azumi::prelude::*;

/// Lesson 15: Full Application Pattern
///
/// Building a complete interactive todo app

#[azumi::live]
pub struct TodoApp {
    pub show_completed: bool,
    pub item_count: i32,
}

#[azumi::live_impl(component = "todo_app_view")]
impl TodoApp {
    pub fn toggle_filter(&mut self) {
        self.show_completed = !self.show_completed;
    }

    pub fn add_item(&mut self) {
        self.item_count += 1;
    }

    pub fn clear(&mut self) {
        self.item_count = 0;
    }
}

/// Todo app component
#[azumi::component]
pub fn todo_app_view<'a>(state: &'a TodoApp) -> impl Component + 'a {
    html! {

        <div class={todo_app}>
            <header class={app_header}>
                <h1 class={app_title}>"📝 Azumi Todos"</h1>
                <p class={app_subtitle}>"Built with Azumi Live"</p>
            </header>

            <div class={input_section}>
                <input class={todo_input} placeholder="What needs to be done?" />
                <button class={add_btn} on:click={state.add_item}>"Add"</button>
            </div>

            <div class={filter_section}>
                <div>
                    <button
                        class={if !state.show_completed { "filter_btn filter_active" } else { "filter_btn" }}
                        on:click={state.toggle_filter}>
                        "Active"
                    </button>
                    <button
                        class={if state.show_completed { "filter_btn filter_active" } else { "filter_btn" }}
                        on:click={state.toggle_filter}>
                        "Completed"
                    </button>
                </div>
                <span class={item_count} data-bind="item_count">{state.item_count}</span>
                <button class={clear_btn} on:click={state.clear}>"Clear"</button>
            </div>

            <div class={todo_list}>
                @if state.item_count == 0 {
                    <div class={empty_state}>
                        "🎉 No todos! Add one above."
                    </div>
                }
                @if state.item_count > 0 {
                    <p>"You have " {state.item_count} " item(s) in your list."</p>
                    <div style={ --bg-color: "#e0f7fa"; --padding: "1rem" }>
                        "This box is styled with the new Style DSL!"
                    </div>
                }
            </div>
        </div>
        <style>
            .todo_app {
                max-width: "500px";
                background: "white";
                border-radius: "12px";
                border: "1px solid #e0e0e0";
                overflow: "hidden";
            }
            .app_header {
                background: "linear-gradient(135deg,#667eea 0%,#764ba2 100%)";
                color: "white";
                padding: "2rem";
                text-align: "center";
            }
            .app_title {
                margin: "0";
                font-size: "2rem";
            }
            .app_subtitle {
                opacity: "0.8";
                margin-top: "0.5rem";
            }
            .input_section {
                padding: "1rem";
                display: "flex";
                gap: "0.5rem";
                border-bottom: "1px solid #eee";
            }
            .todo_input {
                flex: "1";
                padding: "0.75rem";
                border: "1px solid #ddd";
                border-radius: "6px";
                font-size: "1rem";
            }
            .add_btn {
                padding: "0.75rem 1.5rem";
                background: "#4caf50";
                color: "white";
                border: "none";
                border-radius: "6px";
                cursor: "pointer";
                font-size: "1rem";
            }
            .filter_section {
                padding: "1rem";
                display: "flex";
                justify-content: "space-between";
                align-items: "center";
                background: "#f8f9fa";
            }
            .filter_btn {
                padding: "0.5rem 1rem";
                border: "1px solid #ddd";
                border-radius: "4px";
                background: "white";
                cursor: "pointer";
            }
            .filter_active {
                background: "#2196f3";
                color: "white";
                border-color: "#2196f3";
            }
            .item_count {
                font-size: "1.5rem";
                font-weight: "bold";
                color: "#667eea";
            }
            .clear_btn {
                padding: "0.5rem 1rem";
                background: "#f44336";
                color: "white";
                border: "none";
                border-radius: "4px";
                cursor: "pointer";
            }
            .todo_list {
                padding: "1rem";
            }
            .empty_state {
                text-align: "center";
                padding: "2rem";
                color: "#999";
            }
        </style>
    }
}

/// Full page component for Lesson 15
#[azumi::component]
pub fn lesson15_page<'a>(state: &'a TodoApp) -> impl Component + 'a {
    html! {
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="utf-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1" />
            <title>"Lesson 15: Full Application"</title>
            <style>
                body {
                    font-family: "system-ui, sans-serif";
                    margin: "0";
                    padding: "2rem";
                    background: "#fafafa";
                }
                .container { max-width: "800px"; margin: "0 auto"; }
                .header { text-align: "center"; margin-bottom: "2rem"; }
                .main_title { font-size: "2rem"; color: "#333"; }
                .subtitle { color: "#666"; }
                .demo_area { display: "flex"; justify-content: "center"; margin: "2rem 0"; }
                .explanation {
                    background: "#e8f5e9";
                    padding: "1.5rem";
                    border-radius: "8px";
                    margin: "2rem 0";
                }
                .filter_btn_g { padding: "0.5rem 1rem"; border: "1px solid #ddd"; border-radius: "4px"; background: "white"; cursor: "pointer"; }
                .filter_active_g { background: "#2196f3"; color: "white"; border-color: "#2196f3"; }
            </style>
        </head>
        <body>
            <div class={container}>
                <header class={header}>
                    <h1 class={main_title}>"Lesson 15: Full Application"</h1>
                    <p class={subtitle}>"Building a complete interactive todo app"</p>
                </header>

                <div class={explanation}>
                    <h3>"🚀 Putting It All Together"</h3>
                    <ul>
                        <li><strong>"Multiple actions"</strong>" - add, toggle filter, clear"</li>
                        <li><strong>"Conditional rendering"</strong>" - empty state vs items"</li>
                        <li><strong>"Optimistic updates"</strong>" - instant count changes"</li>
                    </ul>
                </div>

                <div class={demo_area}>
                    @todo_app_view(state = state)
                </div>
            </div>
            // Scripts injected automatically
        </body>
        </html>
    }
}

// Handler for Axum
pub async fn lesson15_handler() -> impl axum::response::IntoResponse {
    let app_state = TodoApp {
        show_completed: false,
        item_count: 0,
    };
    use lesson15_page_component::Props;
    let page =
        lesson15_page_component::render(Props::builder().state(&app_state).build().expect("props"));
    axum::response::Html(azumi::render_to_string(&page))
}
