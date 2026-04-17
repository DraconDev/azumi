# Project State

## Current Focus
Added security validation for `Raw()` usage in HTML templates to prevent XSS vulnerabilities

## Completed
- [x] Implemented `validate_raw_usage()` function to detect and warn about potentially unsafe `Raw()` patterns
- [x] Added detection for suspicious patterns like `format!` with user-controlled data
- [x] Included comprehensive warning message with security guidance and reference to documentation
- [x] Enhanced token parsing to recursively check nested expressions for `Raw()` calls
