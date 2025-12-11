use azumi::prelude::*;

/// Lesson 11: Async Loading Patterns
///
/// Demonstrates how to handle loading and error states for async operations.
/// Key concept: Use `loading: bool` and `error: Option<String>` in your state.

#[azumi::live]
pub struct UserLoader {
    pub loading: bool,
    pub error: Option<String>,
    pub users: Vec<String>,
}

#[azumi::live_impl(component = "user_loader_view")]
impl UserLoader {
    pub async fn load_users(&mut self) {
        // 1. Optimistic Update: Set loading=true instantly
        self.loading = true;
        self.error = None;

        // 2. Real Async Delay (non-blocking!)
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        // 3. Update State
        self.users = vec![
            "Alice Chen".to_string(),
            "Bob Smith".to_string(),
            "Charlie Kim".to_string(),
        ];
        self.loading = false;
    }

    pub async fn load_fail(&mut self) {
        self.loading = true;
        self.error = None;

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        self.loading = false;
        self.error = Some("Network timeout: Could not reach user database.".to_string());
        self.users.clear();
    }

    pub fn reset(&mut self) {
        self.loading = false;
        self.error = None;
        self.users.clear();
    }
}

#[azumi::component]
pub fn user_loader_view<'a>(state: &'a UserLoader) -> impl Component + 'a {
    html! {


        <div class={container}>
            <div class={card}>
                <div class={header}>
                    <h1>"Async Data Loading"</h1>
                    <p>"Click actions to see optimistic loading states."</p>
                </div>

                // ===============================================
                // The Pattern: Logic-less View Switching
                // ===============================================

                @if state.loading {
                    <div class={loading_state}>
                        <div class={spinner}></div>
                        <p>"Fetching users from database..."</p>
                    </div>
                } else {
                    @if state.error.is_some() {
                        <div class={error_state}>
                            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <circle cx="12" cy="12" r="10"></circle>
                                <line x1="12" y1="8" x2="12" y2="12"></line>
                                <line x1="12" y1="16" x2="12.01" y2="16"></line>
                            </svg>
                            <div>
                                <strong>"Error Occurred"</strong>
                                <p>{state.error.as_ref().unwrap()}</p>
                            </div>
                        </div>
                    } else {
                        @if state.users.is_empty() {
                            <div class={empty_state}>
                                "No users loaded. Ready to fetch."
                            </div>
                        } else {
                            <ul class={user_list}>
                                @for user in &state.users {
                                    <li class={user_item}>
                                        <div class={avatar}>{&user[0..1]}</div>
                                        {user}
                                    </li>
                                }
                            </ul>
                        }
                    }
                }

                <div class={controls}>
                    <button class={btn_primary} on:click={state.load_users}>
                        "Load Users (Success)"
                    </button>
                    <button class={btn_danger} on:click={state.load_fail}>
                        "Load Users (Fail)"
                    </button>
                    <button class={btn_outline} on:click={state.reset}>
                        "Reset"
                    </button>
                </div>
            </div>
        </div>
        <style>
            .container { max-width: "600px"; margin: "2rem auto"; padding: "2rem"; }
            .card {
                background: "white"; border-radius: "12px";
                background: "white"; border-radius: "12px";
                // box_shadow: "0 4px 6px -1px rgba(0, 0, 0, 0.1)";
                padding: "2rem"; border: "1px solid #e5e7eb";
            }
            .header { text-align: "center"; margin-bottom: "2rem"; }

            /* Loading State */
            .loading_state { text-align: "center"; padding: "3rem"; color: "#6b7280"; }
            .spinner {
                display: "inline-block"; width: "40px"; height: "40px";
                border: "3px solid #e5e7eb"; border-top-color: "#4f46e5";
                border-radius: "50%"; animation: "spin 1s linear infinite";
                margin-bottom: "1rem";
            }
            @keyframes spin { to { transform: "rotate(360deg)"; } }

            /* Error State */
            .error_state {
                background: "#fef2f2"; color: "#991b1b";
                padding: "1rem"; border-radius: "8px";
                display: "flex"; align_items: "center"; gap: "0.5rem";
                margin-bottom: "1rem";
            }

            /* Data State */
            .user_list { list-style: "none"; padding: "0"; }
            .user_item {
                display: "flex"; align_items: "center"; gap: "1rem";
                padding: "1rem"; border-bottom: "1px solid #f3f4f6";
            }
            .avatar {
                width: "40px"; height: "40px"; background: "#e0e7ff";
                color: "#4f46e5"; border-radius: "50%";
                display: "flex"; align-items: "center"; justify-content: "center";
                font-weight: "bold";
            }

            /* Controls */
            .controls {
                display: "flex"; gap: "1rem"; justify-content: "center";
                margin-top: "2rem"; padding-top: "2rem"; border-top: "1px solid #e5e7eb";
            }
            .btn_primary {
                padding: "0.75rem 1.5rem"; border-radius: "6px"; font-weight: "500";
                border: "none"; cursor: "pointer"; transition: "all 0.2s";
                background: "#4f46e5"; color: "white";
            }
            .btn_primary:hover { background: "#4338ca"; }
            .btn_danger {
                padding: "0.75rem 1.5rem"; border-radius: "6px"; font-weight: "500";
                border: "none"; cursor: "pointer"; transition: "all 0.2s";
                background: "#ef4444"; color: "white";
            }
            .btn_danger:hover { background: "#dc2626"; }
            .btn_outline {
                padding: "0.75rem 1.5rem"; border-radius: "6px"; font-weight: "500";
                border: "none"; cursor: "pointer"; transition: "all 0.2s";
                background: "white"; border: "1px solid #d1d5db"; color: "#374151";
            }
            .btn_outline:hover { background: "#f9fafb"; }
            .empty_state { text-align: "center"; color: "#6b7280"; padding: "2rem"; }
        </style>
    }
}

pub async fn lesson11_handler() -> axum::response::Html<String> {
    let state = UserLoader {
        loading: false,
        error: None,
        users: vec![],
    };

    use user_loader_view_component::*;
    let component_html = azumi::render_to_string(&render(
        Props::builder().state(&state).build().expect("props"),
    ));

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Lesson 11: Loading Patterns</title>
    <style>body {{ font-family: system-ui; background: #f3f4f6; margin: 0; }}</style>
</head>
<body>
    {}
    <script src="/static/idiomorph.js"></script>
    <script src="/static/azumi.js"></script>
</body>
</html>"#,
        component_html
    );
    axum::response::Html(html)
}
