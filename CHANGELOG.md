# Version 0.0.3

- Add `state_fn!` and `mut_state_fn!` convenience macros.
- Make other convenience macros accept `_` for argument names in closures.
- Remove equation-type specific methods in `Event` and `Filter`
- In `StateCoordFn` class, replace use of `RKState` with `impl State`
- Add `prev` and `prev_d` methods for `StateCoordFn`s
- Add events to `Equation` struct, which are merged with `loc_events` in `Solver`. Add method `loc` to add located event without a callback (primary use: discontinuity handling), and `on_loc` which is similar to `Solver::on_loc`.
- Remove `Box` usage from `State::coord_fns`.
- Add `DDEMutStateFnMut`, which allows state-mutating events which have (non-mutating) access to history.
- Add `d_prev` method for a `State` for free evaluation of derivative of the state at the beginning of the last computed step.

# Version 0.0.2

- Major internal change: `Vec<dyn ...>` containers are replaced with static `hlist2::HList`. 
- Major internal change: for `rk::RungeKuttaTable::a`, type is changed from `[&[f64]; S]` to a flat array `[f64; S*(S-1)/2]`.
- When using library, `#![feature(generic_const_exprs)]` is a requirement on the user side (bummer).
- Bug fixes

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
