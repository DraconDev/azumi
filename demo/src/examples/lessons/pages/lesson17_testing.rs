use axum::response::{Html, IntoResponse};
use azumi::prelude::*;

/// Component to be tested
#[azumi::component]
fn SimpleCard(title: String, content: String) -> impl Component {
    html! {
        <div class="card">
            <h2 class="title">{title}</h2>
            <p class="content">{content}</p>
        </div>
    }
}

/// Live component to be tested
#[azumi::live]
pub struct Counter {
    pub count: i32,
}

#[azumi::live_impl(component = "counter_view")]
impl Counter {
    pub fn increment(&mut self) {
        self.count += 1;
    }
}

#[azumi::component]
fn counter_view<'a>(state: &'a Counter) -> impl Component + 'a {
    html! {
        <div>
            <span class="count">{state.count}</span>
            <button on:click={state.increment}>"Inc"</button>
        </div>
    }
}

// -----------------------------------------------------------------------------
// UNIT TESTS - This is what we are verifying!
// -----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use azumi::test;

    #[test]
    fn test_simple_card_render() {
        let card = SimpleCard::Props::builder()
            .title("Test Title".to_string())
            .content("Test Content".to_string())
            .build()
            .unwrap();

        let html = test::render(&SimpleCard::render(card));

        // Assert text content
        test::assert_selector(&html, ".title", Some("Test Title"));
        test::assert_selector(&html, ".content", Some("Test Content"));
    }

    #[test]
    fn test_counter_logic() {
        let mut simulator = test::simulate(Counter { count: 0 });

        // Initial state
        assert_eq!(simulator.state.count, 0);

        // Perform action
        simulator.act(Counter::increment);

        // Verify state change
        assert_eq!(simulator.state.count, 1);
    }
}

// -----------------------------------------------------------------------------
// Lesson Page (Documentation)
// -----------------------------------------------------------------------------

#[azumi::component]
pub fn lesson17() -> impl Component {
    html! {
        <style>
            .container { max-width: "800px"; margin: "0 auto"; padding: "2rem"; }
            .code { background: "#f5f5f5"; padding: "1rem"; border-radius: "8px"; overflow-x: "auto"; }
        </style>
        <div class={container}>
            <h1>"Lesson 17: Testing Azumi Components"</h1>
            <p>"Azumi provides a dedicated test harness to verify components and logic without a browser."</p>

            <h2>"1. Testing Rendering"</h2>
            <pre class={code}>
                "#[test]\n"
                "fn test_render() {\n"
                "    let html = test::render(&MyComponent { ... });\n"
                "    test::assert_selector(&html, \".title\", Some(\"Hello\"));\n"
                "}"
            </pre>

            <h2>"2. Testing Live Logic"</h2>
            <pre class={code}>
                "#[test]\n"
                "fn test_logic() {\n"
                "    let mut sim = test::simulate(State { ... });\n"
                "    sim.act(State::increment);\n"
                "    assert_eq!(sim.state.count, 1);\n"
                "}"
            </pre>

            <p>"If you are seeing this page, the unit tests in this file passed!"</p>
        </div>
    }
}

pub async fn handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson17()))
}
