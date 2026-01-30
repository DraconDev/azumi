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

    // Azumi uses attribute-based scoping: data-sHASH
    let count = output.matches("data-s").count();
    // 2 in <style> tags, 3 in <div> tags = 5 total
    assert!(
        count >= 5,
        "Expected at least 5 data-s attributes, found {}\nOutput: {}",
        count,
        output
    );

    // Verify we have at least two DIFFERENT scope IDs
    let mut scopes = std::collections::HashSet::new();
    for part in output.split("data-azumi-scope=\"") {
        if let Some(end) = part.find('"') {
            scopes.insert(&part[..end]);
        }
    }
    assert!(
        scopes.len() >= 2,
        "Expected at least 2 unique scope IDs, found {:?}",
        scopes
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
    // Flexible match for CSS content (ignoring exact whitespace/semicolons)
    assert!(
        output.contains(".global_box"),
        "Global selector should be present"
    );
    assert!(
        output.contains("background: green") || output.contains("background:green"),
        "Global style value should be present"
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
    // Azumi renders as "--w: 75.5%" (with space)
    assert!(
        output.contains("--w: 75.5%"),
        "Custom property should be rendered correctly. Output: {}",
        output
    );
}

#[test]
fn test_multiple_custom_properties() {
    let comp = html! {
        <div style={--a: "1"; --b: "2"; --c: "3"}>"Multi"</div>
    };
    let output = test::render(&comp);
    assert!(output.contains("--a: 1") && output.contains("--b: 2") && output.contains("--c: 3"));
}

// ════════════════════════════════════════════════════════════════════════════
// Complex Scoping (Multiple Components)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_style_multiple_classes() {
    // Test that multiple classes from <style> work together
    let comp = html! {
        <div class={container}>
            <div class={header}>"Header"</div>
            <div class={content}>"Content"</div>
        </div>
        <style>
            .container { padding: "1rem"; }
            .header { font-weight: "bold"; }
            .content { color: "blue"; }
        </style>
    };
    let output = test::render(&comp);

    // All classes should be present
    assert!(output.contains("class=\"container\""));
    assert!(output.contains("class=\"header\""));
    assert!(output.contains("class=\"content\""));
    assert!(output.contains("data-s"));
}
