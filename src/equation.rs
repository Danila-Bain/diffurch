//! Defines [Equation], which holds the right hand side of the equation.

use hlist2::{HList, Nil, ops::Append};

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
        $crate::Equation::new($crate::state_fn!(|| $expr)).max_delay(0.)
    };
    (|$t:ident| $expr:expr) => {
        $crate::Equation::new($crate::state_fn!(|$t| $expr)).max_delay(0.)
    };
    (|[$($x:pat),+]| $expr:expr) => {
        $crate::Equation::new($crate::state_fn!(|[$($x),+]| $expr)).max_delay(0.)
    };
    (|$t:pat, [$($x:pat),+]| $expr:expr) => {
        $crate::Equation::new($crate::state_fn!(|$t, [$($x),+]| $expr)).max_delay(0.)
    };
    (|$t:pat, [$($x:pat),+], [$($x_:pat),+]| $expr:expr) => {
        $crate::Equation::new($crate::state_fn!(|$t, [$($x),+], [$($x_),+]| $expr)).max_delay(f64::MAX)
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
    /// Add detected event without any callback (for discontinuity tracking)
    ///
    /// Those events do get propagated
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
            events: events.append((locate, event_mut!())),
            max_delay,
        }
    }

    /// Add located event with callback
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

    /// Add a variable delay to track discontinuities
    ///
    /// Sets [Self::max_delay] to `f64::INFINITY`
    pub fn delay_with_smoothing_order<D: StateFnMut<N, Output = f64>>(
        self,
        delayed_arg_fn: D,
        smoothing_order: usize,
    ) -> Equation<
        N,
        RHS,
        <Propagations as Append>::Output<Loc<Propagator<N, D>, Propagation, location::Bisection>>,
        Events,
    >
    where
        Propagations: Append,
    {
        let Equation {
            rhs,
            propagations,
            events,
            max_delay: _,
        } = self;

        Equation {
            rhs,
            propagations: propagations.append(crate::loc::Loc(
                crate::loc::Propagator::new(delayed_arg_fn, smoothing_order),
                crate::loc::Propagation,
                crate::loc::location::Bisection,
            )),
            events,
            max_delay: f64::INFINITY,
        }
    }

    /// Add a variable delay to track discontinuities
    ///
    /// Sets [Self::max_delay] to `f64::INFINITY`
    pub fn delay<D: StateFnMut<N, Output = f64>>(
        self,
        delayed_arg_fn: D,
    ) -> Equation<
        N,
        RHS,
        <Propagations as Append>::Output<Loc<Propagator<N, D>, Propagation, location::Bisection>>,
        Events,
    >
    where
        Propagations: Append,
    {
        self.delay_with_smoothing_order(delayed_arg_fn, 1)
    }


    /// Add a variable delay to track discontinuities
    ///
    /// Sets [Self::max_delay] to `f64::INFINITY`
    pub fn neutral_delay<D: StateFnMut<N, Output = f64>>(
        self,
        delayed_arg_fn: D,
    ) -> Equation<
        N,
        RHS,
        <Propagations as Append>::Output<Loc<Propagator<N, D>, Propagation, location::Bisection>>,
        Events,
    >
    where
        Propagations: Append,
    {
        self.delay_with_smoothing_order(delayed_arg_fn, 0)
    }

    /// Add constant delay to the equation
    pub fn const_delay(
        self,
        delay: f64,
    ) -> Equation<
        N,
        RHS,
        <Propagations as Append>::Output<
            Loc<Propagator<N, impl StateFnMut<N, Output = f64>>, Propagation, location::Bisection>,
        >,
        Events,
    >
    where
        Propagations: Append,
    {
        self.const_delay_with_smoothing_order(delay, 1)
    }


    /// Add constant neutral delay to the equation
    pub fn const_neutral_delay(
        self,
        delay: f64,
    ) -> Equation<
        N,
        RHS,
        <Propagations as Append>::Output<
            Loc<Propagator<N, impl StateFnMut<N, Output = f64>>, Propagation, location::Bisection>,
        >,
        Events,
    >
    where
        Propagations: Append,
    {
        self.const_delay_with_smoothing_order(delay, 0)
    }

    /// Add a constant delay to track discontinuities and set max_delay if there are no variable
    /// delays
    pub fn const_delay_with_smoothing_order(
        self,
        delay: f64,
        smoothing_order: usize,
    ) -> Equation<
        N,
        RHS,
        <Propagations as Append>::Output<
            Loc<Propagator<N, impl StateFnMut<N, Output = f64>>, Propagation, location::Bisection>,
        >,
        Events,
    >
    where
        Propagations: Append,
    {
        let Equation {
            rhs,
            propagations,
            events,
            mut max_delay,
        } = self;

        if max_delay.is_nan() {
            max_delay = delay;
        } else if !max_delay.is_infinite() {
            max_delay = max_delay.max(delay)
        }

        Equation {
            rhs,
            propagations: propagations.append(crate::loc::Loc(
                crate::loc::Propagator::new(state_fn!(move |t| t - delay), smoothing_order),
                crate::loc::Propagation,
                crate::loc::location::Bisection,
            )),
            events,
            max_delay,
        }
    }
}
