use crate::examples::lessons::components::layout::DarkModernLayout;
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
                    <h1 class={title}>"Async Data Loading"</h1>
                    <p class={subtitle}>"Click actions to see optimistic loading states."</p>
                </div>

                // ===============================================
                // The Pattern: Logic-less View Switching
                // ===============================================

                <div class={content_area}>
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
                </div>

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
            .container { max-width: "700px"; margin: "0 auto"; }
            .card {
                border: "1px solid rgba(255,255,255,0.05)";
                border-radius: "16px";
                padding: "2rem";
                background: "rgba(30, 41, 59, 0.6)";
                backdrop-filter: "blur(10px)";
                color: "#e2e8f0";
            }
            .header { text-align: "center"; margin-bottom: "2rem"; }
            .title { color: "#e2e8f0"; margin-bottom: "0.5rem"; font-size: "2rem"; }
            .subtitle { color: "#94a3b8"; font-size: "1rem"; }

            .content_area { min-height: "200px"; display: "flex"; flex-direction: "column"; justify-content: "center"; }

            /* Loading State */
            .loading_state { text-align: "center"; padding: "2rem"; color: "#94a3b8"; }
            .spinner {
                display: "inline-block"; width: "40px"; height: "40px";
                border: "3px solid rgba(255,255,255,0.1)"; border-top-color: "#818cf8";
                border-radius: "50%"; animation: "spin 1s linear infinite";
                margin-bottom: "1rem";
            }
            @keyframes spin { to { transform: "rotate(360deg)"; } }

            /* Error State */
            .error_state {
                background: "rgba(220, 38, 38, 0.2)"; color: "#fca5a5";
                padding: "1rem"; border-radius: "8px"; border: "1px solid rgba(220, 38, 38, 0.3)";
                display: "flex"; align_items: "center"; gap: "1rem";
                margin-bottom: "1rem";
            }

            /* Data State */
            .user_list { list-style: "none"; padding: "0"; display: "grid"; gap: "0.5rem"; }
            .user_item {
                display: "flex"; align_items: "center"; gap: "1rem";
                padding: "1rem"; border-bottom: "1px solid rgba(255,255,255,0.05)";
                background: "rgba(255,255,255,0.02)"; border-radius: "8px";
            }
            .avatar {
                width: "40px"; height: "40px"; background: "linear-gradient(to right, #6366f1, #818cf8)";
                color: "white"; border-radius: "50%";
                display: "flex"; align-items: "center"; justify-content: "center";
                font-weight: "bold"; font-size: "1.2rem";
            }

            /* Controls */
            .controls {
                display: "flex"; gap: "1rem"; justify-content: "center"; flex-wrap: "wrap";
                margin-top: "2rem"; padding-top: "2rem"; border-top: "1px solid rgba(255,255,255,0.1)";
            }
            .btn_primary {
                padding: "0.75rem 1.5rem"; border-radius: "8px"; font-weight: "600";
                border: "none"; cursor: "pointer"; transition: "all 0.2s";
                background: "linear-gradient(to right, #4f46e5, #4338ca)"; color: "white";
            }
            .btn_primary:hover { opacity: "0.9"; }
            .btn_danger {
                padding: "0.75rem 1.5rem"; border-radius: "8px"; font-weight: "600";
                border: "none"; cursor: "pointer"; transition: "all 0.2s";
                background: "linear-gradient(to right, #ef4444, #dc2626)"; color: "white";
            }
            .btn_danger:hover { opacity: "0.9"; }
            .btn_outline {
                padding: "0.75rem 1.5rem"; border-radius: "8px"; font-weight: "600";
                border: "1px solid rgba(255,255,255,0.1)"; cursor: "pointer"; transition: "all 0.2s";
                background: "transparent"; color: "#cbd5e1";
            }
            .btn_outline:hover { background: "rgba(255,255,255,0.05)"; color: "white"; }
            .empty_state { text-align: "center"; color: "#64748b"; padding: "2rem"; font-style: "italic"; }
        </style>
    }
}

#[azumi::component]
pub fn lesson11() -> impl azumi::Component {
    let state = UserLoader {
        loading: false,
        error: None,
        users: vec![],
    };

    html! {
        @DarkModernLayout() {
            @user_loader_view(state=&state)
        }
    }
}

pub async fn lesson11_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson11()))
}
