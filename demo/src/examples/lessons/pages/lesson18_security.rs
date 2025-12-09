use axum::response::{Html, IntoResponse};
use azumi::prelude::*;

/// Live component with secure state
#[azumi::live]
pub struct SecureCounter {
    pub count: i32,
    pub is_admin: bool,
}

#[azumi::live_impl(component = "secure_view")]
impl SecureCounter {
    pub fn increment(&mut self) {
        self.count += 1;
    }
}

#[azumi::component]
fn secure_view<'a>(state: &'a SecureCounter) -> impl Component + 'a {
    let count_class = "count";
    let admin_status = if state.is_admin {
        "Admin Mode"
    } else {
        "User Mode"
    };

    html! {
        <div>
            <h2>"Security Verification"</h2>
            <p>"If you inspect the DOM, you will see a signature in the az-scope attribute."</p>
            <p>"State: " {admin_status}</p>

            <div class={count_class}>
                "Count: " {state.count}
            </div>

            <button on:click={state.increment}>"Secure Increment"</button>
        </div>
    }
}

// -----------------------------------------------------------------------------
// VERIFICATION TEST
// -----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_implicit_security_rejection() {
        let state = SecureCounter {
            count: 10,
            is_admin: false,
        };
        let signed_scope = state.to_scope();

        // 1. Verify normal request works (Implicitly signed)
        let response = __azumi_live_handlers::increment_handler(signed_scope.clone()).await;
        assert_eq!(
            response.status(),
            StatusCode::OK,
            "Valid signed state should be accepted"
        );

        // 2. Simulate Attacker trying to modify state client-side
        // They try to change is_admin to true without updating the signature
        let tampered_scope = signed_scope.replace("false", "true");

        println!("Tampered Scope: {}", tampered_scope);

        // 3. Verify the handler REJECTS it automatically
        // We didn't write any verification code in SecureCounter, but the macro provided it.
        let response = __azumi_live_handlers::increment_handler(tampered_scope).await;
        assert_eq!(
            response.status(),
            StatusCode::BAD_REQUEST,
            "Tampered state should be rejected automatically"
        );
    }
}

#[azumi::component]
pub fn lesson18() -> impl Component {
    let initial_state = SecureCounter {
        count: 0,
        is_admin: false,
    };

    html! {
        <html>
        <head>
            <meta charset="utf-8" />
            <title>"Lesson 18: Security"</title>
            <style>
                .body { font-family: "system-ui"; background: "#f8fafc"; margin: "0"; color: "#334155"; }
                .container { max-width: "600px"; margin: "0 auto"; padding: "4rem 2rem"; }
                .card { background: "white"; padding: "2rem"; border-radius: "12px"; box-shadow: "0 4px 6px -1px rgb(0 0 0 / 0.1)"; border: "1px solid #e2e8f0"; }
                .header { text-align: "center"; margin-bottom: "2rem"; }
                .title { font-size: "2rem"; font-weight: "800"; color: "#0f172a"; margin: "0 0 0.5rem 0"; }
                .subtitle { color: "#64748b"; font-size: "1.1rem"; }

                .demo_section { margin-top: "2rem"; padding-top: "2rem"; border-top: "1px solid #e2e8f0"; }
                .info_box { background: "#eff6ff"; border-left: "4px solid #3b82f6"; padding: "1rem"; margin-bottom: "1.5rem"; border-radius: "4px"; }
                .info_text { margin: "0"; color: "#1e40af"; font-size: "0.95rem"; line-height: "1.5"; }
                .code_snippet { font-family: "monospace"; background: "#1e293b"; color: "#e2e8f0"; padding: "0.2rem 0.4rem"; border-radius: "4px"; font-size: "0.9em"; }

                .counter_display { text-align: "center"; margin: "2rem 0"; }
                .count_val { font-size: "4rem"; font-weight: "bold"; color: "#0f172a"; line-height: "1"; }
                .status_badge { display: "inline-block"; padding: "0.25rem 0.75rem"; border-radius: "9999px"; font-size: "0.875rem"; font-weight: "600"; margin-top: "1rem"; }
                .status_user { background: "#f1f5f9"; color: "#475569"; }
                .status_admin { background: "#dcfce7"; color: "#166534"; }

                .btn { display: "inline-flex"; align-items: "center"; justify-content: "center"; padding: "0.75rem 1.5rem"; background: "#0f172a"; color: "white"; font-weight: "600"; border-radius: "8px"; border: "none"; cursor: "pointer"; transition: "all 0.2s"; width: "100%"; font-size: "1rem"; }
                .btn:hover { background: "#1e293b"; transform: "translateY(-1px)"; }
                .btn:active { transform: "translateY(0)"; }

                .verify_steps { background: "#f8fafc"; padding: "1.5rem"; border-radius: "8px"; margin-top: "2rem"; border: "1px solid #e2e8f0"; }
                .verify_title { margin: "0 0 1rem 0"; font-size: "1rem"; color: "#0f172a"; }
                .step_list { margin: "0"; padding-left: "1.2rem"; }
                .step_item { margin-bottom: "0.5rem"; color: "#475569"; }
            </style>
        </head>
        <body class={body}>
            <div class={container}>
                <div class={card}>
                    <div class={header}>
                        <h1 class={title}>"Automatic Security"</h1>
                        <p class={subtitle}>"Signed State & Anti-Tampering"</p>
                    </div>

                    <div class={info_box}>
                        <p class={info_text}>
                            "Azumi automatically signs all component state with HMAC-SHA256. "
                            "If a malicious user tries to modify the JSON in "
                            <span class={code_snippet}>"az-scope"</span>
                            ", the server will reject the action."
                        </p>
                    </div>

                    {
                        secure_view_component::render(
                            secure_view_component::Props::builder()
                                .state(&initial_state)
                                .build()
                                .unwrap()
                        )
                    }

                    <div class={verify_steps}>
                        <h3 class={verify_title}>"How to Verify:"</h3>
                        <ol class={step_list}>
                            <li class={step_item}>"Inspect the button below in DevTools."</li>
                            <li class={step_item}>"Find the parent div with the <span class={code_snippet}>az-scope</span> attribute."</li>
                            <li class={step_item}>"The attribute contains formatted: <span class={code_snippet}>JSON|SIGNATURE</span>."</li>
                            <li class={step_item}>"Try changing <span class={code_snippet}>\"is_admin\":false</span> to <span class={code_snippet}>true</span>."</li>
                            <li class={step_item}>"Click 'Secure Increment' and watch the network request fail (400 Bad Request)."</li>
                        </ol>
                    </div>
                </div>
            </div>
            <script src="/static/idiomorph.js"></script>
            <script src="/static/azumi.js"></script>
        </body>
        </html>
    }
}

// Update secure_view to use new styles
#[azumi::component]
fn secure_view<'a>(state: &'a SecureCounter) -> impl Component + 'a {
    let counter_display = "counter_display";
    let count_val = "count_val";
    let btn = "btn";
    let status_badge = "status_badge";
    let status_user = "status_user";
    let status_admin = "status_admin";

    let badge_class = if state.is_admin {
        format!("{} {}", status_badge, status_admin)
    } else {
        format!("{} {}", status_badge, status_user)
    };

    let status_text = if state.is_admin {
        "Admin Access Unlocked"
    } else {
        "Standard User Access"
    };

    html! {
        <div>
            <div class={counter_display}>
                <div class={count_val}>{state.count}</div>
                <span class={badge_class}>
                    {status_text}
                </span>
            </div>

            <button class={btn} on:click={state.increment}>
                "Secure Increment"
            </button>
        </div>
    }
}

pub async fn handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson18()))
}
