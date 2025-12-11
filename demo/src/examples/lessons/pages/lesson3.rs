/// Lesson 4: Component Composition Patterns
///
/// Building complex UIs from simple components
#[azumi::component]
pub fn card<'a>(title: &'a str, content: &'a str) -> impl azumi::Component + 'a {
    azumi::html! {
        <div class={card}>
            <h3 class={card_title}>{title}</h3>
            <p>{content}</p>
        </div>
        <style>
            .card { border: "1px solid #eee"; padding: "1rem"; margin: "0.5rem"; }
            .card_title { font-weight: "bold"; margin-bottom: "0.5rem"; }
        </style>
    }
}

/// Example: Dashboard composed of multiple cards
#[azumi::component]
pub fn dashboard() -> impl azumi::Component {
    azumi::html! {
        <div>
            <h2 class={dashboard_title}>"Component Composition Dashboard"</h2>
            <div class={dashboard_container}>
                @card(title="Welcome", content="Welcome to Azumi Component Composition")
                @card(title="Features", content="Type-safe components that compose beautifully")
                @card(title="Performance", content="Compile-time optimized rendering")
            </div>
        </div>
        <style>
            .dashboard_container { display: "grid"; gap: "1rem"; }
            .dashboard_title { font-size: "1.5rem"; color: "#2196f3"; margin-bottom: "1rem"; }
        </style>
    }
}

/// Example: Complex layout with nested composition
#[azumi::component]
pub fn complex_layout() -> impl azumi::Component {
    azumi::html! {
        <div class={layout_container}>
            <div class={header_section}>
                <h2>"Complex Layout Composition"</h2>
                <p>"Multiple components working together"</p>
            </div>
            <div class={main_section}>
                <div>
                    @card(title="Main Content", content="This is the primary content area")
                    @card(title="Additional Info", content="More information here")
                </div>
                <div class={sidebar}>
                    <h3>"Sidebar"</h3>
                    @card(title="Quick Links", content="Navigation and tools")
                </div>
            </div>
        </div>
        <style>
            .layout_container { display: "grid"; gap: "1rem"; }
            .header_section { background: "#f0f0f0"; padding: "1rem"; }
            .main_section { display: "grid"; grid-template-columns: "2fr1fr"; gap: "1rem"; }
            .sidebar { background: "#f9f9f9"; padding: "1rem"; }
        </style>
    }
}

/// Main lesson demonstration component
#[azumi::component]
pub fn lesson3() -> impl azumi::Component {
    azumi::html! {
        <div class={container}>
            <header class={header}>
                <h1 class={main_title}>"Lesson 4: Component Composition Patterns"</h1>
                <p class={subtitle}>"Building complex UIs from simple components"</p>
            </header>

            <section class={key_points}>
                <h2 class={section_title}>"Key Concepts"</h2>
                <ul class={points_list}>
                    <li class={point}>"✅ Create simple, focused components"</li>
                    <li class={point}>"✅ Compose them together to build complex UIs"</li>
                    <li class={point}>"✅ Reuse components across your application"</li>
                    <li class={point}>"✅ Pass props to customize component behavior"</li>
                    <li class={point}>"✅ Maintain clean separation of concerns"</li>
                </ul>
            </section>

            <section class={examples}>
                <div class={example_card}>
                    @dashboard()
                </div>
                <div class={example_card}>
                    @complex_layout()
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
pub async fn lesson3_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson3()))
}
