use azumi::{html, test};

// ════════════════════════════════════════════════════════════════════════════
// Combinatorial Control Flow
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_nested_if_else_matrix() {
    let check = |a: bool, b: bool, c: bool| {
        let comp = html! {
            <div>
                @if a {
                    @if b { "A-B" } else { "A-notB" }
                } else {
                    @if c { "notA-C" } else { "notA-notC" }
                }
            </div>
        };
        test::render(&comp)
    };

    assert!(check(true, true, true).contains("A-B"));
    assert!(check(true, false, true).contains("A-notB"));
    assert!(check(false, true, true).contains("notA-C"));
    assert!(check(false, false, false).contains("notA-notC"));
}

#[test]
fn test_loop_with_nested_logic() {
    let items = vec![1, 2, 3, 4, 5];
    let comp = html! {
        <div>
            @for item in &items {
                <span>
                    @if item % 2 == 0 {
                        "Even: " {item}
                    } else {
                        "Odd: " {item}
                    }
                </span>
            }
        </div>
    };
    let output = test::render(&comp);
    assert!(output.contains("Odd: 1"));
    assert!(output.contains("Even: 2"));
    assert!(output.contains("Odd: 3"));
}

#[test]
fn test_match_with_nesting() {
    let vals = vec![Some(1), None, Some(2)];
    let comp = html! {
        <div>
            @for val in &vals {
                @match val {
                    Some(v) => {
                        @if *v > 1 {
                            "Big: " {v}
                        } else {
                            "Small: " {v}
                        }
                    }
                    None => { "Empty" }
                }
            }
        </div>
    };
    let output = test::render(&comp);
    assert!(output.contains("Small: 1"));
    assert!(output.contains("Empty"));
    assert!(output.contains("Big: 2"));
}

// ════════════════════════════════════════════════════════════════════════════
// Expressions Stress
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_complex_math_expressions() {
    let x: f64 = 10.0;
    let y: f64 = 20.0;
    let comp = html! {
        <div data-calc={ (x * y + 5.0).to_string() }>
            { (x.powi(2) + y.sqrt()).to_string() }
        </div>
    };
    let output = test::render(&comp);
    assert!(output.contains("data-calc=\"205\""));
    assert!(output.contains("104.47")); // 100 + 4.472
}

#[test]
fn test_string_manipulation_in_template() {
    let s = "  Azumi Framework  ";
    let comp = html! {
        <div>
            <span class={s.trim().to_lowercase().replace(" ", "_")}>
                {s.trim().to_uppercase()}
            </span>
        </div>
    };
    let output = test::render(&comp);
    assert!(output.contains("class=\"azumi_framework\""));
    assert!(output.contains("AZUMI FRAMEWORK"));
}

// ════════════════════════════════════════════════════════════════════════════
// Scale Tests
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_high_iteration_loop() {
    let items: Vec<i32> = (0..500).collect();
    let comp = html! {
        <table>
            @for i in &items {
                <tr>
                    <td>"Row " {i}</td>
                    <td>{if i % 2 == 0 { "Even" } else { "Odd" }}</td>
                </tr>
            }
        </table>
    };
    let output = test::render(&comp);
    assert!(output.contains("Row 499"));
    assert!(output.contains("<td>Odd</td>"));
    assert_eq!(output.matches("<tr>").count(), 500);
}

#[test]
    let lorem = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. ".repeat(10);
    let comp = html! {
        <div>
            <h1>"Title"</h1>
            <p>{lorem}</p>
            <ul>
                @for i in 0..100 {
                    <li>"Item "{i}</li>
                }
            </ul>
        </div>
    };
    let output = test::render(&comp);
    assert!(output.contains("Item 99"));
    assert!(output.len() > 5000);
}

// ════════════════════════════════════════════════════════════════════════════
// Edge Case Logic
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_optional_attributes_logic() {
    let maybe_id = Some("my-id");
    let no_id: Option<&str> = None;

    let comp = html! {
        <div>
            @if let Some(id) = maybe_id {
                <span id={id}>"Has ID"</span>
            }
            @if let Some(id) = no_id {
                <span id={id}>"Should not exist"</span>
            }
        </div>
    };
    let output = test::render(&comp);
    assert!(output.contains("id=\"my-id\""));
    assert!(!output.contains("Should not exist"));
}
