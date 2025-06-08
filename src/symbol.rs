//! Defines symbols

// use f64::*;

use crate::State;
use eager2::{eager, eager_macro};

#[eager_macro]
macro_rules! func_list {
    () => {
        (recip, t, -t.powi(2).recip(), [-2., -1., 1., 2.]),
        (acos, t, -(1. - t.powi(2)).sqrt().recip(), [-1., -0.5, 0.5, 1.]),
        (asin, t, (1. - t.powi(2)).sqrt().recip(), [-1., -0.5, 0.5, 1.]),
        (acosh, t, (t.powi(2) - 1.).sqrt().recip(), [1., 2., 3., 4.]),
        (asinh, t, (t.powi(2) + 1.).sqrt().recip(), [-1., 0., 1., 2.]),
        (atan, t, (1. + t.powi(2)).recip(), [-1., 0., 1., 2.]),
        (atanh, t, (1. - t.powi(2)).recip(), [-1., -0.5, 0.5, 1.]),
        (cos, t, -t.sin(), [-2., -1., 1., 2.]),
        (sin, t, t.cos(), [-2., -1., 1., 2.]),
        (cosh, t, t.sinh(), [-2., -1., 1., 2.]),
        (sinh, t, t.cosh(), [-2., -1., 1., 2.]),
        (tan, t, t.cos().powi(2).recip(), [-2., -1., 1., 2.]),
        (tanh, t, t.cosh().powi(2).recip(), [-2., -1., 1., 2.]),
        (exp, t, t.exp(), [-2., -1., 1., 2.]),
        (exp2, t, t.exp2() * f64::ln(2.), [-2., -1., 1., 2.]),
        (exp_m1, t, t.exp(), [-2., -1., 1., 2.]),
        (ln, t, t.recip(), [0., 1., 2., 3.]),
        (ln_1p, t, (t + 1.).recip(), [-1., 0., 1., 2.]),
        (log2, t, (t * f64::ln(2.)).recip(), [0., 1., 2., 3.]),
        (log10, t, (t * f64::ln(10.)).recip(), [0., 1., 2., 3.]),
        // (erf, t, (-t.powi(2)).exp() * (2. / std::f64::consts::PI.sqrt()), [-2., -1., 1., 2.]), // unstable
        // (erfc, t, -(-t.powi(2)).exp() * (2. / std::f64::consts::PI.sqrt())),
        (sqrt, t, (2. * t.sqrt()).recip(), [0., 1., 2., 3.]),
        (cbrt, t, (3. * t.cbrt().powi(2)).recip(), [-2., -1., 1., 2.]),
        (to_radians, _t, std::f64::consts::PI/180., [-2., -1., 1., 2.]),
        (to_degrees, _t, 180./std::f64::consts::PI, [-2., -1., 1., 2.]),
    };
}

macro_rules! impl_unary_op {
    ($struct:ident, $op:ident, $($Arg:ident),*) => {
        impl<$($Arg: Symbol),*> std::ops::$op for $struct<$($Arg),*> {
            type Output = $op<Self>;
            paste::paste! {
                fn [<$op:snake>](self) -> Self::Output {
                    $op(self)
                }
            }
        }
    }
}
macro_rules! impl_binary_op {
    ($struct:ident, $op:ident $(,)? $($Arg:ident),*) => {
        /// Self + Symbol
        impl<RHS: Symbol, $($Arg: Symbol),*> std::ops::$op<RHS> for $struct<$($Arg),*> {
            type Output = $op<Self, RHS>;
            paste::paste! {
                fn [<$op:snake>](self, rhs: RHS) -> Self::Output {
                    $op(self, rhs)
                }
            }
        }
        // f64 + Self
        impl<$($Arg: Symbol),*> std::ops::$op<$struct<$($Arg),*>> for f64 {
            type Output = $op<Constant, $struct<$($Arg),*>>;
            paste::paste! {
                fn [<$op:snake>](self, rhs: $struct<$($Arg),*>) -> Self::Output {
                    $op(Constant(self), rhs)
                }
            }
        }
        // Self + f64
        impl<$($Arg: Symbol),*> std::ops::$op<f64> for $struct<$($Arg),*> {
            type Output = $op<$struct<$($Arg),*>, Constant>;
            paste::paste! {
                fn [<$op:snake>](self, rhs: f64) -> Self::Output {
                    $op(self, Constant(rhs))
                }
            }
        }
    };
}
macro_rules! impl_ops {
    ($struct:ident $(,)? $($Arg:ident),*) => {
        impl_unary_op!($struct, Neg, $($Arg),*);
        impl_binary_op!($struct, Add, $($Arg),*);
        impl_binary_op!($struct, Sub, $($Arg),*);
        impl_binary_op!($struct, Mul, $($Arg),*);
        impl_binary_op!($struct, Div, $($Arg),*);
    };
}
#[eager_macro]
macro_rules! symbol_trait_methods {
    ($(($func:ident, $arg:ident, $derivative:expr, [$t:expr, $x:expr, $y:expr, $z:expr])),* $(,)?) => {
        $(eager2::lazy!{paste::paste!{
            fn $func (self) -> [<$func:camel>]<Self> {
                [<$func:camel>](self)
            }
        }})*
    };
}
#[eager_macro]
macro_rules! func_structs {
    ($(($func:ident, $arg:ident, $derivative:expr, [$t:expr, $x:expr, $y:expr, $z:expr])),* $(,)?) => { $( eager2::lazy!{ paste::paste!{
        #[doc = "Class implementing symbol function `" $func "` applied to `Arg`"]
        ///
        #[doc = concat!("
# Example
```rust
use diffurch::*;

let state = State::new(", stringify!($t), ", [", stringify!($x), ", ", stringify!($y), ", ", stringify!($z), "], f64::NAN, &rk::EULER);

let t = Time;
let [x, y, z] = Coord::vec();

assert_eq!(t.eval(&state), state.t);
assert_eq!(x.eval(&state), state.x[0]);
assert_eq!(y.eval(&state), state.x[1]);
assert_eq!(z.eval(&state), state.x[2]);
assert_eq!(t.", stringify!($func), "().eval(&state), state.t.", stringify!($func), "());
assert_eq!(x.", stringify!($func), "().eval(&state), state.x[0].", stringify!($func), "());
assert_eq!(y.", stringify!($func), "().eval(&state), state.x[1].", stringify!($func), "());
assert_eq!(z.", stringify!($func), "().eval(&state), state.x[2].", stringify!($func), "());macro_rules! first {
    ($first:tt $(, $rest:tt)*) => {
        $first
    };
}

let derivative = |", stringify!($arg), ": f64| ", stringify!($derivative), ";
assert_eq!(t.", stringify!($func), "().dt().eval(&state), derivative(state.t));
```
")]
        #[derive(Copy, Clone, Debug)]
        pub struct [<$func:camel>]<Arg>(Arg);
        impl_ops!([<$func:camel>], Arg);
        impl<Arg: Symbol> Symbol for [<$func:camel>]<Arg>
            where
                // Arg: std::ops::Add<f64, Output=Add<Arg, f64>>,
            //     Arg: std::ops::Sub<f64>,
            //     Arg: std::ops::Mul<f64>,
            //     Arg: std::ops::Div<f64>,
            //     f64: std::ops::Add<Arg>,
            //     f64: std::ops::Sub<Arg>,
            //     f64: std::ops::Mul<Arg>,
            //     f64: std::ops::Div<Arg>,
            {
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

            fn dt(self) -> impl Symbol {
                let $arg = self.0;
                Mul(($derivative).into_symbol(), self.0.dt())
            }
        }
    }})*};
}

/// # Example
/// ```rust
/// use diffurch::*;
///
/// let state = State::new(1., [2., 3., 4.], f64::NAN, &rk::EULER);
///
/// let t = Time;
/// let [x, y, z] = Coord::vec();
///
/// assert_eq!(t.eval(&state), 1.);
/// assert_eq!(x.eval(&state), 2.);
/// assert_eq!(y.eval(&state), 3.);
/// assert_eq!(z.eval(&state), 4.);
/// ```
///
pub trait Symbol
where
    Self: Sized,
    Self: Copy,
    Self: std::ops::Neg,
    Self: std::ops::Add<f64, Output = Add<Self, Constant>>,
    Self: std::ops::Sub<f64, Output = Sub<Self, Constant>>,
    Self: std::ops::Mul<f64, Output = Mul<Self, Constant>>,
    Self: std::ops::Div<f64, Output = Div<Self, Constant>>,
{
    fn eval<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64;
    fn eval_at<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>, t: f64) -> f64;
    fn eval_prev<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64;

    fn dt(self) -> impl Symbol;

    eager! { symbol_trait_methods!(func_list!())}

    fn powi(self, i: i32) -> Powi<Self> {
        Powi(self, i)
    }

    // fn neg(self) -> Neg<Self> {
    //     Neg(self)
    // }
}

pub trait IntoSymbol {
    fn into_symbol(self) -> impl Symbol;
}
impl<S: Symbol> IntoSymbol for S {
    fn into_symbol(self) -> impl Symbol {
        self
    }
}
impl IntoSymbol for f64 {
    fn into_symbol(self) -> impl Symbol {
        Constant(self)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Constant(pub f64);
impl Symbol for Constant {
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

    fn dt(self) -> impl Symbol {
        Constant(0.)
    }
}
impl_ops!(Constant);

// pub struct Constant(f64);
// impl Symbol for f64 {
//     fn eval<'a, const N: usize, const S: usize>(&self, _state: &State<'a, N, S>) -> f64 {
//         *self
//     }
//
//     fn eval_at<'a, const N: usize, const S: usize>(
//         &self,
//         _state: &State<'a, N, S>,
//         _t: f64,
//     ) -> f64 {
//         *self
//     }
//
//     fn eval_prev<'a, const N: usize, const S: usize>(&self, _state: &State<'a, N, S>) -> f64 {
//         *self
//     }
//
//     fn dt(self) -> impl Symbol {
//         0.
//     }
// }

#[derive(Copy, Clone, Debug)]
pub struct Time;
impl Symbol for Time {
    fn eval<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64 {
        state.t
    }
    fn eval_at<'a, const N: usize, const S: usize>(&self, _state: &State<'a, N, S>, t: f64) -> f64 {
        t
    }
    fn eval_prev<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64 {
        state.t_prev
    }

    fn dt(self) -> impl Symbol {
        Constant(1.)
    }
}
impl_ops!(Time);

#[derive(Copy, Clone, Debug)]
pub struct Coord(usize);
impl Symbol for Coord {
    fn eval<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64 {
        state.x[self.0]
    }
    fn eval_at<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>, t: f64) -> f64 {
        state.eval(t, self.0)
    }
    fn eval_prev<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64 {
        state.x_prev[self.0]
    }

    fn dt(self) -> impl Symbol {
        CoordDerivative(self.0)
    }
}
impl_ops!(Coord);

impl Coord {
    pub fn vec<const N: usize>() -> [Coord; N] {
        std::array::from_fn(|i| Coord(i))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CoordDerivative(usize);
impl Symbol for CoordDerivative {
    fn eval<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64 {
        state.eval_derivative(state.t, self.0)
    }
    fn eval_at<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>, t: f64) -> f64 {
        state.eval_derivative(t, self.0)
    }
    fn eval_prev<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64 {
        state.eval_derivative(state.t_prev, self.0)
    }

    fn dt(self) -> impl Symbol {
        unimplemented!("differentiating CoordDerivative is not implemented");
        #[allow(unreachable_code)]
        Constant(0.)
    }
}
impl_ops!(CoordDerivative);

#[derive(Copy, Clone, Debug)]
pub struct Add<L: Symbol, R: Symbol>(L, R);
impl<L: Symbol, R: Symbol> Symbol for Add<L, R> {
    fn eval<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64 {
        self.0.eval(state) + self.1.eval(state)
    }

    fn eval_at<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>, t: f64) -> f64 {
        self.0.eval_at(state, t) + self.1.eval_at(state, t)
    }

    fn eval_prev<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64 {
        self.0.eval_prev(state) + self.1.eval_prev(state)
    }

    fn dt(self) -> impl Symbol {
        Add(self.0.dt(), self.1.dt())
    }
}
impl_ops!(Add, L, R);

#[derive(Copy, Clone, Debug)]
pub struct Sub<L: Symbol, R: Symbol>(L, R);
impl<L: Symbol, R: Symbol> Symbol for Sub<L, R> {
    fn eval<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64 {
        self.0.eval(state) - self.1.eval(state)
    }

    fn eval_at<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>, t: f64) -> f64 {
        self.0.eval_at(state, t) - self.1.eval_at(state, t)
    }

    fn eval_prev<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64 {
        self.0.eval_prev(state) - self.1.eval_prev(state)
    }

    fn dt(self) -> impl Symbol {
        Sub(self.0.dt(), self.1.dt())
    }
}
impl_ops!(Sub, L, R);

#[derive(Copy, Clone, Debug)]
pub struct Mul<L: Symbol, R: Symbol>(L, R);
impl<L: Symbol, R: Symbol> Symbol for Mul<L, R> {
    fn eval<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64 {
        self.0.eval(state) * self.1.eval(state)
    }

    fn eval_at<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>, t: f64) -> f64 {
        self.0.eval_at(state, t) * self.1.eval_at(state, t)
    }

    fn eval_prev<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64 {
        self.0.eval_prev(state) * self.1.eval_prev(state)
    }

    fn dt(self) -> impl Symbol {
        Add(Mul(self.0.dt(), self.1), Mul(self.0, self.1.dt()))
    }
}
impl_ops!(Mul, L, R);

#[derive(Copy, Clone, Debug)]
pub struct Div<L: Symbol, R: Symbol>(L, R);
impl<L: Symbol, R: Symbol> Symbol for Div<L, R> {
    fn eval<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64 {
        self.0.eval(state) / self.1.eval(state)
    }

    fn eval_at<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>, t: f64) -> f64 {
        self.0.eval_at(state, t) / self.1.eval_at(state, t)
    }

    fn eval_prev<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64 {
        self.0.eval_prev(state) / self.1.eval_prev(state)
    }

    fn dt(self) -> impl Symbol {
        Div(
            Sub(Mul(self.0.dt(), self.1), Mul(self.0, self.1.dt())),
            self.1.powi(2),
        )
    }
}
impl_ops!(Div, L, R);

#[derive(Copy, Clone, Debug)]
pub struct Neg<Arg: Symbol>(Arg);
impl<Arg: Symbol> Symbol for Neg<Arg> {
    fn eval<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64 {
        -self.0.eval(state)
    }

    fn eval_at<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>, t: f64) -> f64 {
        -self.0.eval_at(state, t)
    }

    fn eval_prev<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64 {
        -self.0.eval_prev(state)
    }

    fn dt(self) -> impl Symbol {
        Neg(self.0.dt())
    }
}
impl_ops!(Neg, Arg);

#[derive(Copy, Clone, Debug)]
pub struct Powi<Arg>(Arg, i32);
impl<Arg: Symbol> Symbol for Powi<Arg> {
    fn eval<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64 {
        self.0.eval(state).powi(self.1)
    }

    fn eval_at<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>, t: f64) -> f64 {
        self.0.eval_at(state, t).powi(self.1)
    }

    fn eval_prev<'a, const N: usize, const S: usize>(&self, state: &State<'a, N, S>) -> f64 {
        self.0.eval_prev(state).powi(self.1)
    }

    fn dt(self) -> impl Symbol {
        self.1 as f64 * Powi(self.0, self.1 - 1) * self.0.dt()
    }
}
impl_ops!(Powi, Arg);

eager! { func_structs!(func_list!()) }

// Functions with no formula for derivative
// gamma
// ln_gamma

// Functions of two variables
// powi
// log
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
//
//
//
// Usage
//
// let (t, [x, y, z]) = state_symbols();
// let eq = [x - y, y + x.exp(), z - y];
//
//
