# Project State

## Current Focus
Enhanced security validation for `Raw()` usage in HTML templates with pattern-based allowlisting

## Completed
- [x] Refactored `Raw()` validation to detect suspicious patterns (format!, user input, etc.)
- [x] Added known-good patterns (azumi_script, trusted constants) that don't require explicit opt-in
- [x] Improved error messages with specific guidance for safe vs unsafe `Raw()` usage
- [x] Updated documentation references to point to the security guide for `Raw()` usage
