# Project State

## Current Focus
Enhanced CSS scoping to properly handle functional pseudo-classes containing `:root`

## Completed
- [x] Added test cases for `:is()`, `:where()`, `:not()`, and `:has()` pseudo-classes containing `:root`
- [x] Implemented special handling to preserve `:root` when nested inside functional pseudo-classes
- [x] Ensured scoped selectors maintain correct structure while excluding `:root` from scoping
