use azumi::html;

/// Lesson 3: Global vs Component CSS
///
/// Understanding style scoping options
#[azumi::component]
pub fn global_css_example() -> impl azumi::Component {
    html! {
        <style global>
            body { font-family: "Arial, sans-serif"; }
        </style>
        <div class={component_class}>
            <h2 class={local_class}>"Scoped Style"</h2>
            <p class={global_demo}>"Global Style Effect (simulated)"</p>
            <p>"This component demonstrates CSS scoping concepts"</p>
        </div>
        // Global styles - not scoped to component

        // Component-scoped styles - automatically scoped
        <style>
            .component_class { background: "#f5f5f5"; padding: "1rem"; }
            .local_class { color: "blue"; }
            .global_demo { color: "purple"; font-style: "italic"; }
        </style>
    }
}

/// Example: Multiple components with different scoping
#[azumi::component]
pub fn mixed_scoping_example() -> impl azumi::Component {
    html! {
        <div class={container}>
            <h3 class={scoped_title}>"Scoping Concepts"</h3>
            <p class={global_simulation}>"Global styles affect everything"</p>
            <p class={scoped_title}>"Scoped styles are component-specific"</p>
        </div>
        <style global>
            /* This would affect the entire app */
            /* body { font-family: "Arial, sans-serif"; } */
        </style>

        <style>
            .container { padding: "1rem"; border: "1px solid #ddd"; }
            .scoped_title { color: "#2196f3"; }
            .global_simulation { font-size: "1.1rem"; font-weight: "bold"; }
        </style>
    }
}

/// Example: CSS scoping best practices
#[azumi::component]
pub fn scoping_best_practices() -> impl azumi::Component {
    html! {
        <div class={best_practices}>
            <h3>"CSS Scoping Best Practices"</h3>

            <div class={practice_item}>
                <span class={do_class}>"DO:"</span> " Use component-scoped styles for most cases"
            </div>

            <div class={practice_item}>
                <span class={do_class}>"DO:"</span> " Use global styles only for truly global elements"
            </div>

            <div class={practice_item}>
                <span class={dont_class}>"DON'T:"</span> " Overuse global styles - they can cause conflicts"
            </div>

            <div class={practice_item}>
                <span class={do_class}>"DO:"</span> " Let Azumi handle scoping automatically"
            </div>
        </div>
        <style>
            .best_practices { padding: "1.5rem"; background: "#f9f9f9"; }
            .practice_item { margin: "0.5rem 0"; padding: "0.5rem"; background: "white"; }
            .do_class { color: "green"; font-weight: "bold"; }
            .dont_class { color: "red"; font-weight: "bold"; }
        </style>
    }
}

/// Main lesson demonstration component
#[azumi::component]
pub fn lesson2() -> impl azumi::Component {
    html! {
        <div class={container}>
            <header class={header}>
                <h1 class={main_title}>"Lesson 3: Global vs Component CSS"</h1>
                <p class={subtitle}>"Understanding style scoping options"</p>
            </header>

            <section class={key_points}>
                <h2 class={section_title}>"Key Concepts"</h2>
                <ul class={points_list}>
                    <li class={point}>"✅ Global styles use <style global> tag"</li>
                    <li class={point}>"✅ Component styles use <style> tag (automatically scoped)"</li>
                    <li class={point}>"✅ Global styles affect the entire application"</li>
                    <li class={point}>"✅ Component styles are scoped to prevent conflicts"</li>
                    <li class={point}>"✅ Azumi handles scoping automatically for component styles"</li>
                </ul>
            </section>

            <section class={examples}>
                <div class={example_card}>
                    @global_css_example()
                </div>
                <div class={example_card}>
                    @mixed_scoping_example()
                </div>
                <div class={example_card}>
                    @scoping_best_practices()
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
pub async fn lesson2_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson2()))
}
