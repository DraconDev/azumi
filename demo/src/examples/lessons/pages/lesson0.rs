use azumi::html;

/// Lesson 1: Introduction to Azumi Components
///
/// Basic component structure with proper syntax
#[azumi::component]
pub fn hello_world() -> impl azumi::Component {
    html! {
        <div class={greeting}>"Hello, Azumi!"</div>
        <style>
            .greeting { color: "#1976d2"; font-size: "1.5rem"; }
        </style>
    }
}

/// Example: Component with styling and structure
#[azumi::component]
pub fn basic_component() -> impl azumi::Component {
    html! {
        <div class={container}>
            <h1 class={title}>"Basic Azumi Component"</h1>
            <p>"This demonstrates the basic component structure"</p>
        </div>
        <style>
            .container { padding: "1rem"; border: "1px solid #ddd"; }
            .title { color: "#2196f3"; }
        </style>
    }
}

/// Example: Component with multiple elements
#[azumi::component]
pub fn multi_element_component() -> impl azumi::Component {
    html! {
        <div class={card}>
            <h2 class={card_title}>"Multi-Element Component"</h2>
            <p class={card_content}>"Components can contain multiple elements with proper styling"</p>
            <p class={card_content}>"All CSS is automatically scoped to this component"</p>
        </div>
        <style>
            .card { padding: "1.5rem"; margin: "1rem 0"; border: "1px solid #eee"; border-radius: "8px"; }
            .card_title { font-size: "1.2rem"; color: "#1976d2"; margin-bottom: "0.5rem"; }
            .card_content { color: "#666"; }
        </style>
    }
}

/// Main lesson demonstration component
#[azumi::component]
pub fn lesson0() -> impl azumi::Component {
    html! {
        <div class={container}>
            <header class={header}>
                <h1 class={main_title}>"Lesson 1: Introduction to Azumi Components"</h1>
                <p class={subtitle}>"Basic component syntax and structure"</p>
            </header>

            <section class={key_points}>
                <h2 class={section_title}>"Key Concepts"</h2>
                <ul class={points_list}>
                    <li class={point}>"✅ Components use #[azumi::component] macro"</li>
                    <li class={point}>"✅ CSS is automatically scoped to each component"</li>
                    <li class={point}>"✅ Components return impl azumi::Component"</li>
                    <li class={point}>"✅ HTML structure uses html! macro"</li>
                    <li class={point}>"✅ All text content must be quoted"</li>
                </ul>
            </section>

            <section class={examples}>
                <div class={example_card}>
                    @hello_world()
                </div>
                <div class={example_card}>
                    @basic_component()
                </div>
                <div class={example_card}>
                    @multi_element_component()
                </div>
            </section>
        </div>


    }
}

// Handler for Axum
pub async fn handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson0()))
}
