//! Diffurch is a library that implements numerical methods for ordinary and delay differential
//! equations. It features a wery flexible event system for controlling the output of the solver, dense output, and event location.
#![feature(unboxed_closures, fn_traits, tuple_trait)]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]
#![feature(file_buffered)]
// #![warn(missing_docs)]

pub mod equation;
pub mod event;
pub mod initial_condition;
pub mod loc;
pub mod rk;
pub mod solver;
pub mod state;
pub mod filter;
pub mod polynomial;
pub mod fn_hlist;
//
pub use equation::*;
pub use event::*;
pub use initial_condition::*;
pub use loc::*;
pub use solver::*;
pub use state::*;
pub use filter::*;
pub use fn_hlist::*;
