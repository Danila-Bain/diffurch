//! Defines [State], the core object which is acted upon during integration.

/// [State] is an object that represents the state of the equation during solving.
///
/// [crate::Equation], [crate::Event], [crate::Loc] are all defined in terms of functions on the
/// state instead of functions on time and coordinates to include functions to work with delay
/// differential equations without overcomplicating api for ordinary differential equations.
///
/// For functions on [State], see [StateFn] and [MutStateFn]
pub struct State<'a, const N: usize, const S: usize>
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
    pub x_init: crate::InitialCondition<'a, N>,
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

impl<'a, const N: usize, const S: usize> State<'a, N, S>
where
    [(); S * (S - 1) / 2]:,
{
    /// State constructor used in [crate::Solver]
    pub fn new(
        t_init: f64,
        x_init: impl Into<crate::InitialCondition<'a, N>>,
        t_span: f64,
        rk: &'a crate::rk::RungeKuttaTable<S>,
    ) -> Self {
        let x_init: crate::InitialCondition<'_, N> = x_init.into();
        let x = match &x_init {
            &crate::InitialCondition::Point(value) => value,
            crate::InitialCondition::Function(f)
            | crate::InitialCondition::FunctionWithDerivative(f, _) => f(t_init),
        };

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

    /// Push current values [State::t], [State::x], [State::k] to history, and pop old history
    /// (older than `self.t_prev - self.t_span - (self.t - self.t_prev)`).
    pub fn push_current(&mut self) {
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
    pub fn make_step(&mut self, rhs: &mut impl StateFnMut<N, [f64; N]>, t_step: f64) {
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
    pub fn make_zero_step(&mut self) {
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
    pub fn undo_step(&mut self) {
        self.t = self.t_prev;
        self.x = self.x_prev;
    }

    /// Evaluate coordinate vector of the state at the time `t` using interpolant provided by
    /// [crate::rk::RungeKuttaTable::bi]. For `t < self.t_init`, the field [State::x_init] is used.
    ///
    /// Since the past history may be cleared according to the [State::t_span], this function may
    /// panic, if the evaluation of deleted section of history is attempted.
    pub fn eval_all(&self, t: f64) -> [f64; N] {
        if t <= self.t_init {
            self.x_init.eval(t)
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
    pub fn eval(&self, t: f64, coordinate: usize) -> f64 {
        if t <= self.t_init {
            self.x_init.eval(t)[coordinate]
        } else if self.t_prev <= t && t <= self.t {
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
    pub fn eval_derivative(&self, t: f64, coordinate: usize) -> f64 {
        if t <= self.t_init {
            self.x_init.eval_d(t)[coordinate]
        } else if self.t_prev <= t && t <= self.t {
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
    pub fn coord_fns(&'a self) -> [Box<dyn 'a + StateCoordFnTrait>; N] {
        std::array::from_fn(|i| {
            let coord_fn: Box<dyn 'a + StateCoordFnTrait> = Box::new(StateCoordFn::<'a, N, S> {
                state: self,
                coord: i,
            });
            coord_fn
        })
    }
}

pub trait StateFnMut<const N: usize, Ret> {
    /// Evaluate the function at the current state.
    fn eval<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Ret
    where
        [(); S * (S - 1) / 2]:;
    /// Evaluate the function at the previous step of the state.
    fn eval_prev<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Ret
    where
        [(); S * (S - 1) / 2]:;
    /// Evaluate the function at the state at the time `t` by means of [State::eval_all]
    fn eval_at<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>, t: f64) -> Ret
    where
        [(); S * (S - 1) / 2]:;
}

pub trait MutStateFnMut<const N: usize, Ret> {
    /// Evaluate the function at the current state.
    fn eval_mut<'b, 'c, const S: usize>(&mut self, state: &'b mut State<'c, N, S>) -> Ret
    where
        [(); S * (S - 1) / 2]:;
}

pub struct ConstantStateFnMut<F: FnMut<(), Output = Ret>, Ret>(pub F);
impl<F: FnMut<(), Output = Ret>, Ret, const N: usize> StateFnMut<N, Ret>
    for ConstantStateFnMut<F, Ret>
{
    fn eval<'b, const S: usize>(&mut self, _state: &'b State<'b, N, S>) -> Ret
    where
        [(); S * (S - 1) / 2]:,
    {
        (self.0)()
    }
    fn eval_prev<'b, const S: usize>(&mut self, _state: &'b State<'b, N, S>) -> Ret
    where
        [(); S * (S - 1) / 2]:,
    {
        (self.0)()
    }
    fn eval_at<'b, const S: usize>(&mut self, _state: &'b State<'b, N, S>, _t: f64) -> Ret
    where
        [(); S * (S - 1) / 2]:,
    {
        (self.0)()
    }
}
impl<F: FnMut<(), Output = Ret>, Ret, const N: usize> MutStateFnMut<N, Ret>
    for ConstantStateFnMut<F, Ret>
{
    fn eval_mut<'b, 'c, const S: usize>(&mut self, _state: &'b mut State<'c, N, S>) -> Ret
    where
        [(); S * (S - 1) / 2]:,
    {
        (self.0)()
    }
}

pub struct TimeStateFnMut<F: FnMut<(f64,), Output = Ret>, Ret>(pub F);
impl<F: FnMut<(f64,), Output = Ret>, Ret, const N: usize> StateFnMut<N, Ret>
    for TimeStateFnMut<F, Ret>
{
    fn eval<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Ret
    where
        [(); S * (S - 1) / 2]:,
    {
        (self.0)(state.t)
    }
    fn eval_prev<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Ret
    where
        [(); S * (S - 1) / 2]:,
    {
        (self.0)(state.t_prev)
    }
    fn eval_at<'b, const S: usize>(&mut self, _state: &'b State<'b, N, S>, t: f64) -> Ret
    where
        [(); S * (S - 1) / 2]:,
    {
        (self.0)(t)
    }
}
impl<F: FnMut<(f64,), Output = Ret>, Ret, const N: usize> MutStateFnMut<N, Ret>
    for TimeStateFnMut<F, Ret>
{
    fn eval_mut<'b, 'c, const S: usize>(&mut self, state: &'b mut State<'c, N, S>) -> Ret
    where
        [(); S * (S - 1) / 2]:,
    {
        (self.0)(state.t)
    }
}

pub struct TimeMutStateFnMut<F: for<'a> FnMut<(&'a mut f64,), Output = Ret>, Ret>(pub F);
impl<F: for<'a> FnMut<(&'a mut f64,), Output = Ret>, Ret, const N: usize> MutStateFnMut<N, Ret>
    for TimeMutStateFnMut<F, Ret>
{
    fn eval_mut<'b, 'c, const S: usize>(&mut self, state: &'b mut State<'c, N, S>) -> Ret
    where
        [(); S * (S - 1) / 2]:,
    {
        (self.0)(&mut state.t)
    }
}

pub struct ODEStateFnMut<const N: usize, F: FnMut<([f64; N],), Output = Ret>, Ret>(pub F);
impl<F: FnMut<([f64; N],), Output = Ret>, Ret, const N: usize> StateFnMut<N, Ret>
    for ODEStateFnMut<N, F, Ret>
{
    fn eval<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Ret
    where
        [(); S * (S - 1) / 2]:,
    {
        (self.0)(state.x)
    }
    fn eval_prev<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Ret
    where
        [(); S * (S - 1) / 2]:,
    {
        (self.0)(state.x_prev)
    }
    fn eval_at<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>, t: f64) -> Ret
    where
        [(); S * (S - 1) / 2]:,
    {
        (self.0)(state.eval_all(t))
    }
}
impl<F: for<'a> FnMut<([f64; N],), Output = Ret>, Ret, const N: usize> MutStateFnMut<N, Ret>
    for ODEStateFnMut<N, F, Ret>
{
    fn eval_mut<'b, 'c, const S: usize>(&mut self, state: &'b mut State<'c, N, S>) -> Ret
    where
        [(); S * (S - 1) / 2]:,
    {
        (self.0)(state.x)
    }
}

pub struct ODEMutStateFnMut<
    const N: usize,
    F: for<'a> FnMut<(&'a mut [f64; N],), Output = Ret>,
    Ret,
>(pub F);
impl<F: for<'a> FnMut<(&'a mut [f64; N],), Output = Ret>, Ret, const N: usize> MutStateFnMut<N, Ret>
    for ODEMutStateFnMut<N, F, Ret>
{
    fn eval_mut<'b, 'c, const S: usize>(&mut self, state: &'b mut State<'c, N, S>) -> Ret
    where
        [(); S * (S - 1) / 2]:,
    {
        (self.0)(&mut state.x)
    }
}

pub struct ODE2StateFnMut<const N: usize, F: FnMut<(f64, [f64; N]), Output = Ret>, Ret>(pub F);
impl<F: FnMut<(f64, [f64; N]), Output = Ret>, Ret, const N: usize> StateFnMut<N, Ret>
    for ODE2StateFnMut<N, F, Ret>
{
    fn eval<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Ret
    where
        [(); S * (S - 1) / 2]:,
    {
        (self.0)(state.t, state.x)
    }
    fn eval_prev<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Ret
    where
        [(); S * (S - 1) / 2]:,
    {
        (self.0)(state.t_prev, state.x_prev)
    }
    fn eval_at<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>, t: f64) -> Ret
    where
        [(); S * (S - 1) / 2]:,
    {
        (self.0)(t, state.eval_all(t))
    }
}
impl<F: for<'a> FnMut<(f64, [f64; N]), Output = Ret>, Ret, const N: usize> MutStateFnMut<N, Ret>
    for ODE2StateFnMut<N, F, Ret>
{
    fn eval_mut<'b, 'c, const S: usize>(&mut self, state: &'b mut State<'c, N, S>) -> Ret
    where
        [(); S * (S - 1) / 2]:,
    {
        (self.0)(state.t, state.x)
    }
}

pub struct ODE2MutStateFnMut<
    const N: usize,
    F: for<'a> FnMut<(&'a mut f64, &'a mut [f64; N]), Output = Ret>,
    Ret,
>(pub F);
impl<F: for<'a> FnMut<(&'a mut f64, &'a mut [f64; N]), Output = Ret>, Ret, const N: usize>
    MutStateFnMut<N, Ret> for ODE2MutStateFnMut<N, F, Ret>
{
    fn eval_mut<'b, 'c, const S: usize>(&mut self, state: &'b mut State<'c, N, S>) -> Ret
    where
        [(); S * (S - 1) / 2]:,
    {
        (self.0)(&mut state.t, &mut state.x)
    }
}

// struct dde_closure;
// impl dde_closure {
//     fn eval<const S: usize>(t: f64, [x]: [f64; 1], [x_]: [StateCoordFn<'_, 1, S>; 1]) -> [f64; 1] {
//         [-x + 2. * x_(t - 1.)]
//     }
// }

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

pub struct DDEStateFnMut<
    const N: usize,
    F: for<'a> FnMut<(f64, [f64; N], [Box<dyn 'a + StateCoordFnTrait>; N]), Output = Ret>,
    Ret,
>(pub F);
impl<
    F: for<'a> FnMut<(f64, [f64; N], [Box<dyn 'a + StateCoordFnTrait>; N]), Output = Ret>,
    Ret,
    const N: usize,
> StateFnMut<N, Ret> for DDEStateFnMut<N, F, Ret>
{
    fn eval<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Ret
    where
        [(); S * (S - 1) / 2]:,
    {
        (self.0)(state.t, state.x, state.coord_fns())
    }
    fn eval_prev<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Ret
    where
        [(); S * (S - 1) / 2]:,
    {
        (self.0)(state.t_prev, state.x_prev, state.coord_fns())
    }
    fn eval_at<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>, t: f64) -> Ret
    where
        [(); S * (S - 1) / 2]:,
    {
        (self.0)(t, state.eval_all(t), state.coord_fns())
    }
}
impl<
    F: for<'a> FnMut<(f64, [f64; N], [Box<dyn 'a + StateCoordFnTrait>; N]), Output = Ret>,
    Ret,
    const N: usize,
> MutStateFnMut<N, Ret> for DDEStateFnMut<N, F, Ret>
{
    fn eval_mut<'b, 'c, const S: usize>(&mut self, state: &'b mut State<'c, N, S>) -> Ret
    where
        [(); S * (S - 1) / 2]:,
    {
        (self.0)(state.t, state.x, state.coord_fns())
    }
}

// struct DDEMutStateFnMut<F>(F);
// impl<F: for<'a> FnMut<(&'a mut f64, &'a mut [f64; N]), Output = Ret>, Ret, const N: usize>
//     MutStateFnMut<N, Ret> for DDEMutStateFnMut<F>
// {
//     fn eval_mut<'b, const S: usize>(&mut self, state: &'b mut State<'b, N, S>) -> Ret {
//         (self.0)(&mut state.t, &mut state.x)
//     }
// }

// /// Enum that hold closures of different signatures, and manages how they are evaluated at the
// /// state.
// pub enum StateFn<'a, const N: usize, Ret> {
//     /// Function, that do not depend on the state
//     Constant(Box<dyn 'a + FnMut() -> Ret>),
//     /// Function, that only depends on the time of the state
//     Time(Box<dyn 'a + FnMut(f64) -> Ret>),
//     /// Function, that depends on the coordinates of the state
//     ODE(Box<dyn 'a + FnMut([f64; N]) -> Ret>),
//     /// Function, that depends on the time and the coordinates of the state
//     ODE2(Box<dyn 'a + FnMut(f64, [f64; N]) -> Ret>),
//     /// Function, that depends on the past of the state.
//     ///
//     /// The third argument in the function is vector of [StateCoordFn]s, provided by [State::coord_fns].
//     DDE(Box<dyn 'a + FnMut(f64, [f64; N], [Box<dyn '_ + StateCoordFnTrait>; N]) -> Ret>),
// }
//
// impl<'a, const N: usize, Ret> StateFn<'a, N, Ret> {
//     /// Evaluate the function at the current state.
//     pub fn eval<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Ret {
//         match self {
//             StateFn::Constant(f) => f(),
//             StateFn::Time(f) => f(state.t),
//             StateFn::ODE(f) => f(state.x),
//             StateFn::ODE2(f) => f(state.t, state.x),
//             StateFn::DDE(f) => f(state.t, state.x, state.coord_fns()),
//         }
//     }
//
//     /// Evaluate the function at the state at the time `t` by means of [State::eval_all]
//     pub fn eval_at<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>, t: f64) -> Ret {
//         match self {
//             StateFn::Constant(f) => f(),
//             StateFn::Time(f) => f(t),
//             StateFn::ODE(f) => f(state.eval_all(t)),
//             StateFn::ODE2(f) => f(t, state.eval_all(t)),
//             StateFn::DDE(f) => f(t, state.eval_all(t), state.coord_fns()),
//         }
//     }
//
//     /// Evaluate the function at the previous step of the state.
//     pub fn eval_prev<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Ret {
//         match self {
//             StateFn::Constant(f) => f(),
//             StateFn::Time(f) => f(state.t_prev),
//             StateFn::ODE(f) => f(state.x_prev),
//             StateFn::ODE2(f) => f(state.t_prev, state.x_prev),
//             StateFn::DDE(f) => f(state.t_prev, state.x_prev, state.coord_fns()),
//         }
//     }
//
//     /// Constructor for [StateFn::Constant] variant.
//     ///
//     /// Shorthand for `StateFn::Constant(Box::new(f))`
//     pub fn constant(f: impl 'a + FnMut() -> Ret) -> Self {
//         StateFn::Constant(Box::new(f))
//     }
//     /// Constructor for [StateFn::Time] variant.
//     ///
//     /// Shorthand for `StateFn::Time(Box::new(f))`
//     pub fn time(f: impl 'a + FnMut(f64) -> Ret) -> Self {
//         StateFn::Time(Box::new(f))
//     }
//     /// Constructor for [StateFn::ODE] variant.
//     ///
//     /// Shorthand for `StateFn::ODE(Box::new(f))`
//     pub fn ode(f: impl 'a + FnMut([f64; N]) -> Ret) -> Self {
//         StateFn::ODE(Box::new(f))
//     }
//     /// Constructor for [StateFn::ODE2] variant.
//     ///
//     /// Shorthand for `StateFn::ODE2(Box::new(f))`
//     pub fn ode2(f: impl 'a + FnMut(f64, [f64; N]) -> Ret) -> Self {
//         StateFn::ODE2(Box::new(f))
//     }
//     /// Constructor for [StateFn::DDE] variant.
//     ///
//     /// Shorthand for `StateFn::DDE(Box::new(f))`
//     pub fn dde(
//         f: impl 'a + FnMut(f64, [f64; N], [Box<dyn '_ + StateCoordFnTrait>; N]) -> Ret,
//     ) -> Self {
//         StateFn::DDE(Box::new(f))
//     }
// }
//
// /// Same as [StateFn], but can mutate the state.
// ///
// /// It has the variants present in [StateFn], but introduces additional variants, which can mutate
// /// the state. The implementation of the methods, comparing to [StateFn], accepts a mutable reference
// /// to [State] instead of an immutable reference.
// pub enum MutStateFn<'a, const N: usize, Ret> {
//     /// Same as [StateFn::Constant]
//     Constant(Box<dyn 'a + FnMut() -> Ret>),
//     /// Same as [StateFn::Time]
//     Time(Box<dyn 'a + FnMut(f64) -> Ret>),
//     /// Same as [StateFn::Time], but accepts a mutable reference to the state time.
//     TimeMut(Box<dyn 'a + FnMut(&mut f64) -> Ret>),
//     /// Same as [StateFn::ODE]
//     ODE(Box<dyn 'a + FnMut([f64; N]) -> Ret>),
//     /// Same as [StateFn::Time], but accepts a mutable reference to the state coordinate vector.
//     ODEMut(Box<dyn 'a + FnMut(&mut [f64; N]) -> Ret>),
//     /// Same as [StateFn::ODE2]
//     ODE2(Box<dyn 'a + FnMut(f64, [f64; N]) -> Ret>),
//     /// Same as [StateFn::Time], but accepts a mutable references to the state time and coordinate vector.
//     ODE2Mut(Box<dyn 'a + FnMut(&mut f64, &mut [f64; N]) -> Ret>),
//     /// Same as [StateFn::DDE]
//     DDE(Box<dyn 'a + FnMut(f64, [f64; N], [Box<dyn '_ + StateCoordFnTrait>; N]) -> Ret>),
//     // DDEMut(Box<dyn 'a + Fn(&mut f64, &mut [f64; N], [Box<dyn '_ + StateCoordFnTrait>; N]) -> Ret>),
// }
//
// impl<'a, const N: usize, Ret> MutStateFn<'a, N, Ret> {
//     /// Call the function at the current state.
//     pub fn eval<'b, const S: usize>(&mut self, state: &'b mut State<N, S>) -> Ret {
//         match self {
//             MutStateFn::Constant(f) => f(),
//             MutStateFn::Time(f) => f(state.t),
//             MutStateFn::TimeMut(f) => f(&mut state.t),
//             MutStateFn::ODE(f) => f(state.x),
//             MutStateFn::ODEMut(f) => f(&mut state.x),
//             MutStateFn::ODE2(f) => f(state.t, state.x),
//             MutStateFn::ODE2Mut(f) => f(&mut state.t, &mut state.x),
//             MutStateFn::DDE(f) => f(state.t, state.x, state.coord_fns()),
//             // StateFnMut::DDEMut(f) => {f(&mut state.t, &mut state.x, state.coord_fns()) } // Bad borrowing
//         }
//     }
//
//     /// Evaluate the function at the state at the time `t` by means of [State::eval_all].
//     ///
//     /// Note that in this case, state cannot be mutated (hence the immutability of the reference),
//     /// because computed interpolated values are temporary. In other words, you can't change the
//     /// past.
//     ///
//     /// Also, in the future, ways to alter the past might be added, but they probably will be
//     /// limited to linear mappings on the values in [State::k_seq], used for renormalization of the
//     /// linear equations, which is necessary for computation of the Lyapunov exponents.
//     pub fn eval_at<'b, const S: usize>(&mut self, state: &'b State<N, S>, mut t: f64) -> Ret {
//         match self {
//             MutStateFn::Constant(f) => f(),
//             MutStateFn::Time(f) => f(t),
//             MutStateFn::TimeMut(f) => f(&mut t),
//             MutStateFn::ODE(f) => f(state.eval_all(t)),
//             MutStateFn::ODEMut(f) => f(&mut state.eval_all(t)),
//             MutStateFn::ODE2(f) => f(t, state.eval_all(t)),
//             MutStateFn::ODE2Mut(f) => {
//                 let t2 = t;
//                 f(&mut t, &mut state.eval_all(t2))
//             }
//             MutStateFn::DDE(f) => f(t, state.eval_all(t), state.coord_fns()),
//             // StateFnMut::DDEMut(f) =>  {let t2 = t; f(&mut t, &mut state.eval_all(t2), state.coord_fns())},
//         }
//     }
//
//     /// Constructor for [MutStateFn::Constant] variant.
//     ///
//     /// Shorthand for `MutStateFn::Constant(Box::new(f))`
//     pub fn constant(f: impl 'a + FnMut() -> Ret) -> Self {
//         MutStateFn::Constant(Box::new(f))
//     }
//     /// Constructor for [MutStateFn::Time] variant.
//     ///
//     /// Shorthand for `MutStateFn::Time(Box::new(f))`
//     pub fn time(f: impl 'a + FnMut(f64) -> Ret) -> Self {
//         MutStateFn::Time(Box::new(f))
//     }
//     /// Constructor for [MutStateFn::TimeMut] variant.
//     ///
//     /// Shorthand for `MutStateFn::TimeMut(Box::new(f))`
//     pub fn time_mut(f: impl 'a + FnMut(&mut f64) -> Ret) -> Self {
//         MutStateFn::TimeMut(Box::new(f))
//     }
//     /// Constructor for [MutStateFn::ODE] variant.
//     ///
//     /// Shorthand for `MutStateFn::ODE(Box::new(f))`
//     pub fn ode(f: impl 'a + FnMut([f64; N]) -> Ret) -> Self {
//         MutStateFn::ODE(Box::new(f))
//     }
//     /// Constructor for [MutStateFn::ODEMut] variant.
//     ///
//     /// Shorthand for `MutStateFn::ODEMut(Box::new(f))`
//     pub fn ode_mut(f: impl 'a + FnMut(&mut [f64; N]) -> Ret) -> Self {
//         MutStateFn::ODEMut(Box::new(f))
//     }
//     /// Constructor for [MutStateFn::ODE2] variant.
//     ///
//     /// Shorthand for `MutStateFn::ODE2(Box::new(f))`
//     pub fn ode2(f: impl 'a + FnMut(f64, [f64; N]) -> Ret) -> Self {
//         MutStateFn::ODE2(Box::new(f))
//     }
//     /// Constructor for [MutStateFn::ODE2Mut] variant.
//     ///
//     /// Shorthand for `MutStateFn::ODE2Mut(Box::new(f))`
//     pub fn ode2_mut(f: impl 'a + FnMut(&mut f64, &mut [f64; N]) -> Ret) -> Self {
//         MutStateFn::ODE2Mut(Box::new(f))
//     }
//     /// Constructor for [MutStateFn::DDE] variant.
//     ///
//     /// Shorthand for `MutStateFn::DDE(Box::new(f))`
//     pub fn dde(
//         f: impl 'a + FnMut(f64, [f64; N], [Box<dyn '_ + StateCoordFnTrait>; N]) -> Ret,
//     ) -> Self {
//         MutStateFn::DDE(Box::new(f))
//     }
// }

/// Struct that holds a reference to the state, and the coordinate index.
///
/// It implements Fn() -> f64 and Fn(f64) -> f64 traits, as evaluation of current and past state
/// respectively.
pub struct StateCoordFn<'a, const N: usize, const S: usize>
where
    [(); S * (S - 1) / 2]:,
{
    /// Reference to the state
    pub state: &'a State<'a, N, S>,
    /// Coordinate index
    pub coord: usize,
}

/// Trait to erase generic parameter S from StateCoordFn
pub trait StateCoordFnTrait: Fn() -> f64 + Fn(f64) -> f64 {
    /// evaluate the derivative
    fn d(&self, t: f64) -> f64;
}

impl<'a, const N: usize, const S: usize> FnOnce<()> for StateCoordFn<'a, N, S>
where
    [(); S * (S - 1) / 2]:,
{
    type Output = f64;
    #[inline]
    extern "rust-call" fn call_once(self, _: ()) -> Self::Output {
        self.state.x[self.coord]
    }
}

impl<'a, const N: usize, const S: usize> FnMut<()> for StateCoordFn<'a, N, S>
where
    [(); S * (S - 1) / 2]:,
{
    #[inline]
    extern "rust-call" fn call_mut(&mut self, _: ()) -> Self::Output {
        self.state.x[self.coord]
    }
}

impl<'a, const N: usize, const S: usize> Fn<()> for StateCoordFn<'a, N, S>
where
    [(); S * (S - 1) / 2]:,
{
    #[inline]
    extern "rust-call" fn call(&self, _: ()) -> Self::Output {
        self.state.x[self.coord]
    }
}

impl<'a, const N: usize, const S: usize> FnOnce<(f64,)> for StateCoordFn<'a, N, S>
where
    [(); S * (S - 1) / 2]:,
{
    type Output = f64;
    #[inline]
    extern "rust-call" fn call_once(self, arg: (f64,)) -> Self::Output {
        self.state.eval(arg.0, self.coord)
    }
}

impl<'a, const N: usize, const S: usize> FnMut<(f64,)> for StateCoordFn<'a, N, S>
where
    [(); S * (S - 1) / 2]:,
{
    #[inline]
    extern "rust-call" fn call_mut(&mut self, arg: (f64,)) -> Self::Output {
        self.state.eval(arg.0, self.coord)
    }
}

impl<'a, const N: usize, const S: usize> Fn<(f64,)> for StateCoordFn<'a, N, S>
where
    [(); S * (S - 1) / 2]:,
{
    extern "rust-call" fn call(&self, arg: (f64,)) -> Self::Output {
        self.state.eval(arg.0, self.coord)
    }
}

impl<'a, const N: usize, const S: usize> StateCoordFnTrait for StateCoordFn<'a, N, S>
where
    [(); S * (S - 1) / 2]:,
{
    fn d(&self, t: f64) -> f64 {
        self.state.eval_derivative(t, self.coord)
    }
}
// impl<'state, const N: usize, const S: usize, IF: Fn(f64) -> [f64; N], DIF: Fn(f64) -> [f64; N]>
//     CoordFn<'state, N, S, crate::util::with_derivative::Differentiable<IF, DIF>>
// {
//     pub fn d(&self, t: f64) -> f64 {
//         return self.state_ref.eval_derivative(t, self.coordinate);
//     }
// }
