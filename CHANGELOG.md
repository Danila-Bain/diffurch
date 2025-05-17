# Version 0.0.1

Summary:
- minimal documentation is finished
- added convinience macros `event!`, `event_mut!`, and `equation!`
- filtering for `Loc`
- more examples
- bug fixes

## Documentation

Now it exists.

## Added
- Added `event!` and `event_mut!` macros for creating `Event`s from closures of different calling signatures
- Added `equation!` macro for creating `Equation` from closures of different calling signatures
- Added filtering to `Loc`, for filtered event detection

## Examples
- Lorenz system
- Chaotic billiard example
- Clamp DDE

## Fixes
- `Solver::new()`: fixed issue of not working template parameter inference
- `Solver`: fixed issue of panic("evaluation of not yet computed state) when using `Event::subdivide` due to the rounding errors
- For located events, now the earliest located event is chosen, and other are ignored for the step.

## Internal changes
- `Loc::locate` is now responcible for detection and returns `Option<f64>`, with `None` corresponding to undetected event.
- Move filtering functionality from `Event` into `Filter` trait
- `StateFn` is now holds `FnMut` instead of `Fn`
- `StateFnMut` is renamed to `MutStateFn`
- move `util::polynomial_macro` and `util::with_derivative` modules into `polynomial` module.

## Removed
- Removed unused `WithDerivative` trait
