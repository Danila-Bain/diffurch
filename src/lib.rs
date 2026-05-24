#![allow(clippy::excessive_precision)]

pub mod initial_condition;
pub mod interval;
pub mod loc;
pub mod rk;
pub mod solver;
pub mod state;
pub mod stepsize;
pub mod traits;

mod util;

pub use initial_condition::InitFn;
pub use loc::{Filter, Locator, periodic::Periodic};
pub use solver::Solver;
pub use state::{StateFn, StateRef, StateRefMut};
pub use stepsize::AutomaticStepsize;

pub use rk::*;
pub type RK<T, const S: usize, const I: usize> = ButcherTableu<T, S, I>;

pub use derive_state::State;
