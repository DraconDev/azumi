// Example: Async Loading Pattern with Existing Azumi Syntax
// This shows how to handle async data fetching with loading/error states
// WITHOUT any new macro syntax - just using existing @if

use azumi::prelude::*;

// ============================================
// Pattern: Using existing @if for loading states
// ============================================

#[azumi::live]
pub struct UserList {
    pub loading: bool,
    pub error: Option<String>,
    pub users: Vec<String>,
}

#[azumi::live_impl(component = "user_list_view")]
impl UserList {
    /// Called when user clicks "Load Users"
    /// The prediction sets loading=true instantly
    /// Server fetches data and returns with loading=false
    pub fn load_users(&mut self) {
        self.loading = true;
        self.error = None;

        // In real app, this would be async fetch
        // For demo, we simulate success
        self.users = vec![
            "Alice".to_string(),
            "Bob".to_string(),
            "Charlie".to_string(),
        ];
        self.loading = false;
    }

    /// Simulate an error scenario
    pub fn load_with_error(&mut self) {
        self.loading = true;
        self.error = None;

        // Simulate failure
        self.loading = false;
        self.error = Some("Failed to fetch users".to_string());
        self.users.clear();
    }

    /// Reset state
    pub fn reset(&mut self) {
        self.loading = false;
        self.error = None;
        self.users.clear();
    }
}

#[azumi::component]
pub fn user_list_view<'a>(state: &'a UserList) -> impl Component + 'a {
    html! {
        <style>
            .container {
                padding: "2rem";
                max-width: "500px";
                border: "1px solid #e0e0e0";
                border-radius: "12px";
            }
            .loading {
                text-align: "center";
                padding: "2rem";
                color: "#666";
            }
            .spinner {
                display: "inline-block";
                width: "40px";
                height: "40px";
                border: "4px solid #f3f3f3";
                border-top: "4px solid #667eea";
                border-radius: "50%";
                animation: "spin 1s linear infinite";
            }
            .error {
                background: "#ffebee";
                color: "#c62828";
                padding: "1rem";
                border-radius: "8px";
                margin: "1rem 0";
            }
            .user_list {
                list-style: "none";
                padding: "0";
            }
            .user_item {
                padding: "0.75rem";
                background: "#f5f5f5";
                margin: "0.5rem 0";
                border-radius: "4px";
            }
            .btn_row {
                display: "flex";
                gap: "0.5rem";
                margin-top: "1rem";
            }
            .btn {
                padding: "0.5rem 1rem";
                border: "none";
                border-radius: "4px";
                cursor: "pointer";
            }
            .btn_primary { background: "#667eea"; color: "white"; }
            .btn_danger { background: "#f44336"; color: "white"; }
            .btn_secondary { background: "#9e9e9e"; color: "white"; }
        </style>

        <div class="container">
            <h2>"👥 User List"</h2>

            // ============================================
            // THIS IS THE PATTERN: @if for loading/error/success
            // No new syntax needed!
            // ============================================

            @if state.loading {
                // Loading state
                <div class={loading}>
                    <div class={spinner}></div>
                    <p>"Loading users..."</p>
                </div>
            } else if state.error.is_some() {
                // Error state
                <div class={error}>
                    "❌ " {state.error.as_ref().unwrap()}
                </div>
            } else if state.users.is_empty() {
                // Empty state
                <p>"No users loaded yet. Click a button below."</p>
            } else {
                // Success state - show data
                <ul class={user_list}>
                    @for user in &state.users {
                        <li class={user_item}>"👤 " {user}</li>
                    }
                </ul>
            }

            // Actions
            <div class={btn_row}>
                <button class={btn btn_primary} on:click={state.load_users}>
                    "Load Users"
                </button>
                <button class={btn btn_danger} on:click={state.load_with_error}>
                    "Simulate Error"
                </button>
                <button class={btn btn_secondary} on:click={state.reset}>
                    "Reset"
                </button>
            </div>
        </div>
    }
}

// ============================================
// Summary:
//
// Current syntax (works today):
//   @if state.loading { <Spinner /> }
//   else @if state.error.is_some() { <Error /> }
//   else { <Content /> }
//
// Benefits:
//   - No new macro syntax needed
//   - Explicit about state
//   - Type-safe (Rust checks the fields exist)
//   - Predictions work for loading = true
//
// The only "new" thing we'd add:
//   - Pattern documentation
//   - Maybe a helper: LoadingState<T> enum
// ============================================
