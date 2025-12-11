use azumi::html;

#[azumi::component]
pub fn test_global_styles() -> impl azumi::Component {
    html! {
        <div class={scoped_box}>
            <h1>"Global + Scoped Styles Test"</h1>
            <p>"Global styles apply to body and * (unscoped)"</p>
            <p>"Scoped styles apply only to this component"</p>
        </div>

        <style global>
            body {
                margin: "0";
                padding: "0";
                font-family: "Arial, sans-serif";
            }

            * {
                box-sizing: "border-box";
            }
        </style>

        <style>
            .scoped_box {
                padding: "20px";
                background: "#f0f0f0";
                border: "1px solid #ccc";
            }
        </style>

    }
}

pub async fn handler() -> axum::response::Html<String> {
    axum::response::Html(azumi::render_to_string(&test_global_styles()))
}
