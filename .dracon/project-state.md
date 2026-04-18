# Project State

## Current Focus
Enhanced HTML structure validation with stricter Raw() usage rules to block JavaScript content

## Completed
- [x] Updated test cases to verify Raw() usage is now blocked for JavaScript patterns
- [x] Added specific assertions to verify error messages contain "JavaScript content detected"
- [x] Refactored test names to clearly indicate the blocked patterns (azumi_script() and window.location)
