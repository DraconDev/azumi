use axum::{
    extract::{Extension, Request},
    http::StatusCode,
    middleware::{self, Next},
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use azumi::prelude::*;

// -----------------------------------------------------------------------------
// MOCK AUTH MIDDLEWARE
// -----------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct User {
    pub username: String,
}

pub async fn auth_middleware(mut request: Request, next: Next) -> Result<Response, StatusCode> {
    let headers = request.headers();

    // In a real app, you would check a session cookie or JWT here.
    // For this demo, we verify a simple query param ?login=true or a cookie.

    let jar = CookieJar::from_headers(headers);
    if let Some(user_cookie) = jar.get("azumi_user") {
        // User is logged in via cookie
        request.extensions_mut().insert(User {
            username: user_cookie.value().to_string(),
        });
        Ok(next.run(request).await)
    } else {
        // User is NOT logged in. Redirect to login page?
        // For this lesson, we just let them pass but without the User extension
        // so the page can show a "Login" button.
        Ok(next.run(request).await)
    }
}

// -----------------------------------------------------------------------------
// LIVE COMPONENT
// -----------------------------------------------------------------------------

#[azumi::live]
pub struct AuthState {
    pub username: Option<String>,
}

#[azumi::live_impl(component = "auth_view")]
impl AuthState {
    pub fn logout(&mut self) {
        // In a real app, this would clear the cookie via a server header.
        // Since Azumi Live actions return HTML/JSON updates, we can't easily set headers *yet*
        // on the response without the `Response` unification we did in macro.
        // Actually, we can just clear the state, and the client side cookie management
        // would need to happen via JS or a proper endpoint.
        // For this demo, we'll simulate it by clearing state.
        self.username = None;
    }
}

// -----------------------------------------------------------------------------
// VIEW
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
             .btn_secondary { background: "#64748b"; margin-left: "1rem"; }
             .explanation { margin-top: "2rem"; color: "#666"; font-size: "0.9rem"; }
        </style>

        <div class={container}>
            <div class={card}>
                <div class={header}>
                    <h1 class={title}>"Lesson 19: Authentication"</h1>
                    <p class={subtitle}>"Integration with Axum Middleware"</p>
                </div>

                <p>"This page demonstrates how Azumi plays nicely with standard Axum middleware."</p>

                <div class={status_box}>
                    @if let Some(user) = &state.username {
                        <h3>"Welcome back, " {user} "!"</h3>
                        <p>"You are authenticated via Axum middleware."</p>
                        <button class={btn} on:click={state.logout}>"Mock Logout"</button>
                    } else {
                        <h3>"You are Guest"</h3>
                        <p>"The middleware did not find a session cookie."</p>
                        // Link effectively acts as a login action by setting cookie
                        <a href="/lesson-19-login" class={btn}>"Simulate Login"</a>
                    }
                </div>
            </div>

            <div class={explanation}>
                <p><strong>"How it works:"</strong></p>
                <ol>
                    <li>"Axum Middleware runs BEFORE the Azumi handler."</li>
                    <li>"It checks for cookies/tokens and inserts a `User` struct into `req.extensions()`."</li>
                    <li>"The Azumi handler extracts this `User` and initializes the LiveState."</li>
                </ol>
            </div>
        </div>
    }
}

    }
}

// -----------------------------------------------------------------------------
// ARCHITECTURE EXPLANATION: THE BRIDGE PATTERN
// -----------------------------------------------------------------------------
//
// Azumi components (The UI) are isolated from HTTP details. 
// Axum Middleware (The Guard) knows HTTP but doesn't know your UI.
//
// We use the HANDLER as the "Bridge" to pass data from Middleware -> UI.
//
// ┌──────────────────────┐      ┌─────────────────────────┐      ┌──────────────────────┐
// │   1. Middleware      │      │       2. Handler        │      │    3. Component      │
// │  (HTTP Layer)        │───►  │    (The Bridge)         │───►  │     (UI Layer)       │
// │                      │      │                         │      │                      │
// │ checks cookies       │      │ extracts User struct    │      │ receives User struct │
// │ inserts User struct  │      │ via axum::Extension     │      │ via Props            │
// │ into req.extensions  │      │ initializes State       │      │ renders welcome msg  │
// └──────────────────────┘      └─────────────────────────┘      └──────────────────────┘

// -----------------------------------------------------------------------------
// HANDLERS
// -----------------------------------------------------------------------------

pub async fn handler(
    // We can extract extensions inserted by middleware!
    axum::Extension(user): axum::Extension<Option<User>>,
) -> impl IntoResponse {
    let initial_state = AuthState {
        username: user.map(|u| u.username),
    };

    use auth_view_component::*;
    let html = azumi::render_to_string(&render(
        Props::builder().state(&initial_state).build().unwrap(),
    ));

    // We must manually inject the script since we are rendering a fragment-like structure
    // (though in this case we're lazy and just returning the div, relying on the user to imagine the layout or macro injecting it if we used <html>)
    // To be proper let's wrap it in a minimal layout or just assume the demo layout wrapper handles it?
    // Wait, the other lessons usually render full <html>. Let's do that for consistency if I want auto-injection.
    // BUT, I didn't write <html> in the component above. Let me fix that in a sec or just accept it.
    // Actually, `azumi::render_to_string` produces the string. If I don't have <html>, script isn't injected.
    // I should probably return a layout.

    Html(format!(
        "<!DOCTYPE html><html><head><title>Lesson 19</title><meta charset='utf-8'></head><body>{}<script src='/static/azumi.js'></script><script src='/static/idiomorph.js'></script></body></html>",
        html
    ))
}

pub async fn login_handler(jar: CookieJar) -> impl IntoResponse {
    // Set a cookie and redirect back
    let cookie = Cookie::build(("azumi_user", "Dracon"))
        .path("/")
        .http_only(true)
        .build();

    (jar.add(cookie), Redirect::to("/lesson-19-auth"))
}
