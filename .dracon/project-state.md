# Project State

## Current Focus
Added authentication and authorization framework for Azumi

## Completed
- [x] Implemented trait-based authorization system with `HasCurrentUser` interface
- [x] Added `#[require_auth]` attribute support for action handlers
- [x] Created auth provider registration system with `register_auth_provider()`
- [x] Added comprehensive error handling with `AuthError` enum
- [x] Included example implementation for Axum extension-based auth
- [x] Added documentation for setup and usage patterns
- [x] Created internal helper functions for request processing
- [x] Added test cases for auth provider registration
