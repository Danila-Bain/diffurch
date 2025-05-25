//! Defines symbols
//!
//! This module will allow the following syntax to apply (or somewhat close to it)
//! ```rust
//!
//! // let (t, [x, y, z]) = Simbolic::state();
//!
//! // let eq = [y, -x(t - 1)];
//! // let ic = [t.sin(), t.cos()];
//!
//! // Solver::new()
//!   //   .on_step(event!([t, x, y, z]).to_std())
//!     // .on_loc(x.powi(2) - 1. == 0, event_mut!())
//!
//!
//! ```

use crate::State;
use eager2::{eager, eager_macro, unstringify};

pub trait StateSymbol
where
    Self: Sized,
{
    fn eval<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64;
    fn eval_at<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>, t: f64) -> f64;
    fn eval_prev<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64;

    fn dt(self) -> impl StateSymbol;
}

pub struct Constant(f64);
impl StateSymbol for Constant {
    fn eval<'a, const N: usize, const S: usize>(&self, _state: &State<'a, N, S>) -> f64 {
        self.0
    }

    fn eval_at<'a, const N: usize, const S: usize>(
        &self,
        _state: &State<'a, N, S>,
        _t: f64,
    ) -> f64 {
        self.0
    }

    fn eval_prev<'a, const N: usize, const S: usize>(&self, _state: &State<'a, N, S>) -> f64 {
        self.0
    }

    fn dt(self) -> impl StateSymbol {
        Constant(0.)
    }
}

pub struct Time;
impl StateSymbol for Time {
    fn eval<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64 {
        state.t
    }
    fn eval_at<'a, const N: usize, const S: usize>(&self, _state: &State<'a, N, S>, t: f64) -> f64 {
        t
    }
    fn eval_prev<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64 {
        state.t_prev
    }

    fn dt(self) -> impl StateSymbol {
        Constant(1.)
    }
}

pub struct Coord(usize);
impl StateSymbol for Coord {
    fn eval<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64 {
        state.x[self.0]
    }
    fn eval_at<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>, t: f64) -> f64 {
        state.eval(t, self.0)
    }
    fn eval_prev<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64 {
        state.x_prev[self.0]
    }

    fn dt(self) -> impl StateSymbol {
        CoordDerivative(self.0)
    }
}

pub struct CoordDerivative(usize);
impl StateSymbol for CoordDerivative {
    fn eval<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64 {
        state.eval_derivative(state.t, self.0)
    }
    fn eval_at<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>, t: f64) -> f64 {
        state.eval_derivative(t, self.0)
    }
    fn eval_prev<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64 {
        state.eval_derivative(state.t_prev, self.0)
    }

    fn dt(self) -> impl StateSymbol {
        todo!();
        Constant(0.)
    }
}

pub struct Sin<Arg>(Arg);
impl<Arg: StateSymbol> StateSymbol for Sin<Arg> {
    fn eval<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64 {
        self.0.eval(state).sin()
    }

    fn eval_at<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>, t: f64) -> f64 {
        self.0.eval_at(state, t).sin()
    }

    fn eval_prev<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64 {
        self.0.eval_prev(state).sin()
    }

    fn dt(self) -> impl StateSymbol {
        todo!();
        Constant(0.)
    }
}


#[eager_macro]
macro_rules! impl_func1 {
    ($func:ident, $struct:ident) => {
        pub struct $struct<Arg>(Arg);
        impl<Arg: StateSymbol> StateSymbol for $struct<Arg> {
            fn eval<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64 {
                self.0.eval(state).$func()
            }

            fn eval_at<'a, const N: usize, const S: usize>(
                &self,
                state: &State<'a, N, S>,
                t: f64,
            ) -> f64 {
                self.0.eval_at(state, t).$func()
            }

            fn eval_prev<'a, const N: usize, const S: usize>(
                &self,
                state: &State<'a, N, S>,
            ) -> f64 {
                self.0.eval_prev(state).$func()
            }

            fn dt(self) -> impl StateSymbol {
                // $derivative(self.0) * self.0.dt()
                Constant(0.)
            }
        }
    };
}

#[eager_macro]
macro_rules! impl_funcs1 {
    ($(($func:ident, $derivative:expr)),* $(,)?) => {
            $( impl_func1!($func, unstringify!(concat!("SF", stringify!($func)))))*
    };
}

#[eager_macro]
macro_rules! funcs1 {
    () => {
        // (recip, |t| -t.powi(2).recip()),
        //
        // (acos, |t| -(1. - t.powi(2)).sqrt().recip()),
        // (asin, |t| (1. - t.powi(2)).sqrt().recip()),
        // (acosh, |t| (t.powi(2) - 1.).sqrt().recip()),
        // (asinh, |t| (t.powi(2) + 1.).sqrt().recip()),
        // (atan, |t| (1. + t.powi(2)).recip()),
        // (atanh, |t| (1. - t.powi(2)).recip()),

        (cos, |t| -t.sin()),
        (sin, |t| t.cos()),
        // (cosh, |t| t.sinh()),
        // (sinh, |t| t.cosh()),
        // (tan, |t| t.cos().powi(2).recip()),
        // (tanh, |t| t.cosh().powi(2).recip()),

        // (exp, |t| t.exp()),
        // (exp2, |t| t.exp2() * f64::ln(2.)),
        // (exp_m1, |t| t.exp()),
        // (ln, |t| t.recip()),
        // (ln_1p, |t| (1.+t).recip()),
        // (ln2, |t| (t * f64::ln(2.)).recip()),
        // (ln10, |t| (t * f64::ln(10.)).recip()),
        //
        // (erf, |t| (-t.powi(2)).exp() * (2. / f64::const::PI.sqrt())),
        // (erfc, |t| -(-t.powi(2)).exp() * (2. / f64::const::PI.sqrt())),
        //
        // (sqrt, |t| (2. * t.sqrt()).recip()),
        // (cbrt, |t| (3. * t.cbrt().powi(2)).recip()),
        //
        // (to_radians, |_| f64::const::PI/180.),
        // (to_degrees, |_| 180./f64::const::PI),
    };
}

// AAA!((cosh, |t| t.sinh()), (sinh, |t| t.cosh()));

eager! { impl_funcs1!(funcs1!()) }

// fn f() {
// let x = eager!{add!(numbers!())};
// }

// Functions with no formula for derivative
// gamma
// ln_gamma

// Functions of two variables
// log
// powi
// powf
// atan2
// hypot
// midpoint
// mul_add
// powf
//

// Non-smooth functions, that require events
// abs
// ceil !
// clamp !
// floor
// fract
// max
// maximum
// rem_euclid
// round
// round_ties_event
// signum
// trunc
//
// For them, signum is a discrete variable eventfull version, while signum_unchecked is not
