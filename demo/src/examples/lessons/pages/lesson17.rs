use axum::response::IntoResponse;
use azumi::prelude::*;

#[azumi::component]
fn Lesson17() -> Html {
    let container_class = "lesson-container";
    let img_style = "border: 2px solid red; width: 100px;";

    html! {
        <div class={container_class}>
            <h1>"Lesson 17: Asset Pipeline"</h1>
            <p>"This image path should be rewritten to include a hash:"</p>

            // This should be rewritten to /assets/test_logo.<hash>.png
            <img src="/static/test_logo.png" alt="Test Logo" style={img_style} />

            <p>"If you inspect the element, the src should start with /assets/..."</p>
        </div>
    }
}

pub async fn handler() -> impl IntoResponse {
    Lesson17::new().to_html()
}
