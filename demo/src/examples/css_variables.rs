use azumi::html;

#[azumi::component]
pub fn css_variables_demo() -> impl azumi::Component {
    let percentage = "50%";
    let _color = "yellow";

    html! {
        <div class={progress_bar} --width={percentage}>
            <div class={progress_value}></div>
        </div>
        <div --static-var="100px">
            "Static Var"
        </div>

    }
}

#[allow(dead_code)]
pub async fn css_variables_handler() -> impl axum::response::IntoResponse {
    let component = css_variables_demo();
    axum::response::Html(azumi::render_to_string(&component))
}
