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

    #[test]
    fn test_signed_state_generation() {
        let state = SecureCounter {
            count: 10,
            is_admin: false,
        };

        // This fails if the macro doesn't generate sign_state()
        let scope = state.to_scope();

        println!("Generated Scope: {}", scope);

        // Structure should be JSON|BASE64
        assert!(
            scope.contains('|'),
            "Scope string must contain signature separator '|'"
        );

        let parts: Vec<&str> = scope.split('|').collect();
        assert_eq!(parts.len(), 2, "Scope string must have exactly two parts");

        // Verify we can parse the JSON part
        let json = parts[0];
        assert!(json.contains("\"count\":10"));

        // Verify the security module accepts it
        let verified = azumi::security::verify_state(&scope);
        assert!(verified.is_ok(), "Generated state failed verification");
        assert_eq!(verified.unwrap(), json);
    }

    #[test]
    fn test_tamper_attempt() {
        let state = SecureCounter {
            count: 10,
            is_admin: false,
        };
        let signed = state.to_scope();

        // Attacker tries to make themselves admin!
        let tampered = signed.replace("false", "true");

        println!("Tampered Scope: {}", tampered);

        // Verification MUST fail
        let verified = azumi::security::verify_state(&tampered);
        assert!(
            verified.is_err(),
            "Tampered state was accepted! Security FAILED."
        );
    }
}

#[azumi::component]
pub fn lesson18() -> impl Component {
    let initial_state = SecureCounter {
        count: 0,
        is_admin: false,
    };
    let container = "container";
    html! {
            <style>
                 .container { max-width: "800px"; margin: "0 auto"; padding: "2rem"; }
            </style>
            <div class={container}>
                <h1>"Lesson 18: Security"</h1>
                <p>"Azumi automatically signs state to prevent client-side tampering."</p>
                {
        secure_view::render(
            secure_view::Props::builder()
                .state(&initial_state)
                .build()
                .unwrap()
        )
    }
            </div>
        }
}

pub async fn handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson18()))
}
