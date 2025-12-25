use azumi::html;

#[azumi::component]
pub fn style_at_end() -> impl azumi::Component {
    html! {
        <div class={container}>
            <h1 class={title}>"Hello from Style at End"</h1>
        </div>

        <style>
            .container {
                padding: "20px";
                background: "#f0f0f0";
            }
            .title {
                color: "blue";
                font-size: "24px";
            }
        </style>
    }
}
