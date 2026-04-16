# Project State

## Current Focus
Improve HTML generation safety by switching from `Fn` to `FnOnce` closure handling

## Completed
- [x] Replace `from_fn` with `from_fn_once` in HTML macro to properly handle owned values
- [x] Add documentation explaining the change and its necessity for component props
```
