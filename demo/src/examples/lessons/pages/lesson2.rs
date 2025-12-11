use crate::examples::lessons::components::layout::DarkModernLayout;
use azumi::html;

/// Lesson 3: Global vs Component CSS
///
/// Understanding style scoping options
#[azumi::component]
pub fn global_css_example() -> impl azumi::Component {
    html! {
        <div class={component_class}>
        <h2 class={local_class}>"Scoped Style"</h2>
        <p class={global_demo}>"Global Style Effect (simulated)"</p>
            <p class={text_content}>"This component demonstrates CSS scoping concepts"</p>
        </div>
        // Global styles - not scoped to component
        <style global>
            body { font-family: "'Inter', system-ui, sans-serif"; }
        </style>

        // Component-scoped styles - automatically scoped
        <style>
            .component_class {
                background: "rgba(30, 41, 59, 0.5)";
                padding: "1.5rem";
                border-radius: "12px";
                border: "1px solid rgba(255,255,255,0.1)";
            }
            .local_class { color: "#38bdf8"; font-weight: "600"; margin-bottom: "0.5rem"; }
            .global_demo { color: "#a78bfa"; font-style: "italic"; margin-bottom: "0.5rem"; }
            .text_content { color: "#cbd5e1"; }
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
            <p class={scoped_text}>"Scoped styles are component-specific"</p>
        </div>
        <style global>
            /* This would affect the entire app */
            /* body { font-family: "Arial, sans-serif"; } */
        </style>

        <style>
            .container {
                padding: "1.5rem";
                border: "1px solid rgba(255,255,255,0.1)";
                background: "rgba(15, 23, 42, 0.4)";
                border-radius: "12px";
            }
            .scoped_title { color: "#34d399"; font-size: "1.25rem"; margin-bottom: "0.75rem"; }
            .global_simulation { font-size: "1.0rem"; font-weight: "bold"; color: "#f472b6"; margin-bottom: "0.5rem"; }
            .scoped_text { color: "#94a3b8"; }
        </style>
    }
}

/// Example: CSS scoping best practices
#[azumi::component]
pub fn scoping_best_practices() -> impl azumi::Component {
    html! {
        <div class={best_practices}>
            <h3 class={bp_title}>"CSS Scoping Best Practices"</h3>

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
            .best_practices {
                padding: "1.5rem";
                background: "rgba(30, 41, 59, 0.3)";
                border-radius: "12px";
                border: "1px solid rgba(255,255,255,0.05)";
            }
            .bp_title { color: "#e2e8f0"; margin-bottom: "1rem"; font-size: "1.25rem"; }
            .practice_item {
                margin: "0.75rem 0";
                padding: "0.75rem";
                background: "rgba(15, 23, 42, 0.6)";
                border-radius: "8px";
                color: "#cbd5e1";
                border: "1px solid rgba(255,255,255,0.05)";
            }
            .do_class { color: "#4ade80"; font-weight: "bold"; margin-right: "0.5rem"; }
            .dont_class { color: "#f87171"; font-weight: "bold"; margin-right: "0.5rem"; }
        </style>
    }
}

/// Main lesson demonstration component
#[azumi::component]
pub fn lesson2() -> impl azumi::Component {
    html! {
        @DarkModernLayout() {
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
                        <div class={card_label}>"Example 1: Scoped Styles"</div>
                        @global_css_example()
                    </div>
                    <div class={example_card}>
                        <div class={card_label}>"Example 2: Mixed Scoping"</div>
                        @mixed_scoping_example()
                    </div>
                    <div class={example_card}>
                        <div class={card_label}>"Example 3: Best Practices"</div>
                        @scoping_best_practices()
                    </div>
                </section>
            </div>
            <style>
                .container { max-width: "900px"; margin: "0 auto"; }
                .header { text-align: "center"; margin-bottom: "3rem"; }
                .main_title {
                    font-size: "3rem";
                    font-weight: "800";
                    background: "linear-gradient(to right, #a78bfa, #818cf8)";
                    -webkit-background-clip: "text";
                    -webkit-text-fill-color: "transparent";
                    margin-bottom: "1rem";
                }
                .subtitle { font-size: "1.25rem"; color: "#94a3b8"; }

                .key_points {
                    background: "rgba(30, 41, 59, 0.5)";
                    padding: "2rem";
                    border-radius: "16px";
                    margin-bottom: "3rem";
                    border: "1px solid rgba(255,255,255,0.05)";
                    backdrop-filter: "blur(10px)";
                }
                .section_title {
                    font-size: "1.5rem";
                    color: "#f1f5f9";
                    margin-bottom: "1.5rem";
                    border-bottom: "1px solid rgba(255,255,255,0.1)";
                    padding-bottom: "0.5rem";
                }
                .points_list { list-style: "none"; padding: "0"; display: "grid"; gap: "1rem"; }
                .point {
                    color: "#e2e8f0";
                    padding: "0.75rem";
                    background: "rgba(255,255,255,0.03)";
                    border-radius: "8px";
                    font-size: "1.1rem";
                }

                .examples { display: "grid"; gap: "2rem"; }
                .example_card {
                    border: "1px solid rgba(255,255,255,0.1)";
                    padding: "2rem";
                    border-radius: "16px";
                    background: "rgba(15, 23, 42, 0.6)";
                }
                .card_label {
                    font-size: "0.875rem";
                    color: "#64748b";
                    text-transform: "uppercase";
                    letter-spacing: "0.05em";
                    margin-bottom: "1.5rem";
                    font-weight: "600";
                }
            </style>
        }
    }
}

// Handler for Axum
pub async fn lesson2_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson2()))
}
