# Project State

## Current Focus
Refactored component macro to support snake_case function names and improve module naming consistency

## Completed
- [x] Modified component macro to check for snake_case function names
- [x] Updated module naming to use the original function name for snake_case components
- [x] Added wrapper function generation for snake_case components without required props/children
- [x] Updated live.rs to use the original component name for Axum handler generation
- [x] Maintained backward compatibility for non-snake_case component naming
