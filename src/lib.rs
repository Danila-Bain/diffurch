pub mod rk;
pub mod polynomial;
pub mod initial_condition;
pub mod solver;
pub mod interval;
pub mod state;
pub mod loc;

mod util;

pub use state::{StateFn, StateRef, StateRefMut};
pub use solver::{Solver};
pub use loc::{Loc, periodic::Periodic};

// pub mod callback;
// pub mod filter;
