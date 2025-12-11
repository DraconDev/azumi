use crate::Component;
use axum::response::IntoResponse;

use std::future::Future;

/// Trait for Azumi Actions
/// This is implemented automatically by the `#[azumi::action]` macro
pub trait Action<Input, Output> {
    fn call(input: Input) -> impl Future<Output = Output> + Send;
}

use axum::routing::MethodRouter;

/// Registry entry for an action
pub struct ActionEntry {
    pub path: &'static str,
    pub handler: fn() -> MethodRouter<()>, // Simplified for now
}

inventory::collect!(ActionEntry);

/// Register all collected actions into the router
pub fn register_actions(mut router: axum::Router) -> axum::Router {
    for entry in inventory::iter::<ActionEntry> {
        router = router.route(entry.path, (entry.handler)());
    }
    router
}

/// Helper to wrap an action result into an Axum response
pub async fn handle_action_result<C: Component + ?Sized>(component: &C) -> impl IntoResponse {
    crate::render_to_string(component)
}
