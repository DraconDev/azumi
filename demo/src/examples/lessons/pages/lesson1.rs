use azumi::html;

/// Lesson 2: CSS Scoping & Validation Fundamentals
///
/// Automatic CSS scoping demonstration
#[azumi::component]
pub fn scoped_component() -> impl azumi::Component {
    html! {
        <div class={container}>
            <h1 class={title}>"Automatically Scoped CSS"</h1>
            <p>"This CSS is scoped to this component only"</p>
        </div>
        <style>
            .container { padding: "1rem"; border: "1px solid #ddd"; }
            .title { color: "#2196f3"; }
        </style>
    }
}

/// Example: Multiple components with same class names
#[azumi::component]
pub fn multiple_scoped_components() -> impl azumi::Component {
    html! {
        <div>
            <div class={card}>
                <h3 class={card_title}>"First Component"</h3>
                <p>"This uses the same class names as the second component"</p>
            </div>
            <div class={card}>
                <h3 class={card_title}>"Second Component"</h3>
                <p>"But the CSS is automatically scoped, so no conflicts!"</p>
            </div>
        </div>
        <style>
            .card { padding: "1rem"; margin: "0.5rem"; border: "1px solid #eee"; }
            .card_title { font-weight: "bold"; color: "#1976d2"; }
        </style>
    }
}

/// Example: CSS validation - valid styles
#[azumi::component]
pub fn valid_css_example() -> impl azumi::Component {
    html! {
        <div class={valid_container}>
            <h2 class={valid_title}>"Valid CSS Example"</h2>
            <p class={valid_text}>"This CSS follows Azumi's validation rules"</p>
            <ul>
                <li>"Proper property values"</li>
                <li>"Valid color formats"</li>
                <li>"Correct unit usage"</li>
            </ul>
        </div>
        <style>
            .valid_container { padding: "20px"; background: "#f9f9f9"; }
            .valid_title { color: "#2196f3"; font-size: "1.2rem"; }
            .valid_text { color: "#666"; }
        </style>
    }
}

/// Main lesson demonstration component
#[azumi::component]
pub fn lesson1() -> impl azumi::Component {
    html! {
        <div class={container}>
            <header class={header}>
                <h1 class={main_title}>"Lesson 2: CSS Scoping & Validation Fundamentals"</h1>
                <p class={subtitle}>"Automatic CSS scoping and validation rules"</p>
            </header>

            <section class={key_points}>
                <h2 class={section_title}>"Key Concepts"</h2>
                <ul class={points_list}>
                    <li class={point}>"✅ CSS is automatically scoped to each component"</li>
                    <li class={point}>"✅ No manual CSS management needed"</li>
                    <li class={point}>"✅ Prevents CSS conflicts between components"</li>
                    <li class={point}>"✅ Azumi validates CSS syntax at compile time"</li>
                    <li class={point}>"✅ Only valid CSS properties and values allowed"</li>
                </ul>
            </section>

            <section class={examples}>
                <div class={example_card}>
                    @scoped_component()
                </div>
                <div class={example_card}>
                    @multiple_scoped_components()
                </div>
                <div class={example_card}>
                    @valid_css_example()
                </div>
            </section>
        </div>
        <style>
            .container { padding: "20px"; }
            .header { text-align: "center"; margin-bottom: "30px"; }
            .main_title { font-size: "32px"; color: "#333"; }
            .subtitle { font-size: "18px"; color: "#666"; }
            .key_points { background: "#f9f9f9"; padding: "20px"; border-radius: "8px"; margin-bottom: "30px"; }
            .section_title { font-size: "20px"; margin-bottom: "15px"; }
            .points_list { list-style: "none"; padding: "0"; }
            .point { margin-bottom: "10px"; }
            .examples { display: "grid"; gap: "20px"; }
            .example_card { border: "1px solid #ddd"; padding: "20px"; border-radius: "8px"; }
        </style>
    }
}

// Handler for Axum
pub async fn lesson1_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson1()))
}
