use azumi::html;

// ✅ All valid - should compile successfully
#[azumi::component]
pub fn test_valid_accessibility() -> impl azumi::Component {
    html! {
        <div>
            <img src="logo.png" alt="Company Logo" />
            <img src="decoration.png" alt="" />
            <input type="email" />
            <input type="text" />
            <button type="submit" role="button">"Submit"</button>
            <button type="button">"Cancel"</button>
            <div role="navigation">"Nav"</div>
            <div role="alert">"Alert"</div>
        </div>
    }
}

#[allow(dead_code)]
pub async fn test_handler() -> axum::response::Html<String> {
    let component = test_valid_accessibility();
    axum::response::Html(azumi::render_to_string(&component))
}
