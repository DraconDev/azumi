# Project State

## Current Focus
Improved thread safety guarantees for `FnOnceComponent` by requiring `Send + Sync` on the closure

## Completed
- [x] Made `FnOnceComponent` `Sync` only when closure is `Send + Sync` to prevent unsoundness with non-Sync captured types
- [x] Added documentation explaining thread safety requirements and implementation details
- [x] Updated Cargo.lock to reflect dependency resolution changes
