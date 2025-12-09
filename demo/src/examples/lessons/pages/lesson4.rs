/// Lesson 5: Children Pattern
///
/// Components with children parameter
use azumi::html;

#[azumi::component]
pub fn container(children: impl azumi::Component) -> impl azumi::Component {
    html! {
        <style>
            .content_box { padding: "2rem"; border: "1px solid #ddd"; border-radius: "8px"; }
        </style>
        <div class={content_box}>
            {children}
        </div>
    }
}

/// Example: Layout with children
#[azumi::component]
pub fn layout_example() -> impl azumi::Component {
    html! {
        <style>
            .layout_title { font-size: "1.5rem"; color: "#2196f3"; margin-bottom: "1rem"; }
        </style>
        <div>
            <h2 class={layout_title}>"Container with Children"</h2>

            @container() {
                <p>"This content is passed as children"</p>
                <p>"Children can be any valid Azumi components"</p>
            }
        </div>
    }
}

/// Example: Nested children components
#[azumi::component]
pub fn nested_children() -> impl azumi::Component {
    html! {
        <style>
            .outer_container { background: "#f0f0f0"; padding: "1.5rem"; }
            .inner_container { background: "white"; padding: "1rem"; margin-top: "1rem"; }
        </style>
        <div>
            <h3>"Nested Children Example"</h3>
            @container() {
                <p>"Outer content"</p>
                <div class={outer_container}>
                    <p>"Inner nested content"</p>
                    @container() {
                        <p>"Deeply nested content"</p>
                    }
                </div>
            }
        </div>
    }
}

/// Example: Children with multiple elements
#[azumi::component]
pub fn multiple_children_example() -> impl azumi::Component {
    html! {
        <style>
            .children_demo { display: "grid"; gap: "1rem"; }
            .child_item { padding: "0.5rem"; background: "#f9f9f9"; border: "1px solid #eee"; }
        </style>
        <div>
            @container() {
                <div class={children_demo}>
                    <div class={child_item}>
                        "Child 1"
                    </div>
                    <div class={child_item}>
                        "Child 2"
                    </div>
                    <div class={child_item}>
                        "Child 3"
                    </div>
                </div>
            }
            @container() {
                <p>"Multiple children example"</p>
            }
        </div>
    }
}

/// Main lesson demonstration component
#[azumi::component]
pub fn lesson4() -> impl azumi::Component {
    html! {
        <style>
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
        @layout_mod::DarkModernLayout() {
            <div>
                <header class={header}>
                    <h1 class={main_title}>"Lesson 5: Children Pattern"</h1>
                    <p class={subtitle}>"Components with children parameter"</p>
                </header>

                <section class={key_points}>
                    <h2 class={section_title}>"Key Concepts"</h2>
                    <ul class={points_list}>
                        <li class={point}>"✅ Children parameter allows component composition"</li>
                        <li class={point}>"✅ Pass any Azumi component as children"</li>
                        <li class={point}>"✅ Children can contain multiple elements"</li>
                        <li class={point}>"✅ Enables flexible layout patterns"</li>
                        <li class={point}>"✅ Maintains clean component boundaries"</li>
                    </ul>
                </section>

                <section class={examples}>
                    <div class={example_card}>
                        @layout_example()
                    </div>
                    <div class={example_card}>
                        @nested_children()
                    </div>
                    <div class={example_card}>
                        @multiple_children_example()
                    </div>
                </section>
            </div>
        }
    }
}

// Handler for Axum
pub async fn lesson4_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson4()))
}
