use axum::{extract::Request, middleware::Next, response::Response, Router};

/// Returns the router for Azumi development tools
/// currently includes the hot reload websocket endpoint
pub fn router() -> Router {
    crate::hot_reload::router()
}

/// Middleware to force no-cache headers in development mode
/// usage: .layer(axum::middleware::from_fn(azumi::devtools::no_cache_middleware))
pub async fn no_cache_middleware(req: Request, next: Next) -> Response {
    let mut response = next.run(req).await;

    // Only set headers if we are in debug mode
    #[cfg(debug_assertions)]
    {
        let headers = response.headers_mut();
        // Prevent caching for all responses
        headers.insert(
            "Cache-Control",
            "no-cache, no-store, must-revalidate".parse().unwrap(),
        );
        headers.insert("Pragma", "no-cache".parse().unwrap());
        headers.insert("Expires", "0".parse().unwrap());
    }

    response
}
