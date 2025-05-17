//! Defines [Equation], which holds the right hand side of the equation.

use crate::{StateCoordFnTrait, StateFn};

/// The struct that is used to hold the right hand side of the function.
///
/// [Equation] provides convenience constructor which accept closure
/// representations of the equation, with the types of arguments deduced.
///
/// # Examples
///
/// Constructing equations from closures with different signatures:
/// ```rust
/// use diffurch::Equation;
/// // right hand side (RHS) is a constant:
/// let constant = Equation::constant(|| [1.]); // x'(t) = 1
/// // RHS is a known time function, independent of coordinates:
/// let time = Equation::time(|t| [t.sin(), t.cos()]); // x'(t) = sin(t), y'(t) = cos(t) 
/// // RHS of an autonomous ordinary differential equation (ODE):
/// let ode = Equation::ode(|[x, y]| [-y, x]); // x'(t) = -y(t), y'(t) = x(t)
/// // RHS of a non-autonomous ODE:
/// let ode2 = Equation::ode2(|t, [x, y]| [-y / t, x * t]); // x'(t) 
/// // RHS of a delay differential equation (DDE):
/// let dde = Equation::dde(|t, [x], [x_]| [4. * x * (1. - x_(t - 1.))]); // x'(t) = 4 x(t) (1 - x(t-1))
/// // RHS of a neutral delay differential equation (NDDE):
/// let ndde = Equation::dde(|t, [x], [x_]| [4. * x * (1. - x_.d(t - 1.))]); // x'(t) = 4 x(t) (1 - x'(t-1))
/// ```
///
/// Equivalent code using [crate::equation!] macro:
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
pub struct Equation<'a, const N: usize = 1> {
    /// The right-hand-side of the function, a function that acts on
    /// [crate::State].
    pub rhs: StateFn<'a, N, [f64; N]>,
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
        $crate::Equation::constant(|| $expr)
    };
    (|$t:ident| $expr:expr) => {
        $crate::Equation::time(|$t| $expr)
    };
    (|[$($x:ident),+]| $expr:expr) => {
        $crate::Equation::ode(|[$($x),+]| $expr)
    };
    (|$t:ident, [$($x:ident),+]| $expr:expr) => {
        $crate::Equation::ode2(|$t, [$($x),+]| $expr)
    };
    (|$t:ident, [$($x:ident),+], [$($x_:ident),+]| $expr:expr) => {
        $crate::Equation::dde(|$t, [$($x),+], [$($x_),+]| $expr)
    };
}

impl<'a, const N: usize> Equation<'a, N> {

    /// Constructor that accepts [StateFn] and sets [Equation::max_delay] to `f64::NAN`.
    pub fn new(rhs: StateFn<'a, N, [f64; N]>) -> Self {
        Equation {
            rhs,
            max_delay: f64::NAN,
        }
    }

    /// Constructor that accepts a `Fn() -> [f64; N]` closure and sets [Equation::max_delay] to `0`.
    pub fn constant<RHS>(rhs: RHS) -> Self
    where
        RHS: 'a + Fn<(), Output = [f64; N]>,
    {
        Equation {
            rhs: StateFn::Constant(Box::new(rhs)),
            max_delay: 0.,
        }
    }

    /// Constructor that accepts a `Fn(f64) -> [f64; N]` closure and sets [Equation::max_delay] to `0`. 
    pub fn time<RHS>(rhs: RHS) -> Self
    where
        RHS: 'a + Fn<(f64,), Output = [f64; N]>,
    {
        Equation {
            rhs: StateFn::Time(Box::new(rhs)),
            max_delay: 0.,
        }
    }

    /// Constructor that accepts a `Fn([f64; N]) -> [f64; N]` closure and sets [Equation::max_delay] to `0`.
    pub fn ode<RHS>(rhs: RHS) -> Self
    where
        RHS: 'a + Fn<([f64; N],), Output = [f64; N]>,
    {
        Equation {
            rhs: StateFn::ODE(Box::new(rhs)),
            max_delay: 0.,
        }
    }

    /// Constructor that accepts a `Fn(f64, [f64; N]) -> [f64; N]` closure and sets [Equation::max_delay] to `0`.
    pub fn ode2<RHS>(rhs: RHS) -> Self
    where
        RHS: 'a + Fn<(f64, [f64; N]), Output = [f64; N]>,
    {
        Equation {
            rhs: StateFn::ODE2(Box::new(rhs)),
            max_delay: 0.,
        }
    }

    /// Constructor that accepts a `Fn(f64, [f64; N], ) -> [f64; N]` closure and sets [Equation::max_delay] to `f64::INFINITY`.
    pub fn dde<RHS>(rhs: RHS) -> Self
    where
        RHS: 'a + Fn(f64, [f64; N], [Box<dyn '_ + StateCoordFnTrait>; N]) -> [f64; N],
    {
        Equation {
            rhs: StateFn::DDE(Box::new(rhs)),
            max_delay: f64::INFINITY,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation() {
        let _eq = Equation {
            rhs: StateFn::ODE2(Box::new(|t, [x, y]| [-y / t, x])),
            max_delay: f64::NAN,
        };

        let _eq = Equation::new(StateFn::Constant(Box::new(|| [42.])));
        let _eq = Equation::ode(|[x, y]| [-y, x]);
        let _eq = Equation::ode2(|t, [x, y, z]| [t - y, z - x, x - z / t]);
    }
}
