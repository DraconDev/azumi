use azumi::html;

#[azumi::component]
pub fn css_variables_demo() -> impl azumi::Component {
    let percentage = "50%";
    let _color = "yellow";

    html! {
        <style>
            .progress_bar {
                background-color: "#e0e0e0";
                border-radius: "4px";
                height: "20px";
                width: "100%";
            }
            .progress_value {
                background-color: "#76c7c0";
                border-radius: "4px";
                height: "100%";
                width: "var(--width)";
                transition: "width 0.5s ease-in-out";
            }
        </style>
        <div class={progress_bar} --width={percentage}>
            <div class={progress_value}></div>
        </div>

        <div --static-var="100px">
            "Static Var"
        </div>
    }
}

pub async fn css_variables_handler() -> axum::response::Html<String> {
    let component = css_variables_demo();
    axum::response::Html(azumi::render_to_string(&component))
}
