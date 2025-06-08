//! Defines [Equation], which holds the right hand side of the equation.

use crate::{StateCoordFnTrait, StateFnMut};

// /// Constructing equations from closures with different signatures:
// /// ```rust
// /// use diffurch::Equation;
// /// // right hand side (RHS) is a constant:
// /// let constant = Equation::constant(|| [1.]); // x'(t) = 1
// /// // RHS is a known time function, independent of coordinates:
// /// let time = Equation::time(|t| [t.sin(), t.cos()]); // x'(t) = sin(t), y'(t) = cos(t) 
// /// // RHS of an autonomous ordinary differential equation (ODE):
// /// let ode = Equation::ode(|[x, y]| [-y, x]); // x'(t) = -y(t), y'(t) = x(t)
// /// // RHS of a non-autonomous ODE:
// /// let ode2 = Equation::ode2(|t, [x, y]| [-y / t, x * t]); // x'(t) 
// /// // RHS of a delay differential equation (DDE):
// /// let dde = Equation::dde(|t, [x], [x_]| [4. * x * (1. - x_(t - 1.))]); // x'(t) = 4 x(t) (1 - x(t-1))
// /// // RHS of a neutral delay differential equation (NDDE):
// /// let ndde = Equation::dde(|t, [x], [x_]| [4. * x * (1. - x_.d(t - 1.))]); // x'(t) = 4 x(t) (1 - x'(t-1))
// /// ```
// ///
// /// Equivalent code using [crate::equation!] macro:

/// The struct that is used to hold the right hand side of the function.
///
/// [Equation] provides convenience constructor which accept closure
/// representations of the equation, with the types of arguments deduced.
///
/// # Examples
/// ```rust
/// use diffurch::equation;
/// let constant = equation!(|| [1.]);
/// let time = equation!(|t| [t.sin(), t.cos()]);
/// let ode = equation!(|[x, y]| [-y, x]);
/// let ode2 = equation!(|t, [x, y]| [-y / t, x * t]);
/// let dde = equation!(|t, [x], [x_]| [4. * x * (1. - x_(t - 1.))]);
/// let ndde = equation!(|t, [x], [x_]| [4. * x * (1. - x_.d(t - 1.))]);
/// ```
///
pub struct Equation<const N: usize, RHS: StateFnMut<N, [f64; N]>> {
    /// The right-hand-side of the function, a function that acts on
    /// [crate::State].
    pub rhs: RHS,
    /// The maximal delay, that is present in the equation.
    ///
    /// By default, it is zero for ordinary differential equations, and  `f64::INFINITY` for delay
    /// differential equations.
    /// 
    /// In delay differential equations, the past state of the solution must be stored. But
    /// usually, only recent history is needed, and this field tells the solver, that the history
    /// this old won't be needed. Solver panics, if needed solution history turns out to be deleted. 
    ///
    /// If you need to integrate delay differential equations for long, you might want to set this field using
    /// [Equation::with_delay] method, to avoid excessive memory usage.
    ///
    /// Also, if you use events that compute something non-local, i.e. the amplitude of the
    /// periodic solution, or variance on the part of the solution, you might need to access past
    /// states, in which case you also might need to set this field to a larger value with
    /// [Equation::with_delay]
    ///
    pub max_delay: f64,
}

/// Creates a [crate::Equation] from a closure.
///
/// `equation!` allows `Equation` to be defined with closures of different calling signatures,
/// being like an overloading version of constructors of [crate::Equation]:
/// ```rust
/// use diffurch::equation;
/// let constant = equation!(|| [1.]);
/// let time = equation!(|t| [t.sin(), t.cos()]);
/// let ode = equation!(|[x, y]| [-y, x]);
/// let ode2 = equation!(|t, [x, y]| [-y / t, x * t]);
/// let dde = equation!(|t, [x], [x_]| [4. * x * (1. - x_(t - 1.))]);
/// let ndde = equation!(|t, [x], [x_]| [4. * x * (1. - x_.d(t - 1.))]);
/// ```
#[macro_export]
macro_rules! equation {
    (|| $expr:expr) => {
        $crate::Equation::new_with_delay($crate::state::ConstantStateFnMut(|| $expr), 0.)
    };
    (|$t:ident| $expr:expr) => {
        $crate::Equation::new_with_delay($crate::state::TimeStateFnMut(|$t| $expr), 0.)
    };
    (|[$($x:ident),+]| $expr:expr) => {
        $crate::Equation::new_with_delay($crate::state::ODEStateFnMut(|[$($x),+]| $expr), 0.)
    };
    (|$t:ident, [$($x:ident),+]| $expr:expr) => {
        $crate::Equation::new_with_delay($crate::state::ODE2StateFnMut(|$t, [$($x),+]| $expr), 0.)
    };
    (|$t:ident, [$($x:ident),+], [$($x_:ident),+]| $expr:expr) => {
        $crate::Equation::new_with_delay($crate::state::DDEStateFnMut(|$t, [$($x),+], [$($x_),+]| $expr), f64::MAX)
    };
}

impl<const N: usize, RHS: StateFnMut<N, [f64; N]>> Equation<N, RHS> {

    /// Constructor that accepts [StateFn] and sets [Equation::max_delay] to `f64::NAN`.
    pub fn new(rhs: RHS) -> Self {
        Equation {
            rhs,
            max_delay: f64::NAN,
        }
    }

    pub fn new_with_delay(rhs: RHS, max_delay: f64) -> Self {
        Equation {
            rhs,
            max_delay
        }
    }

    /// Sets [Equation::max_delay] and returns Self
    pub fn with_delay(self, value: f64) -> Self {
        Self {
            rhs: self.rhs,
            max_delay: value,
        }
    }
}
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn creation() {
//         let _eq = Equation {
//             rhs: StateFn::ODE2(Box::new(|t, [x, y]| [-y / t, x])),
//             max_delay: f64::NAN,
//         };
//
//         let _eq = Equation::new(StateFn::Constant(Box::new(|| [42.])));
//         let _eq = Equation::ode(|[x, y]| [-y, x]);
//         let _eq = Equation::ode2(|t, [x, y, z]| [t - y, z - x, x - z / t]);
//     }
// }
