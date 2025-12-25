use azumi::html;

// Test 1: Multiple elements at top level (automatic fragment)
#[azumi::component]
pub fn automatic_fragment() -> impl azumi::Component {
    html! {
        <h1>"First Element"</h1>
        <p>"Second Element"</p>
    }
}

// Test 2: Explicit fragment with <>
#[azumi::component]
pub fn explicit_fragment() -> impl azumi::Component {
    html! {
        <>
            <h1>"First Element"</h1>
            <p>"Second Element"</p>
        </>
    }
}

// Test 3: Nested fragments in control flow
#[azumi::component]
pub fn control_flow_fragment() -> impl azumi::Component {
    let show = true;
    html! {
        <div>"Container"</div>
        @if show {
            <h1>"Shown Title"</h1>
            <p>"Shown Paragraph"</p>
        }
    }
}

// Test 4: Explicit fragment in control flow
#[azumi::component]
pub fn control_flow_explicit_fragment() -> impl azumi::Component {
    let show = true;
    html! {
        <div>"Container"</div>
        @if show {
            <>
                <h1>"Shown Title"</h1>
                <p>"Shown Paragraph"</p>
            </>
        }
    }
}
