pub mod rk;
pub mod polynomial;
pub mod initial_condition;
pub mod solver;
pub mod interval;
pub mod state;

pub use state::{StateFn, StateRef, StateRefMut};
pub use solver::{Solver};

// pub mod callback;
// pub mod filter;
// pub mod collections;
