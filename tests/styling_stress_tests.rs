use azumi::{html, test, Component};

// ════════════════════════════════════════════════════════════════════════════
// Scoping & Isolation
// ════════════════════════════════════════════════════════════════════════════

#[azumi::component]
fn ChildComponent() -> impl Component {
    html! {
        <div class={box}>"Child"</div>
        <style>
            .box { color: "red"; }
        </style>
    }
}

#[azumi::component]
fn ParentComponent() -> impl Component {
    html! {
        <div class={container}>
            <div class={box}>"Parent"</div>
            @ChildComponent()
        </div>
        <style>
            .container { padding: "1rem"; }
            .box { color: "blue"; }
        </style>
    }
}

#[test]
fn test_style_scoping_isolation() {
    let output = test::render(&ParentComponent());

    // Check that both classes exist but are mangled differently (scoped)
    // The macro generates variables like `box` which are replaced by unique IDs.
    // We expect to find two different mangled class names.

    // Scraper doesn't give us the mangled names easily, but we can check for unique strings.
    // In Azumi, scoped classes look like `az-xxxx`.
    let count = output.matches("class=\"az-").count();
    assert!(
        count >= 2,
        "Expected at least 2 scoped classes, found {}",
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
    let output = test::render(&GlobalStyled());
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
        <div class={track}>
            <div class={fill} style={--w: format!("{}%", width)}></div>
        </div>
        <style>
            .track { width: "100px"; height: "10px"; background: "#eee"; }
            .fill { width: "var(--w)"; height: "100%"; background: "blue"; }
        </style>
    }
}

#[test]
fn test_dynamic_custom_properties() {
    let output = test::render(&Progress(75.5));
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
    let output = test::render(&html! {
        <div>
            @let my_class = "manual";
            <div class={my_class}>"Manual"</div>
            <div class={scoped_class}>"Scoped"</div>
        </div>
        <style>
            .scoped_class { color: "yellow"; }
        </style>
    });

    // "manual" should be literal because it's from @let
    assert!(output.contains("class=\"manual\""));
    // "scoped_class" should be mangled
    assert!(output.contains("class=\"az-"));
}
