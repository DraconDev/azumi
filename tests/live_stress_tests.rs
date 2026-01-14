use azumi::{html, test, Component};
use serde::{Deserialize, Serialize};

// ════════════════════════════════════════════════════════════════════════════
// Live State Setup
// ════════════════════════════════════════════════════════════════════════════

#[azumi::live]
#[derive(Serialize, Deserialize, Default)]
pub struct CounterState {
    pub count: i32,
    pub active: bool,
}

#[azumi::live_impl(component = "counter_view")]
impl CounterState {
    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn toggle(&mut self) {
        self.active = !self.active;
    }

    #[azumi::predict("count = 0")]
    pub fn reset(&mut self) {
        self.count = 0;
    }
}

#[azumi::component]
pub fn counter_view<'a>(state: &'a CounterState) -> impl Component + 'a {
    html! {
        <div>
            <span data-bind="count">{state.count}</span>
            <button on:click={state.increment}>"+1"</button>
            <button on:click={state.toggle}>"Toggle"</button>
            <button on:click={state.reset}>"Reset"</button>
        </div>
    }
}

// ════════════════════════════════════════════════════════════════════════════
// Tests
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_live_scope_and_struct_attributes() {
    let state = CounterState::default();
    let comp = html! { @counter_view(state = &state) };
    let output = test::render(&comp);

    // Check for az-scope and az-struct
    assert!(output.contains("az-scope=\""), "az-scope attribute missing");
    assert!(
        output.contains("az-struct=\"CounterState\""),
        "az-struct attribute missing"
    );
}

#[test]
fn test_automatic_predictions() {
    let state = CounterState::default();
    let comp = html! { @counter_view(state = &state) };
    let output = test::render(&comp);

    // increment() -> self.count += 1
    // The compiler should generate a prediction for this simple mutation.
    assert!(
        output.contains("data-predict=\"count = count + 1\""),
        "Automatic prediction for increment missing"
    );

    // toggle() -> self.active = !self.active
    assert!(
        output.contains("data-predict=\"active = !active\""),
        "Automatic prediction for toggle missing"
    );
}

#[test]
fn test_manual_predictions() {
    let state = CounterState::default();
    let comp = html! { @counter_view(state = &state) };
    let output = test::render(&comp);

    // reset() has #[azumi::predict("count = 0")]
    assert!(
        output.contains("data-predict=\"count = 0\""),
        "Manual prediction for reset missing"
    );
}

#[test]
fn test_event_binding_rendering() {
    let state = CounterState::default();
    let comp = html! { @counter_view(state = &state) };
    let output = test::render(&comp);

    // on:click={state.increment} -> az-on="click call increment"
    assert!(
        output.contains("az-on=\"click call increment\""),
        "Event binding for increment missing"
    );
}

#[test]
fn test_data_bind_attribute() {
    let state = CounterState::default();
    let comp = html! { @counter_view(state = &state) };
    let output = test::render(&comp);

    assert!(
        output.contains("data-bind=\"count\""),
        "data-bind attribute missing"
    );
}

// ════════════════════════════════════════════════════════════════════════════
// Complex Live State (Nested/Multiple)
// ════════════════════════════════════════════════════════════════════════════

#[azumi::live]
#[derive(Serialize, Deserialize, Default)]
pub struct NestedState {
    pub child: CounterState,
}

#[azumi::live_impl(component = "nested_view")]
impl NestedState {
    pub fn do_nothing(&mut self) {}
}

#[azumi::component]
fn nested_view<'a>(state: &'a NestedState) -> impl Component + 'a {
    html! {
        <div>
            @counter_view(state = &state.child)
            <button on:click={state.do_nothing}>"Action"</button>
        </div>
    }
}

#[test]
fn test_nested_live_scopes() {
    let state = NestedState::default();
    let comp = html! { @nested_view(state = &state) };
    let output = test::render(&comp);

    // Should have two az-scope attributes (one for NestedState, one for CounterState)
    let scope_count = output.matches("az-scope=\"").count();
    assert!(
        scope_count >= 2,
        "Expected at least 2 az-scope attributes, found {}",
        scope_count
    );
    assert!(output.contains("az-struct=\"NestedState\""));
    assert!(output.contains("az-struct=\"CounterState\""));
}
