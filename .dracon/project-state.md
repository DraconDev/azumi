# Project State

## Current Focus
Enhanced security validation for `Raw()` usage in HTML templates with more precise pattern matching

## Completed
- [x] Added new known-good patterns: `SITE_BASE_CSS`, `session_cleanup`, `window.location.hash`
- [x] Simplified pattern matching by removing unnecessary `format!("{}(", pattern)` wrapper
- [x] Refined suspicious pattern detection with more specific checks:
  - `serde_json::to_string` instead of general `serde_json`
  - Combined checks for `user` + `input` or `request` + `body`
  - Removed standalone `.to_string()` check as it's not inherently suspicious
  - Kept `cookie` as a standalone suspicious pattern
