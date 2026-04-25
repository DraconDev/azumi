# Project State

## Current Focus
Refactored script injection to use explicit `<script>` tags instead of implicit transformation

## Completed
- [x] Updated `live_component_demo.rs` to use explicit `<script src="/static/azumi.js">` instead of `azumi_script()` macro
- [x] Updated `live_demo.rs` to use `azumi_script()` macro instead of explicit `<script>` tag for consistency
