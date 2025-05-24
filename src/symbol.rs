//! Defines symbols
//!
//! This module will allow the following syntax to apply (or somewhat close to it)
//! ```rust
//!
//! let (t, [x, y, z]) = Simbolic::state();
//!
//! let eq = [y, -x(t - 1)];
//! let ic = [t.sin(), t.cos()];
//!
//! Solver::new()
//!     .on_step(event!([t, x, y, z]).to_std())
//!     .on_loc(x.powi(2) - 1. == 0, event_mut!())
//!
//!
//! ```
