use azumi::html;

// Test cases for accessibility validation

// ❌ SHOULD FAIL: Missing alt attribute
#[azumi::component]
pub fn test_missing_alt() -> impl azumi::Component {
    html! {
        <img src="cat.jpg" />
    }
}

// ✅ SHOULD PASS: Has alt attribute
#[azumi::component]
pub fn test_valid_alt() -> impl azumi::Component {
    html! {
        <img src="cat.jpg" alt="A cute cat" />
    }
}

// ✅ SHOULD PASS: Decorative image with empty alt
#[azumi::component]
pub fn test_decorative_image() -> impl azumi::Component {
    html! {
        <img src="decoration.png" alt="" />
    }
}

// ❌ SHOULD FAIL: Invalid input type (typo)
#[azumi::component]
pub fn test_invalid_input_type() -> impl azumi::Component {
    html! {
        <input type="txt" />
    }
}

// ✅ SHOULD PASS: Valid input type
#[azumi::component]
pub fn test_valid_input_type() -> impl azumi::Component {
    html! {
        <input type="text" />
    }
}

// ❌ SHOULD FAIL: Invalid button type (typo)
#[azumi::component]
pub fn test_invalid_button_type() -> impl azumi::Component {
    html! {
        <button type="sumbit">"Submit"</button>
    }
}

// ✅ SHOULD PASS: Valid button type
#[azumi::component]
pub fn test_valid_button_type() -> impl azumi::Component {
    html! {
        <button type="submit">"Submit"</button>
    }
}

// ❌ SHOULD FAIL: Invalid ARIA role
#[azumi::component]
pub fn test_invalid_aria_role() -> impl azumi::Component {
    html! {
        <div role="btn">"Click me"</div>
    }
}

// ✅ SHOULD PASS: Valid ARIA role
#[azumi::component]
pub fn test_valid_aria_role() -> impl azumi::Component {
    html! {
        <button role="button">"Click me"</button>
    }
}

// ✅ SHOULD PASS: Multiple valid validations
#[azumi::component]
pub fn test_all_valid() -> impl azumi::Component {
    html! {
        <div>
            <img src="logo.png" alt="Company Logo" />
            <input type="email" />
            <button type="submit" role="button">"Submit Form"</button>
        </div>
    }
}
