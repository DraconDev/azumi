use azumi::html;

// Test different failing scenarios to see error messages

// ❌ Missing alt
pub fn test_missing_alt() -> impl azumi::Component {
    html! {
        <img src="cat.jpg" />
    }
}

// ❌ Invalid input type
pub fn test_invalid_input_type() -> impl azumi::Component {
    html! {
        <input type="txt" />
    }
}

// ❌ Invalid button type
pub fn test_invalid_button_type() -> impl azumi::Component {
    html! {
        <button type="sumbit">"Click"</button>
    }
}

// ❌ Invalid ARIA role
pub fn test_invalid_role() -> impl azumi::Component {
    html! {
        <div role="btn">"Button"</div>
    }
}

// ❌ Empty button
pub fn test_empty_button() -> impl azumi::Component {
    html! {
        <button type="button"></button>
    }
}
