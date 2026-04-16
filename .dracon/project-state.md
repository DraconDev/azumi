# Project State

## Current Focus
Release v12.0.2 complete

## Release v12.0.2 Summary

### Security
- [x] User-scoped state signing (`sign_state_for_user` / `verify_state_for_user`)
- [x] Cross-user replay attack prevention
- [x] Backward compatible with existing `sign_state` / `verify_state`

### Robustness
- [x] Proper error handling for state deserialization (500 instead of panic)
- [x] Proper error handling for component rendering (500 instead of panic)
- [x] All 22 tests passing

### Removed (Correct Decision)
- [x] Removed `#[require_auth]` - application logic, not framework responsibility
- [x] Removed auth module - let users use Axum middleware/extensions

## Version History
- v12.0.2: Panic fixes in live.rs handlers
- v12.0.0: User-scoped state signing for replay protection
- Earlier: Initial implementation with HMAC integrity