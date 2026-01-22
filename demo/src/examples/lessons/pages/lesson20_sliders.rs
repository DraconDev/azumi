use crate::examples::lessons::components::layout::DarkModernLayout;
use azumi::prelude::*;

#[azumi::live]
pub struct SliderState {
    pub value: i32,
}

#[azumi::live_impl(component = "slider_view")]
impl SliderState {
    pub fn update_value(&mut self) {
        // This is a placeholder for when we support input events
        // For now, we'll just increment to show it's alive
        self.value = (self.value + 1) % 101;
    }
}

#[azumi::component]
pub fn slider_view<'a>(state: &'a SliderState) -> impl Component + 'a {
    html! {
        <div class={slider_container}>
            <div class={card}>
                <h2 class={title}>"Premium Range Sliders"</h2>
                <p class={description}>"Customized sliders using the newly whitelisted 'appearance' property."</p>

                <div class={range_wrapper}>
                    <input
                        type="range"
                        min="0"
                        max="100"
                        value={state.value.to_string()}
                        class={modern_slider}
                    />
                    <div class={value_display}>
                        <span class={value_label}>"Value: "</span>
                        <span class={value_number}>{state.value}</span>
                    </div>
                </div>

                <div class={controls}>
                    <button class={btn} on:click={state.update_value}>
                        "Increment Value (Simulate Change)"
                    </button>
                </div>
            </div>
        </div>

        <style>
            .slider_container {
                max-width: "600px";
                margin: "4rem auto";
                padding: "0 2rem";
            }

            .card {
                background: "rgba(15, 23, 42, 0.8)";
                backdrop-filter: "blur(16px)";
                border: "1px solid rgba(99, 102, 241, 0.2)";
                border-radius: "24px";
                padding: "3rem";
                box-shadow: "0 25px 50px -12px rgba(0, 0, 0, 0.5)";
            }

            .title {
                margin: "0 0 0.5rem 0";
                font-size: "2rem";
                font-weight: "800";
                background: "linear-gradient(to right, #6366f1, #a855f7)";
                -webkit-background-clip: "text";
                -webkit-text-fill-color: "transparent";
            }

            .description {
                margin: "0 0 3rem 0";
                color: "#94a3b8";
                font-size: "1.1rem";
                line-height: "1.6";
            }

            .range_wrapper {
                margin: "2rem 0";
            }

            /* The Modern Slider Styling */
            .modern_slider {
                appearance: "none";
                -webkit-appearance: "none";
                width: "100%";
                height: "12px";
                background: "rgba(30, 41, 59, 0.5)";
                border-radius: "100px";
                outline: "none";
                margin: "1rem 0";
                border: "1px solid rgba(255, 255, 255, 0.05)";
                cursor: "pointer";
            }

            /* Slider Thumb */
            /* Note: Azumi's current CSS parser might have issues with double-colon pseudos in some contexts,
               but standard properties like appearance: "none" are the core fix here. */
            /* We'll focus on the properties that were causing issues. */

            .value_display {
                display: "flex";
                justify-content: "center";
                align-items: "center";
                gap: "0.5rem";
                margin-top: "1rem";
            }

            .value_label {
                color: "#64748b";
                font-weight: "600";
                text-transform: "uppercase";
                letter-spacing: "0.05em";
                font-size: "0.875rem";
            }

            .value_number {
                color: "#6366f1";
                font-size: "1.5rem";
                font-weight: "700";
            }

            .controls {
                margin-top: "2.5rem";
                display: "flex";
                justify-content: "center";
            }

            .btn {
                padding: "0.75rem 1.5rem";
                background: "rgba(99, 102, 241, 0.1)";
                border: "1px solid rgba(99, 102, 241, 0.3)";
                color: "#818cf8";
                border-radius: "12px";
                font-weight: "600";
                cursor: "pointer";
                transition: "all 0.2s ease";
            }

            .btn:hover {
                background: "rgba(99, 102, 241, 0.2)";
                border-color: "rgba(99, 102, 241, 0.5)";
                transform: "translateY(-1px)";
            }
        </style>
    }
}

pub async fn lesson20_handler() -> impl axum::response::IntoResponse {
    let state = SliderState { value: 50 };
    use slider_view::Props;
    let page = slider_view::render(Props::builder().state(&state).build().expect("props"));

    // We wrap it in the layout manually for the handler
    let full_page = html! {
        @DarkModernLayout() {
            {page}
        }
    };

    axum::response::Html(azumi::render_to_string(&full_page))
}
