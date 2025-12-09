use axum::response::{Html, IntoResponse};
use azumi::prelude::*;

#[azumi::component]
pub fn lesson17() -> impl Component {
    html! {
        <style>
            .lesson_container {
                border: "1px solid black";
                padding: "20px";
            }
            .logo_image {
                border: "2px solid red";
                width: "100px";
            }
        </style>
        <div class={lesson_container}>
            <h1>"Lesson 17: Asset Pipeline"</h1>
            <p>"This image path should be rewritten to include a hash:"</p>

            // This should be rewritten to /assets/test_logo.<hash>.png
            <img src="/static/test_logo.png" alt="Test Logo" class={logo_image} />

            <p>"If you inspect the element, the src should start with /assets/..."</p>
        </div>
    }
}

pub async fn handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson17()))
}
