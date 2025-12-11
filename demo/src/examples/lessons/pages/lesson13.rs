use azumi::prelude::*;

/// Lesson 13: Form Handling
///
/// Building forms with Azumi Live

#[azumi::live]
pub struct ContactForm {
    pub submitted: bool,
}

#[azumi::live_impl(component = "contact_form_view")]
impl ContactForm {
    pub fn submit(&mut self) {
        self.submitted = true;
    }

    pub fn reset(&mut self) {
        self.submitted = false;
    }
}

/// Contact form component
#[azumi::component]
pub fn contact_form_view<'a>(state: &'a ContactForm) -> impl Component + 'a {
    html! {

        <div class={form_container}>
            <h2 class={form_title}>"📧 Contact Form"</h2>

            @if state.submitted {
                <div class={success_box}>
                    <div class={success_icon}>"✅"</div>
                    <div class={success_text}>"Thank you for your message!"</div>
                </div>
                <button class={btn btn_secondary} on:click={state.reset} >
                    "Send Another"
                </button>
            }

            @if !state.submitted {
                <div class={field}>
                    <label class={label}>"Name"</label>
                    <input class={input} type="text" name="name" placeholder="Your name" />
                </div>
                <div class={field}>
                    <label class={label}>"Email"</label>
                    <input class={input} type="email" name="email" placeholder="your@email.com" />
                </div>
                <div class={field}>
                    <label class={label}>"Message"</label>
                    <textarea class={textarea} name="message" placeholder="Your message..."></textarea>
                </div>
                <button class={btn} type="button" on:click={state.submit}>
                    "Submit"
                </button>
            }
        </div>
        <style>
            .form_container {
                max-width: "400px";
                padding: "2rem";
                background: "white";
                border-radius: "12px";
                border: "1px solid #e0e0e0";
            }
            .form_title {
                margin-bottom: "1.5rem";
                color: "#333";
            }
            .field {
                display: "grid";
                gap: "0.5rem";
                margin-bottom: "1rem";
            }
            .label {
                font-weight: "bold";
                color: "#555";
            }
            .input {
                padding: "0.75rem";
                border: "1px solid #ddd";
                border-radius: "6px";
                font-size: "1rem";
            }
            .textarea {
                padding: "0.75rem";
                border: "1px solid #ddd";
                border-radius: "6px";
                font-size: "1rem";
                min-height: "100px";
                resize: "vertical";
            }
            .btn {
                padding: "0.75rem 1.5rem";
                background: "#2196f3";
                color: "white";
                border: "none";
                border-radius: "6px";
                font-size: "1rem";
                cursor: "pointer";
                width: "100%";
            }
            .btn_secondary {
                background: "#757575";
            }
            .success_box {
                padding: "2rem";
                text-align: "center";
                background: "#e8f5e9";
                border-radius: "8px";
            }
            .success_icon {
                font-size: "3rem";
                margin-bottom: "1rem";
            }
            .success_text {
                color: "#2e7d32";
                font-size: "1.2rem";
                font-weight: "bold";
            }
        </style>
    }
}

use crate::examples::lessons::components::layout::DarkModernLayout;

/// Full page component ensuring script injection
#[azumi::component]
pub fn lesson13_page<'a>(state: &'a ContactForm) -> impl Component + 'a {
    html! {
        @DarkModernLayout() {
            <div class={container}>
                <header class={header}>
                    <h1 class={main_title}>"Lesson 13: Form Handling"</h1>
                    <p class={subtitle}>"Building forms with Azumi Live"</p>
                </header>

                <div class={explanation}>
                    <h3 class={exp_title}>"📝 Form Patterns"</h3>
                    <ul class={exp_list}>
                        <li class={exp_item}><strong>"Submit action"</strong>" - Toggles submitted state"</li>
                        <li class={exp_item}><strong>"Reset action"</strong>" - Clears form state"</li>
                        <li class={exp_item}><strong>"Conditional rendering"</strong>" - Shows form or success message"</li>
                    </ul>
                </div>

                <div class={demo_area}>
                    @contact_form_view(state = state)
                </div>
            </div>
        }
        <style>
            .container { max-width: "800px"; margin: "0 auto"; }
            .header { text-align: "center"; margin-bottom: "3rem"; }
            .main_title {
                font-size: "3rem";
                font-weight: "800";
                color: "#e2e8f0";
                background: "linear-gradient(to right, #fbbf24, #f59e0b)";
                -webkit-background-clip: "text";
                -webkit-text-fill-color: "transparent";
                margin-bottom: "1rem";
            }
            .subtitle { font-size: "1.25rem"; color: "#94a3b8"; }

            .explanation {
                background: "rgba(30, 41, 59, 0.4)";
                padding: "2rem";
                border-radius: "16px";
                margin: "0 auto 3rem";
                border: "1px solid rgba(255,255,255,0.05)";
                max-width: "600px";
            }
            .exp_title { color: "#f59e0b"; font-size: "1.25rem"; margin-bottom: "1rem"; }
            .exp_list { color: "#cbd5e1"; padding-left: "1.5rem"; display: "flex"; flex-direction: "column"; gap: "0.5rem"; }
            .exp_item { line-height: "1.6"; }

            .demo_area { display: "flex"; justify-content: "center"; margin: "2rem 0"; }
        </style>
    }
}

// Handler for Axum
pub async fn lesson13_handler() -> impl axum::response::IntoResponse {
    let form_state = ContactForm { submitted: false };
    use lesson13_page_component::Props;
    let page = lesson13_page_component::render(
        Props::builder().state(&form_state).build().expect("props"),
    );
    axum::response::Html(azumi::render_to_string(&page))
}
