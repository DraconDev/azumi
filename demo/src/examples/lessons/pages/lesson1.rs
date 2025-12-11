use crate::examples::lessons::components::layout::DarkModernLayout;
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
            .container {
                padding: "1.5rem";
                border: "1px solid rgba(56, 189, 248, 0.3)";
                background: "rgba(56, 189, 248, 0.1)";
                border-radius: "8px";
            }
            .title { color: "#38bdf8"; margin-bottom: "0.5rem"; font-size: "1.25rem"; }
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
                <p class={card_text}>"This uses the same class names as the second component"</p>
            </div>
            <div class={card}>
                <h3 class={card_title}>"Second Component"</h3>
                <p class={card_text}>"But the CSS is automatically scoped, so no conflicts!"</p>
            </div>
        </div>
        <style>
            .card {
                padding: "1.5rem";
                margin: "1rem 0";
                border: "1px solid rgba(255,255,255,0.1)";
                background: "rgba(30, 41, 59, 0.6)";
                border-radius: "12px";
            }
            .card_title { font-weight: "bold"; color: "#a78bfa"; margin-bottom: "0.5rem"; font-size: "1.1rem"; }
            .card_text { color: "#cbd5e1"; }
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
            <ul class={valid_list}>
                <li>"Proper property values"</li>
                <li>"Valid color formats"</li>
                <li>"Correct unit usage"</li>
            </ul>
        </div>
        <style>
            .valid_container {
                padding: "2rem";
                background: "rgba(16, 185, 129, 0.1)";
                border: "1px solid rgba(16, 185, 129, 0.2)";
                border-radius: "12px";
            }
            .valid_title { color: "#34d399"; font-size: "1.5rem"; margin-bottom: "1rem"; }
            .valid_text { color: "#e2e8f0"; margin-bottom: "1rem"; }
            .valid_list { color: "#cbd5e1"; padding-left: "1.5rem"; }
        </style>
    }
}

/// Main lesson demonstration component
#[azumi::component]
pub fn lesson1() -> impl azumi::Component {
    html! {
        @DarkModernLayout() {
            <div class={container}>
                <header class={header}>
                    <h1 class={main_title}>"Lesson 2: Scoping & Validation"</h1>
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
                         <div class={card_label}>"Example 1: Scoped Styles"</div>
                        @scoped_component()
                    </div>
                    <div class={example_card}>
                         <div class={card_label}>"Example 2: Conflict Prevention"</div>
                        @multiple_scoped_components()
                    </div>
                    <div class={example_card}>
                         <div class={card_label}>"Example 3: Validation"</div>
                        @valid_css_example()
                    </div>
                </section>
            </div>
            <style>
                .container { max-width: "900px"; margin: "0 auto"; }
                .header { text-align: "center"; margin-bottom: "3rem"; }
                .main_title {
                    font-size: "3rem";
                    font-weight: "800";
                    background: "linear-gradient(to right, #34d399, #38bdf8)";
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
pub async fn lesson1_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson1()))
}
