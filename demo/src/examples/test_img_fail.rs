use azumi::html;

// Test: Should FAIL - Missing alt attribute
pub fn test_fail_missing_alt() -> impl azumi::Component {
    html! {
        <img src="cat.jpg" />
    }
}
