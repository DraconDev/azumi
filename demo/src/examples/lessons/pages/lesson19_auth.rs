use axum::{
    response::{Html, IntoResponse, Redirect},
    RequestPartsExt,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use azumi::prelude::*;

use super::super::components::auth_infra::{CurrentUser, User};

// -----------------------------------------------------------------------------
// 1. LIVE STATE
// -----------------------------------------------------------------------------
#[azumi::live]
pub struct AuthState {
    pub username: Option<String>,
}

#[azumi::live_impl(component = "auth_view")]
impl AuthState {
    pub fn logout(&mut self) {
        self.username = None;
    }
}

// -----------------------------------------------------------------------------
// 2. COMPONENT
// -----------------------------------------------------------------------------
#[azumi::component]
fn auth_view<'a>(state: &'a AuthState) -> impl Component + 'a {
    html! {
        <style>
             .container { max-width: "600px"; margin: "4rem auto"; padding: "2rem"; font-family: "system-ui"; color: "#333"; }
             .card { background: "white"; border-radius: "12px"; box-shadow: "0 4px 6px -1px rgba(0,0,0,0.1)"; padding: "2rem"; border: "1px solid #e2e8f0"; }
             .header { text-align: "center"; margin-bottom: "2rem"; }
             .title { font-size: "1.8rem"; margin: "0 0 0.5rem 0"; color: "#1e293b"; }
             .subtitle { color: "#64748b"; }
             .status_box { background: "#f8fafc"; padding: "1.5rem"; border-radius: "8px"; text-align: "center"; margin-top: "1.5rem"; border: "1px solid #e2e8f0"; }
             .btn { background: "#2563eb"; color: "white"; border: "none"; padding: "0.75rem 1.5rem"; border-radius: "6px"; font-weight: "600"; cursor: "pointer"; text-decoration: "none"; display: "inline-block"; }
             .btn:hover { background: "#1d4ed8"; }
             .explanation { margin-top: "2rem"; color: "#666"; font-size: "0.9rem"; }
        </style>

        <div class={container}>
            <div class={card}>
                <div class={header}>
                    <h1 class={title}>"Lesson 19: Authentication"</h1>
                    <p class={subtitle}>"Simplified with Reusable Extractors"</p>
                </div>

                <div class={status_box}>
                    @if let Some(user) = &state.username {
                        <h3>"Welcome back, " {user} "!"</h3>
                        <p>"Authenticated via shared middleware."</p>
                        <button class={btn} on:click={state.logout}>"Mock Logout"</button>
                    } else {
                        <h3>"You are Guest"</h3>
                        <p>"No session found."</p>
                        <a href="/lesson-19-login" class={btn}>"Simulate Login"</a>
                    }
                </div>
            </div>

            <div class={explanation}>
                <p><strong>"How it works:"</strong></p>
                <ol>
                    <li>"Middleware validates cookies and sets `User` extension."</li>
                    <li>"Handler uses `CurrentUser` extractor (zero boilerplate)."</li>
                    <li>"State is initialized with user data."</li>
                </ol>
            </div>
        </div>
    }
}

// -----------------------------------------------------------------------------
// 3. HANDLER
// -----------------------------------------------------------------------------

// Look how clean this is! No traits, no complex imports.
// We just ask for `CurrentUser` from our infrastructure.
pub async fn handler(CurrentUser(user): CurrentUser) -> impl IntoResponse {
    let state = AuthState {
        username: user.map(|u| u.username),
    };

    use auth_view_component::*;
    let html = azumi::render_to_string(&render(Props::builder().state(&state).build().unwrap()));

    Html(format!(
        "<!DOCTYPE html><html><head><title>Lesson 19</title><meta charset='utf-8'></head><body>{}<script src='/static/azumi.js'></script><script src='/static/idiomorph.js'></script></body></html>",
        html
    ))
}

pub async fn login_handler(jar: CookieJar) -> impl IntoResponse {
    let cookie = Cookie::build(("azumi_user", "Dracon"))
        .path("/")
        .http_only(true)
        .build();
    (jar.add(cookie), Redirect::to("/lesson-19-auth"))
}
