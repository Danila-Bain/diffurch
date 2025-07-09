//! Defines [State], the core object which is acted upon during integration.

use crate::InitialCondition;

/// Trait that abstracts [RKState], hiding memory layout and rk scheme specifications
pub trait State<const N: usize> {
    /// get current time of the state
    fn t(&self) -> f64;
    /// get previous step time of the state
    fn t_prev(&self) -> f64;
    /// get mutable reference to a current time of the state.
    ///
    /// Can be used to set the time of state to [f64::INFINITY], effectively stopping integration
    fn t_mut(&mut self) -> &mut f64;
    /// get current position of the state
    fn x(&self) -> [f64; N];
    /// get previous step position of the state
    fn x_prev(&self) -> [f64; N];
    /// get previous step derivative of the state
    fn d_prev(&self) -> [f64; N];
    /// get mutable reference to a current position of the state.
    ///
    /// Can be used to implement impacts in the systems due to some events.
    fn x_mut(&mut self) -> &mut [f64; N];
    /// getter, that combines [State::t_mut] and [State::x_mut]
    fn tx_mut(&mut self) -> (&mut f64, &mut [f64; N]);

    /// Make zero step by setting previous values to current ones.
    ///
    /// This is used internally before applying external changes to a state, such that state
    /// history is not lost.
    fn make_zero_step(&mut self);
    /// Make a step of numerical method of the size `t_step`, using `rhs` as the right hand side of
    /// the differential equation.
    fn make_step(&mut self, rhs: &mut impl StateFnMut<N, Output = [f64; N]>, t_step: f64);
    /// Undo last step. (Repeated use does not have an effect)
    ///
    /// This is used to redo last step, because it was rejected due to event location or (not yet
    /// implemented) step size controller.
    fn undo_step(&mut self);
    /// Save last computed step to history
    fn push_current(&mut self);

    /// Evaluate the postion of the state at the time `t` in the past.
    fn eval_all(&self, t: f64) -> [f64; N];
    /// Evaluate the coordinate `coordinate` of the postion of the state at the time `t` in the past.
    fn eval(&self, t: f64, coordinate: usize) -> f64;
    /// Evaluate the derivative of the coordinate `coordinate` of the postion of the state at the time `t` in the past.
    fn eval_derivative(&self, t: f64, coordinate: usize) -> f64;
    /// Return coordinate functions, that can be used to evaluate state at the past times.
    ///
    /// This is the thing passed to the right hand side functions for delay differential equations.
    fn coord_fns<'b>(&'b self) -> [StateCoordFn<'b, N, Self>; N];
}

/// [State] is an object that represents the state of the equation during solving.
///
/// [crate::Equation], [crate::Event], [crate::Loc] are all defined in terms of functions on the
/// state instead of functions on time and coordinates to include functions to work with delay
/// differential equations without overcomplicating api for ordinary differential equations.
///
/// For functions on [State], see [StateFn] and [MutStateFn]
pub struct RKState<'a, const N: usize, const S: usize, IC: InitialCondition<N>>
where
    [(); S * (S - 1) / 2]:,
{
    /// time of the state at the current step
    pub t: f64,
    /// time of the state at the previous step
    pub t_prev: f64,
    /// initial time of the state
    pub t_init: f64,
    /// length of past history stored in state,
    ///
    /// It must be >= than largest delay encountered in delay differential equation.
    ///
    /// It may be 0., may be f64::INFINITE.
    ///
    /// For negative values, solver will panic.
    pub t_span: f64,
    /// time instances of past steps
    ///
    /// The past values that are no longer needed are pop'ed during computation according to [State::t_span].
    pub t_seq: std::collections::VecDeque<f64>,

    /// position of the state at the current step
    pub x: [f64; N],
    /// position of the state at the previous step
    pub x_prev: [f64; N],
    /// initial condition used to initialize or evaluate the state at times before [State::t_init].
    pub x_init: IC,
    /// state values of past steps
    ///
    /// The past values that are no longer needed are pop'ed during computation according to [State::t_span].
    pub x_seq: std::collections::VecDeque<[f64; N]>,
    /// The Runge-Kutta method stages computed for the last step
    pub k: [[f64; N]; S],
    /// The past Runge-Kutta stages used for evaluation of the state at the past times between the
    /// nodal points.
    pub k_seq: std::collections::VecDeque<[[f64; N]; S]>,

    /// Used Runge-Kutta scheme
    pub rk: &'a crate::rk::RungeKuttaTable<S>,
}

impl<'a, const N: usize, const S: usize, IC: InitialCondition<N>> RKState<'a, N, S, IC>
where
    [(); S * (S - 1) / 2]:,
{
    /// State constructor used in [crate::Solver]
    pub fn new(
        t_init: f64,
        x_init: IC,
        t_span: f64,
        rk: &'a crate::rk::RungeKuttaTable<S>,
    ) -> Self {
        let x = x_init.eval::<0>(t_init);

        Self {
            t_init,
            t: t_init,
            t_prev: t_init,
            t_span,
            t_seq: std::collections::VecDeque::from([t_init]),

            x_init,
            x,
            x_prev: x.clone(),
            x_seq: std::collections::VecDeque::from([x.clone()]),

            k: [[0.; N]; S],
            k_seq: std::collections::VecDeque::new(),

            rk,
        }
    }
}

impl<'a, const N: usize, const S: usize, IC: InitialCondition<N>> State<N> for RKState<'a, N, S, IC>
where
    [(); S * (S - 1) / 2]:,
{
    fn t(&self) -> f64 {
        self.t
    }

    fn t_prev(&self) -> f64 {
        self.t_prev
    }

    fn t_mut(&mut self) -> &mut f64 {
        &mut self.t
    }

    fn x(&self) -> [f64; N] {
        self.x
    }

    fn x_prev(&self) -> [f64; N] {
        self.x_prev
    }

    fn d_prev(&self) -> [f64; N] {
        self.k[0]
    }

    fn x_mut(&mut self) -> &mut [f64; N] {
        &mut self.x
    }

    fn tx_mut(&mut self) -> (&mut f64, &mut [f64; N]) {
        (&mut self.t, &mut self.x)
    }

    /// Push current values [State::t], [State::x], [State::k] to history, and pop old history
    /// (older than `self.t_prev - self.t_span - (self.t - self.t_prev)`).
    fn push_current(&mut self) {
        self.t_seq.push_back(self.t);
        self.x_seq.push_back(self.x);
        self.k_seq.push_back(self.k);
        let t_tail = self.t_prev - self.t_span - (self.t - self.t_prev);
        while t_tail
            > *self
                .t_seq
                .front()
                .expect("Last element won't pop for non-negative t_span")
        {
            self.t_seq.pop_front();
            self.x_seq.pop_front();
            self.k_seq.pop_front();
        }
    }

    /// Advance the state by `t_step`, using right-hand-side `rhs` of the equation.
    fn make_step(&mut self, rhs: &mut impl StateFnMut<N, Output = [f64; N]>, t_step: f64) {
        self.t_prev = self.t;
        self.x_prev = self.x;

        let mut a_i = 0;
        for i in 0..S {
            self.t = self.t_prev + self.rk.c[i] * t_step;

            self.x = std::array::from_fn(|k| {
                self.x_prev[k]
                    + t_step * (0..i).fold(0., |acc, j| acc + self.rk.a[a_i + j] * self.k[j][k])
            });
            a_i += i;
            self.k[i] = rhs.eval(self);
        }

        self.x = std::array::from_fn(|k| {
            self.x_prev[k] + t_step * (0..S).fold(0., |acc, j| acc + self.rk.b[j] * self.k[j][k])
        });
        self.t = self.t_prev + t_step;
    }

    /// Advance the state by a zero step, not modifying current time or coordinates.
    ///
    /// This method is used when the state is modified externally by events, to record adjacent
    /// pre- and post-change states with respect to event.
    fn make_zero_step(&mut self) {
        self.t_prev = self.t;
        self.x_prev = self.x;
        self.k = [[0.; N]; S];
    }

    /// Undo the previous step by setting current values to the previous.
    ///
    /// Used to reject last step due to stepsize controller or located step.
    ///
    /// Using this method twice is the same as using it once, because it just resets current time
    /// and coordinates to the previous, without setting previous values to pre-previous values.
    fn undo_step(&mut self) {
        self.t = self.t_prev;
        self.x = self.x_prev;
    }

    /// Evaluate coordinate vector of the state at the time `t` using interpolant provided by
    /// [crate::rk::RungeKuttaTable::bi]. For `t < self.t_init`, the field [State::x_init] is used.
    ///
    /// Since the past history may be cleared according to the [State::t_span], this function may
    /// panic, if the evaluation of deleted section of history is attempted.
    fn eval_all(&self, t: f64) -> [f64; N] {
        if t <= self.t_init {
            self.x_init.eval::<0>(t)
        } else if self.t_prev <= t && t <= self.t {
            let x_prev = self.x_prev;
            let k = self.k;
            let t_prev = self.t_prev;
            let t_next = self.t;
            let t_step = t_next - t_prev;
            if t_step == 0. {
                return x_prev;
            }
            let theta = (t - t_prev) / t_step;
            return std::array::from_fn(|i| {
                x_prev[i] + t_step * (0..S).fold(0., |acc, j| acc + self.rk.bi[j](theta) * k[j][i])
            });
        } else {
            let i = self.t_seq.partition_point(|t_i| t_i < &t); // first i : t_seq[i] >= t
            if i == 0 {
                panic!(
                    "Evaluation of state in deleted time range. Try adding .with_delay({}) to your equation.",
                    self.t - t
                );
            } else if i == self.t_seq.len() {
                panic!(
                    "Evaluation of state in a not yet computed time range at {t} while state.t is {}.",
                    self.t
                );
            }

            let x_prev = &self.x_seq[i - 1];
            let k = &self.k_seq[i - 1];
            let t_prev = self.t_seq[i - 1];
            let t_next = self.t_seq[i];
            let t_step = t_next - t_prev;
            if t_step == 0. {
                return *x_prev;
            }
            let theta = (t - t_prev) / t_step;

            return std::array::from_fn(|i| {
                x_prev[i] + t_step * (0..S).fold(0., |acc, j| acc + self.rk.bi[j](theta) * k[j][i])
            });
        }
    }

    /// Evaluate one coordinate of a coordinate vector of the state at the time `t` using interpolant provided by
    /// [crate::rk::RungeKuttaTable::bi]. For `t < self.t_init`, the field [State::x_init] is used.
    ///
    /// Since the past history may be cleared according to the [State::t_span], this function may
    /// panic, if the evaluation of deleted section of history is attempted.
    fn eval(&self, t: f64, coordinate: usize) -> f64 {
        // Initial history
        if t <= self.t_init {
            self.x_init.eval::<0>(t)[coordinate]
        }
        // Last step (may be accessed frequently for .subdivide option in Events).
        // So using this, we skip search.
        else if self.t_prev <= t && t <= self.t {
            let x_prev = self.x_prev[coordinate];
            let k = self.k;
            let t_prev = self.t_prev;
            let t_next = self.t;
            let t_step = t_next - t_prev;
            if t_step == 0. {
                return x_prev;
            }
            let theta = (t - t_prev) / t_step;
            return x_prev
                + t_step * (0..S).fold(0., |acc, j| acc + self.rk.bi[j](theta) * k[j][coordinate]);
        } else {
            let i = self.t_seq.partition_point(|t_i| t_i < &t); // first i : t_seq[i] >= t
            if i == 0 {
                panic!(
                    "Evaluation of state in deleted time range. Try adding .with_delay({}) to your equation.",
                    self.t - t
                );
            } else if i == self.t_seq.len() {
                panic!(
                    "Evaluation of state in a not yet computed time range at {t} while state.t is {}.",
                    self.t
                );
            }

            let x_prev = &self.x_seq[i - 1][coordinate];
            let k = &self.k_seq[i - 1];
            let t_prev = self.t_seq[i - 1];
            let t_next = self.t_seq[i];
            let t_step = t_next - t_prev;
            if t_step == 0. {
                return *x_prev;
            }
            let theta = (t - t_prev) / t_step;
            return x_prev
                + t_step * (0..S).fold(0., |acc, j| acc + self.rk.bi[j](theta) * k[j][coordinate]);
        }
    }

    /// Evaluate the derivative of one coordinate of a coordinate vector of the state at the time `t` using interpolant provided by
    /// [crate::rk::RungeKuttaTable::bi]. For `t < self.t_init`, the field [State::x_init] is used.
    ///
    /// Since the past history may be cleared according to the [State::t_span], this function may
    /// panic, if the evaluation of deleted section of history is attempted.
    ///
    /// Also, calling [State::x_init] may panic, because [crate::InitialCondition::eval_d] panics
    /// for [crate::InitialCondition::Function] variant, so you need to use
    /// [crate::InitialCondition::Point] or [crate::InitialCondition::FunctionWithDerivative]
    /// variants instead, which are convertable from [f64; N] or tuple of two closures
    /// respectively (see [crate::InitialCondition::into]).
    fn eval_derivative(&self, t: f64, coordinate: usize) -> f64 {
        // Initial history
        if t <= self.t_init {
            self.x_init.eval::<1>(t)[coordinate]
        }
        // Last step (may be accessed frequently for .subdivide option in Events).
        // So using this, we skip search.
        // If last step has zero length, the previous step is used.
        else if self.t_prev <= t && t <= self.t && self.t != self.t_prev {
            let k = self.k;
            let t_prev = self.t_prev;
            let t_next = self.t;
            let t_step = t_next - t_prev;
            let theta = (t - t_prev) / t_step;
            return (0..S).fold(0., |acc, j| acc + self.rk.bi[j].d(theta) * k[j][coordinate]);
        } else {
            let i = self.t_seq.partition_point(|t_i| t_i < &t); // first i : t_seq[i] >= t
            if i == 0 {
                panic!(
                    "Evaluation of state in deleted time range. Try adding .with_delay({}) to your equation.",
                    self.t - t
                );
            } else if i == self.t_seq.len() {
                panic!(
                    "Evaluation of state in a not yet computed time range at {t} while state.t is {}.",
                    self.t
                );
            }

            let k = &self.k_seq[i - 1];
            let t_prev = self.t_seq[i - 1];
            let t_next = self.t_seq[i];
            let t_step = t_next - t_prev;
            let theta = (t - t_prev) / t_step;
            return (0..S).fold(0., |acc, j| acc + self.rk.bi[j].d(theta) * k[j][coordinate]);
        }
    }

    /// Get a vector of [StateCoordFn]s for evaluation of [StateFn::DDE] and [MutStateFn::DDE]
    /// variants.
    fn coord_fns<'b>(&'b self) -> [StateCoordFn<'b, N, Self>; N] {
        std::array::from_fn(|i| StateCoordFn::<'b, N, Self> {
            state: self,
            coord: i,
        })
    }
}

/// Trait, that defines how a function is evaluated at the state.
pub trait StateFnMut<const N: usize> {
    type Output;
    /// evaluate self at the current state
    fn eval(&mut self, state: &impl State<N>) -> Self::Output;
    /// evaluate self at the previous step state
    fn eval_prev(&mut self, state: &impl State<N>) -> Self::Output;
    /// evaluate self at the state at  the time t
    fn eval_at(&mut self, state: &impl State<N>, t: f64) -> Self::Output;
}
/// Trait, that defines how a function is evaluated at the state, which can also mutate the state.
pub trait MutStateFnMut<const N: usize> {
    type Output;

    /// evaluate self at the mutable state
    fn eval_mut(&mut self, state: &mut impl State<N>) -> Self::Output;
}
/// Constant function of the state
#[derive(Clone, Copy)]
pub struct ConstantStateFnMut<F: FnMut<(), Output = Ret>, Ret>(pub F);
impl<F: FnMut<(), Output = Ret>, Ret, const N: usize> StateFnMut<N> for ConstantStateFnMut<F, Ret> {
    type Output = Ret;

    fn eval(&mut self, _: &impl State<N>) -> Ret {
        (self.0)()
    }

    fn eval_prev(&mut self, _: &impl State<N>) -> Ret {
        (self.0)()
    }

    fn eval_at(&mut self, _: &impl State<N>, _: f64) -> Ret {
        (self.0)()
    }
}

/// Constant function of the mut state
impl<F: FnMut<(), Output = Ret>, Ret, const N: usize> MutStateFnMut<N>
    for ConstantStateFnMut<F, Ret>
{
    type Output = Ret;

    fn eval_mut(&mut self, _: &mut impl State<N>) -> Ret {
        (self.0)()
    }
}

/// Time-dependent function of the state
#[derive(Clone, Copy)]
pub struct TimeStateFnMut<F: FnMut<(f64,), Output = Ret>, Ret>(pub F);
impl<F: FnMut<(f64,), Output = Ret>, Ret, const N: usize> StateFnMut<N> for TimeStateFnMut<F, Ret> {
    type Output = Ret;

    fn eval(&mut self, state: &impl State<N>) -> Ret {
        (self.0)(state.t())
    }

    fn eval_prev(&mut self, state: &impl State<N>) -> Ret {
        (self.0)(state.t_prev())
    }

    fn eval_at(&mut self, _: &impl State<N>, t: f64) -> Ret {
        (self.0)(t)
    }
}

impl<F: FnMut<(f64,), Output = Ret>, Ret, const N: usize> MutStateFnMut<N>
    for TimeStateFnMut<F, Ret>
{
    type Output = Ret;
    fn eval_mut(&mut self, state: &mut impl State<N>) -> Ret {
        (self.0)(state.t())
    }
}
/// Time-mutating function of the state
#[derive(Clone, Copy)]
pub struct TimeMutStateFnMut<F: for<'a> FnMut<(&'a mut f64,), Output = Ret>, Ret>(pub F);
impl<F: for<'a> FnMut<(&'a mut f64,), Output = Ret>, Ret, const N: usize> MutStateFnMut<N>
    for TimeMutStateFnMut<F, Ret>
{
    type Output = Ret;
    fn eval_mut(&mut self, state: &mut impl State<N>) -> Ret {
        (self.0)(state.t_mut())
    }
}

/// Position-dependent function of the state
#[derive(Clone, Copy)]
pub struct ODEStateFnMut<const N: usize, F: FnMut<([f64; N],), Output = Ret>, Ret>(pub F);
impl<F: FnMut<([f64; N],), Output = Ret>, Ret, const N: usize> StateFnMut<N>
    for ODEStateFnMut<N, F, Ret>
{
    type Output = Ret;
    fn eval(&mut self, state: &impl State<N>) -> Ret {
        (self.0)(state.x())
    }

    fn eval_prev(&mut self, state: &impl State<N>) -> Ret {
        (self.0)(state.x_prev())
    }

    fn eval_at(&mut self, state: &impl State<N>, t: f64) -> Ret {
        (self.0)(state.eval_all(t))
    }
}
impl<F: for<'a> FnMut<([f64; N],), Output = Ret>, Ret, const N: usize> MutStateFnMut<N>
    for ODEStateFnMut<N, F, Ret>
{
    type Output = Ret;
    fn eval_mut(&mut self, state: &mut impl State<N>) -> Ret {
        (self.0)(state.x())
    }
}

/// Position-mutating function of the state
#[derive(Clone, Copy)]
pub struct ODEMutStateFnMut<
    const N: usize,
    F: for<'a> FnMut<(&'a mut [f64; N],), Output = Ret>,
    Ret,
>(pub F);
impl<F: for<'a> FnMut<(&'a mut [f64; N],), Output = Ret>, Ret, const N: usize> MutStateFnMut<N>
    for ODEMutStateFnMut<N, F, Ret>
{
    type Output = Ret;
    fn eval_mut(&mut self, state: &mut impl State<N>) -> Ret {
        (self.0)(state.x_mut())
    }
}

/// Time- and position-depending function of the state
#[derive(Clone, Copy)]
pub struct ODE2StateFnMut<const N: usize, F: FnMut<(f64, [f64; N]), Output = Ret>, Ret>(pub F);
impl<F: FnMut<(f64, [f64; N]), Output = Ret>, Ret, const N: usize> StateFnMut<N>
    for ODE2StateFnMut<N, F, Ret>
{
    type Output = Ret;
    fn eval(&mut self, state: &impl State<N>) -> Ret {
        (self.0)(state.t(), state.x())
    }

    fn eval_prev(&mut self, state: &impl State<N>) -> Ret {
        (self.0)(state.t_prev(), state.x_prev())
    }

    fn eval_at(&mut self, state: &impl State<N>, t: f64) -> Ret {
        (self.0)(t, state.eval_all(t))
    }
}

impl<F: for<'a> FnMut<(f64, [f64; N]), Output = Ret>, Ret, const N: usize> MutStateFnMut<N>
    for ODE2StateFnMut<N, F, Ret>
{
    type Output = Ret;
    fn eval_mut(&mut self, state: &mut impl State<N>) -> Ret {
        (self.0)(state.t(), state.x())
    }
}

/// Time- and position-mutating function of the state
#[derive(Clone, Copy)]
pub struct ODE2MutStateFnMut<
    const N: usize,
    F: for<'a> FnMut<(&'a mut f64, &'a mut [f64; N]), Output = Ret>,
    Ret,
>(pub F);
impl<F: for<'a> FnMut<(&'a mut f64, &'a mut [f64; N]), Output = Ret>, Ret, const N: usize>
    MutStateFnMut<N> for ODE2MutStateFnMut<N, F, Ret>
{
    type Output = Ret;
    fn eval_mut(&mut self, state: &mut impl State<N>) -> Ret {
        let (t, x) = state.tx_mut();
        (self.0)(t, x)
    }
}
// // struct dde_closure;
// // impl dde_closure {
// //     fn eval<const S: usize>(t: f64, [x]: [f64; 1], [x_]: [StateCoordFn<'_, 1, S>; 1]) -> [f64; 1] {
// //         [-x + 2. * x_(t - 1.)]
// //     }
// // }
//
// struct DDEStateFnMut<F>(F);
// impl<F: FnMut<(f64, [f64; N]), Output = Ret>, Ret, const N: usize> StateFnMut<N, Ret>
//     for DDEStateFnMut<F>
// {
//     fn eval<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Ret where [(); S * (S - 1) / 2]:  {
//         (self.0)(state.t, state.x)
//     }
//     fn eval_prev<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Ret where [(); S * (S - 1) / 2]:  {
//         (self.0)(state.t_prev, state.x_prev)
//     }
//     fn eval_at<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>, t: f64) -> Ret {
//         (self.0)(t, state.eval_all(t))
//     }
// }

/// Time-, position-, and past state-dependent function of the state
#[derive(Clone, Copy)]
pub struct DDEStateFnMut<
    const N: usize,
    F: for<'a> FnMut<(f64, [f64; N], [&'a dyn StateCoordFnTrait; N]), Output = Ret>,
    Ret,
>(pub F);
impl<
    F: for<'a> FnMut<(f64, [f64; N], [&'a dyn StateCoordFnTrait; N]), Output = Ret>,
    Ret,
    const N: usize,
> StateFnMut<N> for DDEStateFnMut<N, F, Ret>
{
    type Output = Ret;
    fn eval(&mut self, state: &impl State<N>) -> Ret {
        let coord_fns: [StateCoordFn<'_, N, _>; N] = state.coord_fns();
        let coord_fns = std::array::from_fn(|i| &coord_fns[i] as &dyn StateCoordFnTrait);
        (self.0)(state.t(), state.x(), coord_fns)
    }

    fn eval_prev(&mut self, state: &impl State<N>) -> Ret {
        let coord_fns: [StateCoordFn<'_, N, _>; N] = state.coord_fns();
        let coord_fns = std::array::from_fn(|i| &coord_fns[i] as &dyn StateCoordFnTrait);
        (self.0)(state.t_prev(), state.x_prev(), coord_fns)
    }

    fn eval_at(&mut self, state: &impl State<N>, t: f64) -> Ret {
        let coord_fns: [StateCoordFn<'_, N, _>; N] = state.coord_fns();
        let coord_fns = std::array::from_fn(|i| &coord_fns[i] as &dyn StateCoordFnTrait);
        (self.0)(t, state.eval_all(t), coord_fns)
    }
}
impl<
    F: for<'a> FnMut<(f64, [f64; N], [&'a dyn StateCoordFnTrait; N]), Output = Ret>,
    Ret,
    const N: usize,
> MutStateFnMut<N> for DDEStateFnMut<N, F, Ret>
{
    type Output = Ret;
    fn eval_mut(&mut self, state: &mut impl State<N>) -> Ret {
        let coord_fns: [StateCoordFn<'_, N, _>; N] = state.coord_fns();
        let coord_fns = std::array::from_fn(|i| &coord_fns[i] as &dyn StateCoordFnTrait);
        (self.0)(state.t(), state.x(), coord_fns)
    }
}

// Borrowing rules violation
pub struct DDEMutStateFnMut<
    const N: usize,
    F: for<'a> FnMut<
            (
                &'a mut f64,
                &'a mut [f64; N],
                [&'a dyn StateCoordFnTrait; N],
            ),
            Output = Ret,
        >,
    Ret,
>(pub F);
impl<
    F: for<'a> FnMut<
            (
                &'a mut f64,
                &'a mut [f64; N],
                [&'a dyn StateCoordFnTrait; N],
            ),
            Output = Ret,
        >,
    Ret,
    const N: usize,
> MutStateFnMut<N> for DDEMutStateFnMut<N, F, Ret>
{
    type Output = Ret;
    fn eval_mut(&mut self, state: &mut impl State<N>) -> Ret {
        let coord_fns: [StateCoordFn<'_, N, _>; N] = state.coord_fns();
        let coord_fns = std::array::from_fn(|i| &coord_fns[i] as &dyn StateCoordFnTrait);

        let mut t = state.t();
        let mut x = state.x();
        let ret = (self.0)(&mut t, &mut x, coord_fns);

        *state.t_mut() = t;
        *state.x_mut() = x;

        ret
    }
}

pub struct StateFnMutComposition<F, SF>(pub F, pub SF);
impl<Ret1, Ret2, SF: StateFnMut<N, Output = Ret1>, F: FnMut(Ret1) -> Ret2, const N: usize> StateFnMut<N> for StateFnMutComposition<F, SF> {
    type Output = Ret2;

    fn eval(&mut self, state: &impl State<N>) -> Self::Output {
        self.0(self.1.eval(state))
    }

    fn eval_prev(&mut self, state: &impl State<N>) -> Self::Output {
        self.0(self.1.eval_prev(state))
    }

    fn eval_at(&mut self, state: &impl State<N>, t: f64) -> Self::Output {
        self.0(self.1.eval_at(state, t))
    }
}

/// Struct that holds a reference to the state, and the coordinate index.
///
/// It implements Fn() -> f64 and Fn(f64) -> f64 traits, as evaluation of current and past state
/// respectively.
pub struct StateCoordFn<'a, const N: usize, S: State<N> + ?Sized> {
    /// Reference to the state
    pub state: &'a S,
    /// Coordinate index
    pub coord: usize,
}

/// Trait to erase generic parameter S from StateCoordFn
pub trait StateCoordFnTrait: Fn(f64) -> f64 {
    /// evaluate the derivative
    fn d(&self, t: f64) -> f64;
    fn prev(&self) -> f64;
    fn d_prev(&self) -> f64;
}

impl<'a, const N: usize, S: State<N>> FnOnce<(f64,)> for StateCoordFn<'a, N, S> {
    type Output = f64;
    #[inline]
    extern "rust-call" fn call_once(self, arg: (f64,)) -> Self::Output {
        self.state.eval(arg.0, self.coord)
    }
}

impl<'a, const N: usize, S: State<N>> FnMut<(f64,)> for StateCoordFn<'a, N, S> {
    #[inline]
    extern "rust-call" fn call_mut(&mut self, arg: (f64,)) -> Self::Output {
        self.state.eval(arg.0, self.coord)
    }
}

impl<'a, const N: usize, S: State<N>> Fn<(f64,)> for StateCoordFn<'a, N, S> {
    extern "rust-call" fn call(&self, arg: (f64,)) -> Self::Output {
        self.state.eval(arg.0, self.coord)
    }
}

impl<'a, const N: usize, S: State<N>> StateCoordFnTrait for StateCoordFn<'a, N, S> {
    fn d(&self, t: f64) -> f64 {
        self.state.eval_derivative(t, self.coord)
    }

    fn prev(&self) -> f64 {
        self.state.x_prev()[self.coord]
    }

    fn d_prev(&self) -> f64 {
        self.state.d_prev()[self.coord]
    }
}
//
// /// Creates a [crate::Event] from a closure.
// ///
// /// `event!` allows `Event` to be defined with closures of different calling signatures,
// /// being a replacement of some constructors of [crate::Event]:
// ///
// /// ```rust
// /// #![feature(generic_const_exprs)]
// /// #![allow(incomplete_features)]
// ///
// /// use diffurch::event;
// ///
// /// // use in solver for generic parameters inference
// /// let solver = diffurch::Solver::new()
// ///     .on_step(event!(|| 1.)) // equivalent to .on_step(Event::constant(...))
// ///     .on_step(event!(|t| t + t.cos())) // equivalent to .on_step(Event::time(...))
// ///     .on_step(event!(|[x, y]| [x, y, x+y])) // equivalent to .on_step(Event::ode(...))
// ///     .on_step(event!(|t, [x, y]| [t, x, y])) // equivalent to .on_step(Event::ode2(...))
// ///     .on_step(event!(|t, [x, y], [x_, y_]| [t, x, x_(t - 1.)])) // equivalent to .on_step(Event::dde(...))
// ///     .on_step(event!(|t, [x, y], [x_, y_]| [t, x, x_(t - 1.), x_.d(t - 1.)])); // equivalent to .on_step(Event::dde(...))
// /// ```
// ///
// /// For state mutating events, use [event_mut!].
#[macro_export]
macro_rules! state_fn {
    () => {
        $crate::state::ConstantStateFnMut(|| {})
    };
    ($($move:ident)? || $expr:expr) => {
        $crate::state::ConstantStateFnMut($($move)? || {$expr})
    };
    ($($move:ident)? |$t:ident| $expr:expr) => {
        $crate::state::TimeStateFnMut($($move)? |$t| $expr)
    };
    ($($move:ident)? |[$($x:pat),+]| $expr:expr) => {
        $crate::state::ODEStateFnMut($($move)? |[$($x),+]| $expr)
    };
    ($($move:ident)? |$t:pat, [$($x:pat),+]| $expr:expr) => {
        $crate::state::ODE2StateFnMut($($move)? |$t, [$($x),+]| $expr)
    };
    ($($move:ident)? |$t:pat, [$($x:pat),+], [$($x_:pat),+]| $expr:expr) => {
        $crate::state::DDEStateFnMut($($move)? |$t, [$($x),+], [$($x_),+]| $expr)
    };
}
//
// /// State-mutating counter-part of [event!].
// ///
// /// `event_mut!` allows `Event` to be defined with closures of different calling signatures,
// /// being a replacement of some constructors of [crate::Event]:
// ///
// /// ```rust
// /// #![feature(generic_const_exprs)]
// /// #![allow(incomplete_features)]
// ///
// /// use diffurch::event_mut;
// ///
// /// // use in solver for generic parameters inference
// /// let solver = diffurch::Solver::new()
// ///     .on_step(event_mut!(|t| *t = f64::INFINITY))
// ///     .on_step(event_mut!(|[x, y]| {*x = -*x; [*x, *y, *x + *y]}))
// ///     .on_step(event_mut!(|t, [x, y]| {*x = -*y; *t = f64::INFINITY;}));
// /// ```
// ///
#[macro_export]
macro_rules! mut_state_fn {
    ($($move:ident)? |$t:ident| $expr:expr) => {
        $crate::state::TimeMutStateFnMut($($move)? |$t| $expr)
    };
    ($($move:ident)? |[$($x:pat),+]| $expr:expr) => {
        $crate::state::ODEMutStateFnMut($($move)? |[$($x),+]| $expr)
    };
    ($($move:ident)? |$t:pat, [$($x:pat),+]| $expr:expr) => {
        $crate::state::ODE2MutStateFnMut($($move)? |$t, [$($x),+]| $expr)
    };
    (|$t:ident, [$($x:ident),+], [$($x_:ident),+]| $expr:expr) => {
        $crate::state::DDEMutStateFnMut(|$t, [$($x),+], [$($x_),+]| $expr)
    };
}
