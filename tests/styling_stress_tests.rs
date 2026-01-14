use azumi::{html, test, Component};

// ════════════════════════════════════════════════════════════════════════════
// Scoping & Isolation
// ════════════════════════════════════════════════════════════════════════════

#[azumi::component]
fn ChildComponent() -> impl Component {
    html! {
        <div class={inner_box}>"Child"</div>
        <style>
            .inner_box { color: "red"; }
        </style>
    }
}

#[azumi::component]
fn ParentComponent() -> impl Component {
    html! {
        <div class={container}>
            <div class={outer_box}>"Parent"</div>
            @ChildComponent()
        </div>
        <style>
            .container { padding: "1rem"; }
            .outer_box { color: "blue"; }
        </style>
    }
}

#[test]
fn test_style_scoping_isolation() {
    let comp = html! { @ParentComponent() };
    let output = test::render(&comp);

    // Check that both classes exist but are mangled differently (scoped)
    // Azumi class scoping uses unique IDs (az-xxxx).
    let count = output.matches("class=\"az-").count();
    // We have: container, outer_box, inner_box (from child)
    assert!(
        count >= 3,
        "Expected at least 3 scoped classes, found {}",
        count
    );
}

// ════════════════════════════════════════════════════════════════════════════
// Global Style Propagation
// ════════════════════════════════════════════════════════════════════════════

#[azumi::component]
fn GlobalStyled() -> impl Component {
    html! {
        <div class={"global_box"}>"Global"</div>
        <style global>
            .global_box { background: "green"; }
        </style>
    }
}

#[test]
fn test_global_style_unmangled() {
    let comp = html! { @GlobalStyled() };
    let output = test::render(&comp);
    assert!(
        output.contains("class=\"global_box\""),
        "Global class should not be mangled"
    );
    assert!(
        output.contains(".global_box{background:green}"),
        "Global style should be present in output"
    );
}

// ════════════════════════════════════════════════════════════════════════════
// Dynamic Variable Propagation
// ════════════════════════════════════════════════════════════════════════════

#[azumi::component]
fn Progress(width: f64) -> impl Component {
    html! {
        <div class={track_id}>
            <div class={fill_id} style={--w: format!("{}%", width)}></div>
        </div>
        <style>
            .track_id { width: "100px"; height: "10px"; background: "#eee"; }
            .fill_id { width: "var(--w)"; height: "100%"; background: "blue"; }
        </style>
    }
}

#[test]
fn test_dynamic_custom_properties() {
    let comp = html! { @Progress(width = 75.5) };
    let output = test::render(&comp);
    assert!(
        output.contains("style=\"--w:75.5%\""),
        "Custom property should be rendered correctly"
    );
}

#[test]
fn test_multiple_custom_properties() {
    let comp = html! {
        <div style={--a: "1"; --b: "2"; --c: "3"}>"Multi"</div>
    };
    let output = test::render(&comp);
    assert!(output.contains("style=\"--a:1;--b:2;--c:3\""));
}

// ════════════════════════════════════════════════════════════════════════════
// Complex Scoping (Shadowing)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_style_variable_shadowing() {
    // Test that @let variables don't conflict with @style variables
    let comp = html! {
        <div>
            @let my_manual_class = "manual";
            <div class={my_manual_class}>"Manual"</div>
            <div class={scoped_class}>"Scoped"</div>
        </div>
        <style>
            .scoped_class { color: "yellow"; }
        </style>
    };
    let output = test::render(&comp);

    // "manual" should be literal because it's from @let
    assert!(output.contains("class=\"manual\""));
    // "scoped_class" should be mangled
    assert!(output.contains("class=\"az-"));
}
