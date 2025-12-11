use azumi::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub struct CounterState {
    pub count: i32,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ManagementState {
    pub status: String,
    pub count: i32,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct CompositionState {
    pub message: String,
    pub step: i32,
}

/// Lesson 8: Action System Deep Dive
///
/// Server-side interactivity patterns
/// Server-side interactivity patterns
pub fn counter_display(state: CounterState) -> impl azumi::Component {
    html! {
        <div id={counter_box} class={counter} az-scope={serde_json::to_string(&state).unwrap_or_default()}>
            <div class={count_display}>{state.count}</div>
            <button class={counter_button} az-on={click call increment_counter -> #counter_box}>
                "Increment"
            </button>
            <div class={timestamp}>"Last updated: 12:00:00"</div>
        </div>
        <style>
            .counter { padding: "2rem"; text-align: "center"; border: "1px solid #eee"; }
            .count_display { font-size: "3rem"; margin: "1rem 0"; }
            .counter_button { padding: "1rem 2rem"; background: "#4caf50"; color: "white"; border: "none"; cursor: "pointer"; }
            .timestamp { font-size: "0.8rem"; color: "#666"; }
            #counter_box { display: "block"; }
        </style>
    }
}

#[azumi::action]
pub async fn increment_counter(state: CounterState) -> impl azumi::Component {
    let new_state = CounterState {
        count: state.count + 1,
    };
    counter_display(new_state)
}

/// Example: Action with state management
/// Example: Action with state management
pub fn state_management_example(state: ManagementState) -> impl azumi::Component {
    html! {

        <div id={state_box} class={state_container} az-scope={serde_json::to_string(&state).unwrap_or_default()}>
            <h3>"State Management"</h3>

            <div class={state_info}>
                <p>"Current State: " {state.status}</p>
                <p>"Counter: " {state.count}</p>
            </div>

            <button class={action_button} az-on={click call update_state -> #state_box}>
                "Update State"
            </button>
        </div>
        <style>
            .state_container { padding: "1.5rem"; background: "#f9f9f9"; }
            .state_info { margin: "0.5rem 0"; padding: "0.5rem"; background: "white"; }
            .action_button { padding: "0.75rem 1.5rem"; background: "#2196f3"; color: "white"; border: "none"; cursor: "pointer"; }
            #state_box { display: "block"; }
        </style>
    }
}

#[azumi::action]
pub async fn update_state(state: ManagementState) -> impl azumi::Component {
    let new_count = state.count + 1;
    let new_status = if new_count % 2 == 0 {
        "Active"
    } else {
        "Processing"
    };
    let new_state = ManagementState {
        status: new_status.to_string(),
        count: new_count,
    };
    state_management_example(new_state)
}

/// Example: Action composition
/// Example: Action composition
pub fn action_composition_example(state: CompositionState) -> impl azumi::Component {
    html! {

        <div id={composition_box} class={composition_container} az-scope={serde_json::to_string(&state).unwrap_or_default()}>
            <h3>"Action Composition"</h3>

            <div class={action_card}>
                <p>"Message: " {state.message}</p>
                <p>"Step: " {state.step}</p>
            </div>

            <button class={compose_button} az-on={click call compose_actions -> #composition_box}>
                "Compose Actions"
            </button>
        </div>
        <style>
            .composition_container { padding: "1.5rem"; }
            .action_card { margin: "0.5rem 0"; padding: "1rem"; background: "#f5f5f5"; border: "1px solid #eee"; }
            .compose_button { padding: "0.75rem 1.5rem"; background: "#ff4081"; color: "white"; border: "none"; cursor: "pointer"; }
            #composition_box { display: "block"; }
        </style>
    }
}

#[azumi::action]
pub async fn compose_actions(state: CompositionState) -> impl azumi::Component {
    let new_step = state.step + 1;
    let new_message = format!("Action composed at step {}", new_step);
    let new_state = CompositionState {
        message: new_message,
        step: new_step,
    };
    action_composition_example(new_state)
}

/// Main lesson demonstration component
#[azumi::component]
pub fn lesson8() -> impl azumi::Component {
    let counter_state = CounterState { count: 0 };
    let management_state = ManagementState {
        status: "Active".to_string(),
        count: 0,
    };
    let composition_state = CompositionState {
        message: "Initial State".to_string(),
        step: 0,
    };

    html! {
        <script src="/static/azumi.js"></script>

        <div class={container}>
            <header class={header}>
                <h1 class={main_title}>"Lesson 8: Action System Deep Dive"</h1>
                <p class={subtitle}>"Server-side interactivity patterns"</p>
            </header>

            <section class={key_points}>
                <h2 class={section_title}>"Key Concepts"</h2>
                <ul class={points_list}>
                    <li class={point}>"✅ Server-side actions for interactivity"</li>
                    <li class={point}>"✅ State management patterns"</li>
                    <li class={point}>"✅ Action composition"</li>
                    <li class={point}>"✅ Type-safe action parameters"</li>
                    <li class={point}>"✅ Compile-time action validation"</li>
                </ul>
            </section>

            <section class={examples}>
                <div class={example_card}>
                    { counter_display(counter_state) }
                </div>
                <div class={example_card}>
                    { state_management_example(management_state.clone()) }
                </div>
                <div class={example_card}>
                    { action_composition_example(composition_state.clone()) }
                </div>
            </section>
        </div>
        <style>
            .container { padding: "20px"; }
            .header { text-align: "center"; margin-bottom: "30px"; }
            .main_title { font-size: "32px"; color: "#333"; }
            .subtitle { font-size: "18px"; color: "#666"; }
            .key_points { background: "#f9f9f9"; padding: "20px"; border-radius: "8px"; margin-bottom: "30px"; }
            .section_title { font-size: "20px"; margin-bottom: "15px"; }
            .points_list { list-style: "none"; padding: "0"; }
            .point { margin-bottom: "10px"; }
            .examples { display: "grid"; gap: "20px"; }
            .example_card { border: "1px solid #ddd"; padding: "20px"; border-radius: "8px"; }
        </style>
    }
}

// Handler for Axum
pub async fn lesson8_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson8()))
}
