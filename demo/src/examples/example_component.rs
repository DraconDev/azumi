//! Lesson 1: hello_world.rs
//!
//! Simple template showing basic structure
use azumi::html;

/// Simple hello world with fancy styling
/// Maybe use underscores?
#[azumi::component]
pub fn hello_world() -> impl azumi::Component {
    html! {

        <div class={hello_container}>
        <h1 class={hello_title}>
        "Hello Azumi!"
            </h1>
            <h2 class={hello_title}>
                "Hello World!"
                </h2>
                </div>
                <style>
                    .hello_container {
                        background-color: "#f5f5f5";
                        padding: "20px";
                        border-radius: "8px";
                        box-shadow: "0 2px 4px rgba(0, 0, 0, 0.1)";
                    }
                    .hello_title {
                        color: "#333";
                        font-size: "1.5em";
                        font-weight: "bold";
                    }
                </style>
    }
}

/// Basic template with styling
#[azumi::component]
pub fn basic_template() -> impl azumi::Component {
    html! {

        <div class={basic_container} >
            <h1 class={basic_h1}>
                "Hello, World!"
            </h1>
            <h2 class={basic_h2}>
                "Welcome to Azumi"
            </h2>
            <p class={basic_p}>
                "This is a simple styled template"
            </p>
        </div>
        <style>
            .basic_container {
                background-color: "#f5f5f5";
                padding: "20px";
                border-radius: "8px";
                box-shadow: "0 2px 4px rgba(0, 0, 0, 0.1)";
            }
            .basic_h1 {
                color: "#333";
                font-size: "1.5em";
                font-weight: "bold";
            }
            .basic_h2 {
                color: "#666";
                font-size: "1.2em";
            }
            .basic_p {
                color: "#999";
                font-size: "1em";
            }
        </style>
    }
}

// Handler for Axum
#[allow(dead_code)]
pub async fn lesson1_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&hello_world()))
}
