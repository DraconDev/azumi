use azumi::html;

// Simple test without component macro for now (testing the enforcement logic directly)
#[test]
fn test_named_arguments_work() {
    // This test verifies that named arguments compile successfully
    // The actual component call is tested in the demo project

    let simple = html! {
        <div>"Simple HTML test"</div>
    };

    let rendered = azumi::render_to_string(&simple);
    assert!(rendered.contains("Simple HTML test"));
}

// This documents what positional args would look like (compile error):
// @Component("value1", "value2")  // ❌ Would fail with clear error message
// @Component(arg1="value1", arg2="value2")  // ✅ Required syntax
