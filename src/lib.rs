// pub mod delay;
pub mod initial_condition;
pub mod interval;
pub mod loc;
// pub mod polynomial;
pub mod rk;
pub mod solver;
pub mod state;
pub mod traits;

mod util;

pub use loc::{Loc, periodic::Periodic};
pub use solver::Solver;
pub use state::{StateFn, StateRef, StateRefMut};

pub use rk::*;
pub type RK<T, const S: usize, const I: usize> = ButcherTableu<T, S, I>;

// pub mod callback;
// pub mod filter;
