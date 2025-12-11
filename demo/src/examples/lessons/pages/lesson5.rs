use azumi::html;

/// Lesson 6.5: @let Pattern for Local Variables
///
/// Using @let for local variable declarations
#[azumi::component]
pub fn let_pattern_example() -> impl azumi::Component {
    html! {
        <div class={let_demo}>
            <h2>"@let Pattern Examples"</h2>

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
            @let _total_price = base_price * (1.0 + tax_rate);
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
            .let_demo { padding: "1rem"; }
            .calculated { font-weight: "bold"; color: "#2196f3"; }
            .derived { background: "#f0f0f0"; padding: "0.5rem"; }
        </style>
    }
}

/// Example: @let with conditional logic
#[azumi::component]
pub fn let_with_conditions() -> impl azumi::Component {
    html! {
        <div class={conditions_demo}>
            <h3>"@let with Conditions"</h3>

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
            .conditions_demo { padding: "1rem"; background: "#f9f9f9"; }
            .result { font-weight: "bold"; }
        </style>
    }
}

/// Example: @let for component composition
#[azumi::component]
pub fn let_composition_example() -> impl azumi::Component {
    html! {
        <div class={composition_demo}>
            <h3>"@let for Composition"</h3>

            @let title = "Dynamic Component";
            @let content = "This component uses @let variables";

            <div class={component_container}>
                <h4>{title}</h4>
                <p>{content}</p>
            </div>
        </div>
        <style>
            .composition_demo { padding: "1rem"; }
            .component_container { margin: "0.5rem 0"; padding: "0.5rem"; background: "#f5f5f5"; }
        </style>
    }
}

/// Main lesson demonstration component
#[azumi::component]
pub fn lesson5() -> impl azumi::Component {
    html! {
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
pub async fn lesson5_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson5()))
}
