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
    let _container = "container";
    html! {
            <style>
                 .container { max-width: "800px"; margin: "0 auto"; padding: "2rem"; }
            </style>
            <div class={container}>
                <h1>"Lesson 18: Security"</h1>
                <p>"Azumi automatically signs state to prevent client-side tampering."</p>
                {
        secure_view_component::render(
            secure_view_component::Props::builder()
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
