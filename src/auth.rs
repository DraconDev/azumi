//! Authentication and Authorization Framework
//!
//! Azumi provides a closure-based authorization system that integrates with your
//! existing authentication middleware (cookies, JWT, sessions, etc.).
//!
//! # Setup
//!
//! 1. Implement `HasCurrentUser` trait for your auth type
//! 2. Register it with `azumi::auth::register_auth_provider()`
//! 3. Use `#[require_auth]` on actions that need authentication
//!
//! # Example
//!
//! ```ignore
//! use axum::{extract::Extension, http::Request, body::Body};
//! use azumi::auth::{AuthError, AuthResult, HasCurrentUser};
//!
//! // Your User type
//! #[derive(Clone)]
//! pub struct User {
//!     pub id: String,
//!     pub role: String,
//! }
//!
//! // Your auth implementation
//! pub struct MyAuth;
//!
//! impl HasCurrentUser for MyAuth {
//!     fn get_user_id(req: &Request<Body>) -> AuthResult<String> {
//!         let Extension(user) = req.extensions()
//!             .get::<Extension<Option<User>>>()
//!             .cloned()
//!             .unwrap_or(Extension(None));
//!
//!         match user {
//!             Some(u) => Ok(u.id.clone()),
//!             None => Err(AuthError::NotAuthenticated),
//!         }
//!     }
//! }
//!
//! // Register at startup
//! azumi::auth::register_auth_provider(MyAuth);
//! ```

use axum::body::Body;
use axum::http::Request;
use axum::http::StatusCode;
use axum::response::IntoResponse;

#[derive(Debug, Clone)]
pub enum AuthError {
    NotAuthenticated,
    Forbidden,
    Internal(&'static str),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AuthError::NotAuthenticated => {
                (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
            }
            AuthError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden").into_response(),
            AuthError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
        }
    }
}

pub type AuthResult<T> = Result<T, AuthError>;

pub trait HasCurrentUser: Send + Sync + 'static {
    fn get_user_id(req: &Request<Body>) -> AuthResult<String>;
}

pub struct NoAuthProvider;

impl HasCurrentUser for NoAuthProvider {
    fn get_user_id(_req: &Request<Body>) -> AuthResult<String> {
        Err(AuthError::Internal(
            "No auth provider registered. Implement HasCurrentUser and call \
             azumi::auth::register_auth_provider() at startup.",
        ))
    }
}

static AUTH_PROVIDER: std::sync::OnceLock<Box<dyn HasCurrentUser>> = std::sync::OnceLock::new();

pub fn register_auth_provider<T: HasCurrentUser>(provider: T) {
    AUTH_PROVIDER
        .set(Box::new(provider))
        .map_err(|_| ())
        .expect("auth::register_auth_provider() called multiple times");
}

pub fn get_auth_provider() -> &'static dyn HasCurrentUser {
    AUTH_PROVIDER
        .get()
        .map(|p| p.as_ref())
        .unwrap_or(&NoAuthProvider)
}

pub fn extract_user_from_request(req: &Request<Body>) -> AuthResult<String> {
    get_auth_provider().get_user_id(req)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;

    #[test]
    fn test_no_auth_provider_error() {
        let req = Request::new(Body::empty());
        let result = NoAuthProvider.get_user_id(&req);
        assert!(matches!(result, Err(AuthError::Internal(_))));
    }
}
