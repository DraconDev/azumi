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
//!     // Extract from your auth middleware's Extension
//!     let user = req.extensions()
//!         .get::<Extension<User>>()
//!         .ok_or(AuthError::NotAuthenticated)?;
//!     Ok(user.id.clone())
//! };
//!
//! // Register at startup
//! azumi::auth::register_auth_provider(auth_extractor);
//! ```

use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::future::Future;
use std::pin::Pin;

/// Authorization error types
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

/// Result type for auth operations
pub type AuthResult<T> = Result<T, AuthError>;

/// Type alias for the auth extractor function
pub type AuthExtractor =
    for<'a> fn(req: &'a axum::http::Request<axum::body::Body>) -> AuthResult<String>;

/// Marker function for when no auth provider is registered
fn no_auth_provider(_req: &axum::http::Request<axum::body::Body>) -> AuthResult<String> {
    Err(AuthError::Internal(
        "No auth provider registered. Call azumi::auth::register_auth_provider() at startup.",
    ))
}

// Global auth provider storage
static AUTH_PROVIDER: std::sync::OnceLock<AuthExtractor> = std::sync::OnceLock::new();

/// Register your auth extractor at startup.
///
/// Call this once in your `main()` function before handling requests:
///
/// ```ignore
/// use axum::{extract::Extension, http::Request, body::Body};
/// use azumi::auth::{AuthError, AuthResult};
///
/// // Your User type
/// #[derive(Clone)]
/// pub struct User { pub id: String }
///
/// // Create an auth extractor closure
/// let auth_extractor = |req: &Request<Body>| -> AuthResult<String> {
///     let user = req.extensions()
///         .get::<Extension<User>>()
///         .ok_or(AuthError::NotAuthenticated)?;
///     Ok(user.id.clone())
/// };
///
/// fn main() {
///     azumi::auth::register_auth_provider(auth_extractor);
///     // ... rest of app setup
/// }
/// ```
pub fn register_auth_provider(extractor: AuthExtractor) {
    AUTH_PROVIDER
        .set(extractor)
        .map_err(|_| ())
        .expect("auth::register_auth_provider() called multiple times");
}

/// Get the registered auth provider.
///
/// Used internally by generated handlers.
pub fn get_auth_provider() -> AuthExtractor {
    AUTH_PROVIDER.get().copied().unwrap_or(no_auth_provider)
}

/// Internal helper to extract user from a request.
///
/// This is called by generated handlers when `#[require_auth]` is present.
pub fn extract_user_from_request(
    req: &axum::http::Request<axum::body::Body>,
) -> AuthResult<String> {
    get_auth_provider()(req)
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

/// Result type for auth operations
pub type AuthResult<T> = Result<T, AuthError>;

/// Trait for extracting user identity from requests.
///
/// Implement this trait to bridge your authentication system (cookies, JWT,
/// sessions, etc.) to Azumi's authorization system.
///
/// # Example Implementation
///
/// ```ignore
/// use axum::{extract::Extension, http::Request};
/// use azumi::auth::{HasCurrentUser, AuthResult};
///
/// struct MyAuth;
///
/// impl HasCurrentUser for MyAuth {
///     fn get_user_id(req: &Request<Body>) -> AuthResult<String> {
///         // Your auth middleware should have inserted User into extensions
///         let Extension(user) = req.extensions()
///             .get::<Extension<Option<User>>>()
///             .cloned()
///             .unwrap_or(Extension(None));
///
///         match user {
///             Some(u) => Ok(u.id.clone()),
///             None => Err(AuthError::NotAuthenticated),
///         }
///     }
/// }
/// ```
pub trait HasCurrentUser: Send + Sync + 'static {
    /// Extract the current user's ID from the request.
    ///
    /// Return `Ok(user_id)` if authenticated, `Err(AuthError::NotAuthenticated)` if not.
    ///
    /// Note: The framework will automatically reject with 401 if you return `Err`.
    fn get_user_id(req: &Request<axum::body::Body>) -> AuthResult<String>;
}

// This needs to use the Body type from axum
use axum::body::Body;

/// Marker type for when no auth provider is registered
pub struct NoAuthProvider;

impl HasCurrentUser for NoAuthProvider {
    fn get_user_id(_req: &Request<Body>) -> AuthResult<String> {
        Err(AuthError::Internal(
            "No auth provider registered. Implement HasCurrentUser and call \
             azumi::auth::register_auth_provider::<YourAuthType>() at startup.",
        ))
    }
}

// Global auth provider storage
static AUTH_PROVIDER: std::sync::OnceLock<Box<dyn HasCurrentUser>> = std::sync::OnceLock::new();

/// Register your auth provider at startup.
///
/// Call this once in your `main()` function before handling requests:
///
/// ```ignore
/// #[derive(Clone)]
/// struct MyAuth;
///
/// impl azumi::auth::HasCurrentUser for MyAuth {
///     fn get_user_id(req: &Request<Body>) -> AuthResult<String> {
///         // Your extraction logic
///     }
/// }
///
/// fn main() {
///     azumi::auth::register_auth_provider(MyAuth);
///     // ... rest of app setup
/// }
/// ```
pub fn register_auth_provider<T: HasCurrentUser>(provider: T) {
    AUTH_PROVIDER
        .set(Box::new(provider))
        .map_err(|_| ())
        .expect("auth::register_auth_provider() called multiple times");
}

/// Get the registered auth provider.
///
/// Used internally by generated handlers.
pub fn get_auth_provider() -> &'static dyn HasCurrentUser {
    AUTH_PROVIDER
        .get()
        .map(|p| p.as_ref())
        .unwrap_or(&NoAuthProvider)
}

/// Internal helper to extract user from a request.
///
/// This is called by generated handlers when `#[require_auth]` is present.
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
