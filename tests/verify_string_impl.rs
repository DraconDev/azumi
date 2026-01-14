use azumi::{html, render_to_string, Component};

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
fn test_container(children: impl Component) -> impl Component {
    html! { <div>{children}</div> }
}

#[test]
fn test_nested_string() {
    // verifying that String can be passed to implicit Component bound
    let text = "Hello".to_string();
    let comp = html! {
        @test_container { {text} }
    };
    let result = render_to_string(&comp);
    assert_eq!(result, "<div>Hello</div>");
}
