//! Defines [Equation], which holds the right hand side of the equation.

use hlist2::{HList, Nil, convert::IntoHList, ops::Append};

use crate::*;

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

/// The struct that is used to hold the right hand side of the function, and max delay that the
/// equation can use.
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
pub struct Equation<
    const N: usize,
    RHS: StateFnMut<N, Output = [f64; N]>,
    Propagations: HList = Nil,
    Events: HList = Nil,
> {
    /// The right-hand-side of the function, a function that acts on
    /// [crate::State].
    pub rhs: RHS,
    /// Delays present in equation. Mentioned delays are used to manage propagating
    /// discontinuities to preserve the order of underlying continous integration method.
    pub propagations: Propagations,
    /// Locators for discontinuities in equation
    pub events: Events,
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
    /// [Equation::max_delay] method, to avoid excessive memory usage.
    ///
    /// Also, if you use events that compute something non-local, i.e. the amplitude of the
    /// periodic solution, or variance on the part of the solution, you might need to access past
    /// states, in which case you also might need to set this field to a larger value with
    /// [Equation::max_delay]
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
        $crate::Equation::new($crate::state::ConstantStateFnMut(|| $expr)).max_delay(0.)
    };
    (|$t:ident| $expr:expr) => {
        $crate::Equation::new($crate::state::TimeStateFnMut(|$t| $expr)).max_delay(0.)
    };
    (|[$($x:pat),+]| $expr:expr) => {
        $crate::Equation::new($crate::state::ODEStateFnMut(|[$($x),+]| $expr)).max_delay(0.)
    };
    (|$t:pat, [$($x:pat),+]| $expr:expr) => {
        $crate::Equation::new($crate::state::ODE2StateFnMut(|$t, [$($x),+]| $expr)).max_delay(0.)
    };
    (|$t:pat, [$($x:pat),+], [$($x_:pat),+]| $expr:expr) => {
        $crate::Equation::new($crate::state::DDEStateFnMut(|$t, [$($x),+], [$($x_),+]| $expr)).max_delay(f64::MAX)
    };
}

/// Bare bones example:
/// ```rust
/// use diffurch::{Equation, ODE2StateFnMut};
/// let eq = Equation {
///     rhs: ODE2StateFnMut(|t, [x, y]| [-y / t, x]),
///     max_delay: f64::NAN,
///     propagations: hlist2::Nil,
///     events: hlist2::Nil,
/// };
/// ```
impl<const N: usize, RHS: StateFnMut<N, Output = [f64; N]>> Equation<N, RHS> {
    /// Constructor with arbitrary [StateFnMut] function
    ///
    /// # Examples:
    ///
    /// ```rust
    /// use diffurch::{Equation, state::*};
    /// let eq = Equation::new(ConstantStateFnMut(|| [42.]));
    /// let eq = Equation::new(ODEStateFnMut(|[x, y]| [-y, x]));
    /// let eq = Equation::new(ODE2StateFnMut(|t, [x, y]| [-y / t, x]));
    /// ```
    pub fn new(rhs: RHS) -> Self {
        Equation {
            rhs,
            max_delay: f64::NAN,
            propagations: Nil,
            events: Nil,
        }
    }

    /// Sets [Equation::max_delay] and returns Self
    pub fn max_delay(self, value: f64) -> Self {
        Self {
            rhs: self.rhs,
            max_delay: value,
            propagations: Nil,
            events: Nil,
        }
    }
}

impl<const N: usize, RHS: StateFnMut<N, Output = [f64; N]>, Propagations: HList, Events: HList>
    Equation<N, RHS, Propagations, Events>
{
    pub fn loc<L: Locate<N>>(
        self,
        locate: L,
    ) -> Equation<N, RHS, Propagations, <Events as Append>::Output<(L, impl EventCall<N>)>>
    where
        Events: Append,
    {
        let Equation {
            rhs,
            propagations,
            events,
            max_delay,
        } = self;

        Equation {
            rhs,
            propagations,
            events: events.append((locate, event!())),
            max_delay,
        }
    }

    pub fn on_loc<L: Locate<N>, E: EventCall<N>>(
        self,
        locate: L,
        event: E,
    ) -> Equation<N, RHS, Propagations, <Events as Append>::Output<(L, E)>>
    where
        Events: Append,
    {
        let Equation {
            rhs,
            propagations,
            events,
            max_delay,
        } = self;

        Equation {
            rhs,
            propagations,
            events: events.append((locate, event)),
            max_delay,
        }
    }

    pub fn delay<D: StateFnMut<N, Output = f64>>(
        self,
        delayed_arg_fn: D,
    ) -> Equation<N, RHS, <Propagations as Append>::Output<Propagated<D>>, Events>
    where
        Propagations: Append,
    {
        let Equation {
            rhs,
            propagations,
            events,
            max_delay,
        } = self;

        Equation {
            rhs,
            propagations: propagations.append(Propagated::new(delayed_arg_fn)),
            events,
            max_delay,
        }
    }

    pub fn const_delay(
        self,
        delay: f64,
    ) -> Equation<N, RHS, <Propagations as Append>::Output<Propagated<impl StateFnMut<N>>>, Events>
    where
        Propagations: Append,
    {
        let mut new_self = self.delay(state_fn!(move |t| t - delay));
        if new_self.max_delay.is_nan() {
            new_self.max_delay = delay;
        } else if !new_self.max_delay.is_infinite() {
            new_self.max_delay = new_self.max_delay.max(delay)
        }
        new_self
    }
}
