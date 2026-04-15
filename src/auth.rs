//! Authentication and Authorization Framework
//!
//! Azumi provides a closure-based authorization system that integrates with your
//! existing authentication middleware (cookies, JWT, sessions, etc.).
//!
//! # Setup
//!
//! 1. Create a closure that extracts user ID from a request
//! 2. Register it with `azumi::auth::register_auth_provider()`
//! 3. Use `#[require_auth]` on actions that need authentication
//!
//! # Example
//!
//! ```ignore
//! use axum::{extract::Extension, http::Request, body::Body};
//! use azumi::auth::{AuthError, AuthResult};
//!
//! // Your User type
//! #[derive(Clone)]
//! pub struct User {
//!     pub id: String,
//!     pub role: String,
//! }
//!
//! // Create an auth extractor closure
//! let auth_extractor = |req: &Request<Body>| -> AuthResult<String> {
//!     let Extension(user) = req.extensions()
//!         .get::<Extension<Option<User>>>()
//!         .cloned()
//!         .unwrap_or(Extension(None));
//!
//!     match user {
//!         Some(u) => Ok(u.id.clone()),
//!         None => Err(AuthError::NotAuthenticated),
//!     }
//! };
//!
//! // Register at startup
//! azumi::auth::register_auth_provider(auth_extractor);
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

pub type AuthExtractor = for<'a> fn(req: &'a Request<Body>) -> AuthResult<String>;

fn no_auth_provider(_req: &Request<Body>) -> AuthResult<String> {
    Err(AuthError::Internal(
        "No auth provider registered. Call azumi::auth::register_auth_provider() at startup.",
    ))
}

static AUTH_PROVIDER: std::sync::OnceLock<AuthExtractor> = std::sync::OnceLock::new();

pub fn register_auth_provider(extractor: AuthExtractor) {
    AUTH_PROVIDER
        .set(extractor)
        .map_err(|_| ())
        .expect("auth::register_auth_provider() called multiple times");
}

pub fn get_auth_provider() -> AuthExtractor {
    AUTH_PROVIDER.get().copied().unwrap_or(no_auth_provider)
}

pub fn extract_user_from_request(req: &Request<Body>) -> AuthResult<String> {
    get_auth_provider()(req)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;

    #[test]
    fn test_no_auth_provider_error() {
        let req = Request::new(Body::empty());
        let result = no_auth_provider(&req);
        assert!(matches!(result, Err(AuthError::Internal(_))));
    }
}
