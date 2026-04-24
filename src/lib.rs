// pub mod delay;
pub mod initial_condition;
pub mod interval;
pub mod loc;
// pub mod polynomial;
pub mod rk;
pub mod solver;
pub mod state;
pub mod stepsize;
pub mod traits;

mod util;

pub use initial_condition::InitFn;
pub use loc::{Filter, Loc, periodic::Periodic};
pub use solver::Solver;
pub use state::{StateFn, StateRef, StateRefMut};
pub use stepsize::AutomaticStepsize;

pub use rk::*;
pub type RK<T, const S: usize, const I: usize> = ButcherTableu<T, S, I>;

// pub mod callback;
// pub mod filter;

pub use loc::detect::{
    AboveZero, All, BelowZero, IsFalse, IsTrue, Negative, Positive, Switch, SwitchFalse,
    SwitchTrue, Zero,
};
