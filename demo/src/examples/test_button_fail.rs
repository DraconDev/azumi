use azumi::html;

// ❌ Should FAIL - Button with no content or labels
pub fn test_fail_empty_button() -> impl azumi::Component {
    html! {
        <button type="button"></button>
    }
}
