use crate::examples::lessons::components::layout::DarkModernLayout;
use azumi::html;

/// Lesson 6.5: @let Pattern for Local Variables
///
/// Using @let for local variable declarations
#[azumi::component]
pub fn let_pattern_example() -> impl azumi::Component {
    html! {
        <div class={let_demo}>
            <h2 class={title}>"@let Pattern Examples"</h2>

            // Basic variable declaration
            @let name = "Azumi";
            <p>"Hello, " <span class={calculated}>{name}</span> "!"</p>

            // Calculated values
            @let items = vec!["Item 1", "Item 2", "Item 3"];
            @let item_count = items.len();
            <p>"Total items: " <span class={calculated}>{item_count}</span></p>

            // Derived values from calculations
            @let base_price = 100.0;
            @let tax_rate = 0.08;
            @let total_price = base_price * (1.0 + tax_rate);
            <div class={derived}>
                <p>"Base Price: ${base_price}"</p>
                <p>"Tax Rate: {tax_rate * 100}%"</p>
                <p>"Total: ${total_price:.2}"</p>
            </div>

            // Complex data transformations
            @let users = vec![
                ("Alice", 25),
                ("Bob", 30),
                ("Charlie", 35)
            ];
            @let user_names = users.iter().map(|(name, _)| *name).collect::<Vec<&str>>();
            <div class={derived}>
                <h3>"User Names:"</h3>
                @for name in user_names {
                    <p>{name}</p>
                }
            </div>
        </div>
        <style>
            .let_demo { padding: "1.5rem"; }
            .title { color: "#e2e8f0"; margin-bottom: "1rem"; font-size: "1.25rem"; }
            .calculated { font-weight: "bold"; color: "#38bdf8"; }
            .derived {
                background: "rgba(15, 23, 42, 0.4)";
                padding: "1rem";
                border-radius: "8px";
                margin-top: "1rem";
                border: "1px solid rgba(255,255,255,0.05)";
                color: "#cbd5e1";
            }
        </style>
    }
}

/// Example: @let with conditional logic
#[azumi::component]
pub fn let_with_conditions() -> impl azumi::Component {
    html! {
        <div class={conditions_demo}>
            <h3 class={title}>"@let with Conditions"</h3>

            @let score = 85;
            @let grade = if score >= 90 {
                "A"
            } else if score >= 80 {
                "B"
            } else if score >= 70 {
                "C"
            } else {
                "F"
            };

            <p>"Score: " <span class={result}>{score}</span></p>
            <p>"Grade: " <span class={result}>{grade}</span></p>
        </div>
        <style>
            .conditions_demo {
                padding: "1.5rem";
                background: "rgba(20, 184, 166, 0.1)";
                border-radius: "12px";
                border: "1px solid rgba(20, 184, 166, 0.2)";
                color: "#e2e8f0";
            }
            .title { color: "#2dd4bf"; margin-bottom: "1rem"; font-size: "1.25rem"; }
            .result { font-weight: "bold"; color: "#5eead4"; }
        </style>
    }
}

/// Example: @let for component composition
#[azumi::component]
pub fn let_composition_example() -> impl azumi::Component {
    html! {
        <div class={composition_demo}>
            <h3 class={title}>"@let for Composition"</h3>

            @let title = "Dynamic Component";
            @let content = "This component uses @let variables";

            <div class={component_container}>
                <h4 class={comp_title}>{title}</h4>
                <p>{content}</p>
            </div>
        </div>
        <style>
            .composition_demo { padding: "1.5rem"; }
            .title { color: "#e2e8f0"; margin-bottom: "1rem"; font-size: "1.25rem"; }
            .component_container {
                margin: "1rem 0";
                padding: "1rem";
                background: "rgba(30, 41, 59, 0.6)";
                border-radius: "8px";
                border: "1px solid rgba(255,255,255,0.05)";
                color: "#cbd5e1";
            }
            .comp_title { color: "#a5f3fc"; margin-bottom: "0.5rem"; }
        </style>
    }
}

/// Main lesson demonstration component
#[azumi::component]
pub fn lesson5() -> impl azumi::Component {
    html! {
        @DarkModernLayout() {
            <div class={container}>
                <header class={header}>
                    <h1 class={main_title}>"Lesson 6.5: @let Pattern for Local Variables"</h1>
                    <p class={subtitle}>"Using @let for local variable declarations"</p>
                </header>

                <section class={key_points}>
                    <h2 class={section_title}>"Key Concepts"</h2>
                    <ul class={points_list}>
                        <li class={point}>"✅ @let for local variable declarations"</li>
                        <li class={point}>"✅ Works within html! macro"</li>
                        <li class={point}>"✅ Can be used for calculations"</li>
                        <li class={point}>"✅ Supports complex expressions"</li>
                        <li class={point}>"✅ Enables cleaner component logic"</li>
                    </ul>
                </section>

                <section class={examples}>
                    <div class={example_card}>
                        @let_pattern_example()
                    </div>
                    <div class={example_card}>
                        @let_with_conditions()
                    </div>
                    <div class={example_card}>
                        @let_composition_example()
                    </div>
                </section>
            </div>
            <style>
                .container { max-width: "900px"; margin: "0 auto"; }
                .header { text-align: "center"; margin-bottom: "3rem"; }
                .main_title {
                    font-size: "3rem";
                    font-weight: "800";
                    background: "linear-gradient(to right, #2dd4bf, #38bdf8)";
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
            </style>
        }
    }
}

// Handler for Axum
pub async fn lesson5_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson5()))
}
