# Project State

## Current Focus
Improved event handler syntax validation and refactored page meta generation counter

## Completed
- [x] Enhanced HTML event handler validation in `html_structure_validator.rs` to suggest Azumi's `on:` syntax instead of native `on` handlers
- [x] Refactored `PageMetaGuard::new()` in `context.rs` to use `with` pattern for cleaner atomic counter access
