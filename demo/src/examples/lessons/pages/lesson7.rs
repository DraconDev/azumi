use azumi::html;

/// Lesson 7: Form Handling with Validation
///
/// Form validation with compile-time checks
#[derive(Debug)]
#[allow(dead_code)]
struct UserForm {
    name: String,
    email: String,
    age: i32,
}

#[azumi::component]
pub fn user_form_component() -> impl azumi::Component {
    html! {
        <style>
            .form { display: "grid"; gap: "1rem"; max-width: "400px"; }
            .form_field { display: "grid"; gap: "0.5rem"; }
            .form_label { font-weight: "bold"; }
            .form_input { padding: "0.5rem"; border: "1px solid #ddd"; }
            .form_button { padding: "0.75rem"; background: "#2196f3"; color: "white"; border: "none"; cursor: "pointer"; }
        </style>
        <form class={form}>
            <div class={form_field}>
                <label class={form_label} for="name">"Name"</label>
                <input class={form_input} type="text" name="name" required />
            </div>
            <div class={form_field}>
                <label class={form_label} for="email">"Email"</label>
                <input class={form_input} type="email" name="email" required />
            </div>
            <div class={form_field}>
                <label class={form_label} for="age">"Age"</label>
                <input class={form_input} type="number" name="age" min="18" max="120" />
            </div>
            <button class={form_button} type="submit">"Submit"</button>
        </form>
    }
}

/// Example: Validation feedback
#[azumi::component]
pub fn validation_example() -> impl azumi::Component {
    html! {
        <style>
            .validation_container { padding: "1rem"; }
            .error_message { color: "red"; font-size: "0.9rem"; margin-top: "0.5rem"; }
            .success_message { color: "green"; font-size: "0.9rem"; margin-top: "0.5rem"; }
        </style>
        <div class={validation_container}>
            <h3>"Form Validation"</h3>

            <p>"Azumi provides compile-time validation for:"</p>
            <ul>
                <li>"Required fields"</li>
                <li>"Proper input types"</li>
                <li>"Valid attribute values"</li>
                <li>"Accessible form structure"</li>
            </ul>

            <p class={success_message}>"✅ Valid form structure"</p>
            <p class={error_message}>"❌ Invalid patterns caught at compile time"</p>
        </div>
    }
}

/// Example: Complex form with multiple fields
#[azumi::component]
pub fn complex_form_example() -> impl azumi::Component {
    html! {
        <style>
            .complex_form { display: "grid"; gap: "1rem"; max-width: "500px"; }
            .form_section { border: "1px solid #eee"; padding: "1rem"; margin-bottom: "1rem"; }
            .section_title { font-weight: "bold"; margin-bottom: "0.5rem"; }
            .form_grid { display: "grid"; gap: "0.5rem"; }
            .save_button { padding: "0.75rem"; background: "#4caf50"; color: "white"; border: "none"; }
        </style>
        <form class={complex_form}>
            <div class={form_section}>
                <h3 class={section_title}>"Personal Information"</h3>
                <div class={form_grid}>
                    <label for="fullname">"Full Name"</label>
                    <input type="text" name="fullname" required />

                    <label for="birthdate">"Birth Date"</label>
                    <input type="date" name="birthdate" />

                    <label for="country">"Country"</label>
                    <select name="country">
                        <option value="us">"United States"</option>
                        <option value="uk">"United Kingdom"</option>
                        <option value="ca">"Canada"</option>
                    </select>
                </div>
            </div>

            <div class={form_section}>
                <h3 class={section_title}>"Preferences"</h3>
                <div class={form_grid}>
                    <label>
                        <input type="checkbox" name="newsletter" />
                        " Subscribe to newsletter"
                    </label>

                    <label>
                        <input type="checkbox" name="notifications" />
                        " Enable notifications"
                    </label>
                </div>
            </div>

            <button class={save_button} type="submit">"Save Preferences"</button>
        </form>
    }
}

/// Main lesson demonstration component
#[azumi::component]
pub fn lesson7() -> impl azumi::Component {
    html! {
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
        <div class={container}>
            <header class={header}>
                <h1 class={main_title}>"Lesson 7: Form Handling with Validation"</h1>
                <p class={subtitle}>"Form validation with compile-time checks"</p>
            </header>

            <section class={key_points}>
                <h2 class={section_title}>"Key Concepts"</h2>
                <ul class={points_list}>
                    <li class={point}>"✅ Form validation at compile time"</li>
                    <li class={point}>"✅ Required field validation"</li>
                    <li class={point}>"✅ Input type validation"</li>
                    <li class={point}>"✅ Accessible form structure"</li>
                    <li class={point}>"✅ Type-safe form handling"</li>
                </ul>
            </section>

            <section class={examples}>
                <div class={example_card}>
                    @user_form_component()
                </div>
                <div class={example_card}>
                    @validation_example()
                </div>
                <div class={example_card}>
                    @complex_form_example()
                </div>
            </section>
        </div>
    }
}

// Handler for Axum
pub async fn lesson7_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson7()))
}
