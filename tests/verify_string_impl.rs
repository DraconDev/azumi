use azumi::{html, render_to_string};

#[test]
fn test_string_render() {
    let result = render_to_string(&"Hello World".to_string());
    assert_eq!(result, "Hello World");
}

#[test]
fn test_str_render() {
    let result = render_to_string("Hello World");
    assert_eq!(result, "Hello World");
}

#[test]
fn test_escaping() {
    let result = render_to_string("<script>");
    assert_eq!(result, "&lt;script&gt;");
}

#[azumi::component]
fn test_container(children: impl azumi::Component) -> impl azumi::Component {
    html! { <div>{children}</div> }
}

#[test]
fn test_nested_string() {
    // Manually instantiate to bypass html! macro wrapping behavior
    // and prove that String satisfies "impl Component"
    let props = test_container_component::Props::builder().build().unwrap();
    let comp = test_container_component::render(props, "Hello".to_string());

    // Note: render returns impl Component (the rendered structure)
    // We can verify it renders to string
    let result = render_to_string(&comp);
    assert_eq!(result, "<div>Hello</div>");
}
