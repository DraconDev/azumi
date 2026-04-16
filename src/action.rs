use crate::Component;
use axum::response::IntoResponse;
use axum::routing::get;

use std::future::Future;

/// Trait for Azumi Actions
/// This is implemented automatically by the `#[azumi::action]` macro
#[allow(dead_code)]
pub trait Action<Input, Output> {
    fn call(input: Input) -> impl Future<Output = Output> + Send;
}

use axum::routing::MethodRouter;

/// Registry entry for an action
pub struct ActionEntry {
    pub path: &'static str,
    pub handler: fn() -> MethodRouter<()>,
}

inventory::collect!(ActionEntry);

/// Register all collected actions into the router.
/// Also registers the `/azumi.js` route to serve the client runtime.
pub fn register_actions(mut router: axum::Router) -> axum::Router {
    for entry in inventory::iter::<ActionEntry> {
        router = router.route(entry.path, (entry.handler)());
    }
    router.route("/azumi.js", get(azumi_js_handler))
}

/// Handler that serves the embedded Azumi client JavaScript
async fn azumi_js_handler() -> impl IntoResponse {
    (
        [(axum::http::header::CONTENT_TYPE, "application/javascript")],
        crate::AZUMI_JS,
    )
}

/// Helper to wrap an action result into an Axum response with correct Content-Type
pub async fn handle_action_result<C: Component + ?Sized>(component: &C) -> impl IntoResponse {
    axum::response::Html(crate::render_to_string(component))
}
