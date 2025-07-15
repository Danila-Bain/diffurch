//! `diffurch` is a library that implements numerical methods 
//! for ordinary and delay differential equations. 
//!
//! # Supported types of equations
//! - Continuous ODEs (Ordinary Differential Equations).
//! - Non-sliding discontinuous or impulse ODEs (*).
//! - Continuous or impulse (neutral) DDEs (Delay Differential Equations) with constant or variable non-vanishing delays.
//! - Non-sliding discontinuous (neutral) DDEs with constant or variable non-vanishing delays (*).
//!
//! (here (*) means that kind of equation is supported using an event 
//! system rather than by using discontinuous functions directly in equation)
//!
//! # Features
//! - Event detection
//! - Event filtering
//! - Delay-induced discontinuity propagation
//! - Dense output
//! - Wide range of explicit Runge-Kutta methods with interpolation
//! - (not yet) Automatic step size control
//!
//! # Goals
//! - Peak performance: as much as possible is done at compile time, avoiding dynamic dispatch or
//! unnecessary runtime branching.
//! - Support for ODEs and DDEs, including neutral DDEs.
//! - Support for discontinuous, impulse, and hybrid equations.
//! - Ergonomics, minimal boilerplate: use of convenience macros to wrap closures and (not yet
//! there) simple symbolic system.
//!

#![feature(unboxed_closures, fn_traits, tuple_trait)]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]
#![feature(file_buffered)]
#![feature(allocator_api)]

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

#![warn(missing_docs)]
#![deny(rust_2018_idioms)]
#![deny(unused)]
#![deny(refining_impl_trait)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]

#![allow(clippy::module_name_repetitions)]

pub mod equation;
pub mod event;
pub mod filter;
pub mod initial_condition;
pub mod loc;
pub mod polynomial;
pub mod rk;
pub mod solver;
pub mod state;
pub mod collections;

pub use equation::*;
pub use event::*;
pub use filter::*;
pub use initial_condition::*;
pub use loc::*;
pub use solver::*;
pub use state::*;
