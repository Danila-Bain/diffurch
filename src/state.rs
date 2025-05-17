//! Defines [State], the core object which is acted upon during integration.

/// [State] is an object that is primarily used internally.
pub struct State<'a, const N: usize, const S: usize> {
    /// time of the state at the current step
    pub(crate) t: f64,
    /// time of the state at the previous step
    pub(crate) t_prev: f64,
    /// initial time of the state
    t_init: f64,
    /// length of past history stored in state,
    ///
    /// It must be >= than largest delay encountered in delay differential equation.
    ///
    /// It may be 0., may be f64::INFINITE.
    ///
    /// For negative values, solver will panic.
    pub(crate) t_span: f64,
    /// time instances of past steps
    ///
    /// The past values that are no longer needed are pop'ed during computation according to [State::t_span].
    t_seq: std::collections::VecDeque<f64>,

    /// position of the state at the current step
    pub(crate) x: [f64; N],
    /// position of the state at the previous step
    pub(crate) x_prev: [f64; N],
    /// initial condition used to initialize or evaluate the state at times before [State::t_init].
    x_init: crate::InitialCondition<'a, N>,
    /// state values of past steps
    ///
    /// The past values that are no longer needed are pop'ed during computation according to [State::t_span].
    x_seq: std::collections::VecDeque<[f64; N]>,

    k: [[f64; N]; S],
    k_seq: std::collections::VecDeque<[[f64; N]; S]>,

    rk: &'a crate::rk::RungeKuttaTable<'a, S>,
}

// pub trait StateTrait<const N: usize> {
//     fn x(&self) -> [f64; N];
//     fn t(&self) -> f64;
// }

// impl<const N: usize, const S:usize> StateTrait<N> for State<N,S> {
//     fn x(&self) -> [f64; N] {
//         self.x
//     }
//
//     fn t(&self) -> f64 {
//         self.t
//     }
// }

impl<'a, const N: usize, const S: usize> State<'a, N, S> {
    pub fn new(
        t_init: f64,
        x_init: crate::InitialCondition<'a, N>,
        t_span: f64,
        rk: &'a crate::rk::RungeKuttaTable<'a, S>,
    ) -> Self {
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

    pub fn make_step(&mut self, rhs: &mut StateFn<'a, N, [f64; N]>, t_step: f64) {
        self.t_prev = self.t;
        self.x_prev = self.x;

        for i in 0..S {
            self.t = self.t_prev + self.rk.c[i] * t_step;

            self.x = std::array::from_fn(|k| {
                self.x_prev[k]
                    + t_step * (0..i).fold(0., |acc, j| acc + self.rk.a[i][j] * self.k[j][k])
            });
            self.k[i] = rhs.eval(self);
        }

        self.x = std::array::from_fn(|k| {
            self.x_prev[k] + t_step * (0..S).fold(0., |acc, j| acc + self.rk.b[j] * self.k[j][k])
        });
        self.t = self.t_prev + t_step;
    }

    pub fn make_zero_step(&mut self) {
        self.t_prev = self.t;
        self.x_prev = self.x;
        self.k = [[0.; N]; S];
    }

    pub fn undo_step(&mut self) {
        self.t = self.t_prev;
        self.x = self.x_prev;
    }

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

pub enum StateFn<'a, const N: usize, Ret> {
    Constant(Box<dyn 'a + FnMut() -> Ret>),
    Time(Box<dyn 'a + FnMut(f64) -> Ret>),
    ODE(Box<dyn 'a + FnMut([f64; N]) -> Ret>),
    ODE2(Box<dyn 'a + FnMut(f64, [f64; N]) -> Ret>),
    DDE(Box<dyn 'a + FnMut(f64, [f64; N], [Box<dyn '_ + StateCoordFnTrait>; N]) -> Ret>),
}

impl<'a, const N: usize, Ret> StateFn<'a, N, Ret> {
    pub fn eval<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Ret {
        match self {
            StateFn::Constant(f) => f(),
            StateFn::Time(f) => f(state.t),
            StateFn::ODE(f) => f(state.x),
            StateFn::ODE2(f) => f(state.t, state.x),
            StateFn::DDE(f) => f(state.t, state.x, state.coord_fns()),
        }
    }

    pub fn eval_at<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>, t: f64) -> Ret {
        match self {
            StateFn::Constant(f) => f(),
            StateFn::Time(f) => f(t),
            StateFn::ODE(f) => f(state.eval_all(t)),
            StateFn::ODE2(f) => f(t, state.eval_all(t)),
            StateFn::DDE(f) => f(t, state.eval_all(t), state.coord_fns()),
        }
    }

    pub fn eval_prev<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Ret {
        match self {
            StateFn::Constant(f) => f(),
            StateFn::Time(f) => f(state.t_prev),
            StateFn::ODE(f) => f(state.x_prev),
            StateFn::ODE2(f) => f(state.t_prev, state.x_prev),
            StateFn::DDE(f) => f(state.t_prev, state.x_prev, state.coord_fns()),
        }
    }

    pub fn constant(f: impl 'a + FnMut() -> Ret) -> Self {
        StateFn::Constant(Box::new(f))
    }
    pub fn time(f: impl 'a + FnMut(f64) -> Ret) -> Self {
        StateFn::Time(Box::new(f))
    }
    pub fn ode(f: impl 'a + FnMut([f64; N]) -> Ret) -> Self {
        StateFn::ODE(Box::new(f))
    }
    pub fn ode2(f: impl 'a + FnMut(f64, [f64; N]) -> Ret) -> Self {
        StateFn::ODE2(Box::new(f))
    }
    pub fn dde(
        f: impl 'a + FnMut(f64, [f64; N], [Box<dyn '_ + StateCoordFnTrait>; N]) -> Ret,
    ) -> Self {
        StateFn::DDE(Box::new(f))
    }
}

pub enum MutStateFn<'a, const N: usize, Ret> {
    Constant(Box<dyn 'a + FnMut() -> Ret>),
    Time(Box<dyn 'a + FnMut(f64) -> Ret>),
    TimeMut(Box<dyn 'a + FnMut(&mut f64) -> Ret>),
    ODE(Box<dyn 'a + FnMut([f64; N]) -> Ret>),
    ODEMut(Box<dyn 'a + FnMut(&mut [f64; N]) -> Ret>),
    ODE2(Box<dyn 'a + FnMut(f64, [f64; N]) -> Ret>),
    ODE2Mut(Box<dyn 'a + FnMut(&mut f64, &mut [f64; N]) -> Ret>),
    DDE(Box<dyn 'a + FnMut(f64, [f64; N], [Box<dyn '_ + StateCoordFnTrait>; N]) -> Ret>),
    // DDEMut(Box<dyn 'a + Fn(&mut f64, &mut [f64; N], [Box<dyn '_ + StateCoordFnTrait>; N]) -> Ret>),
}

impl<'a, const N: usize, Ret> MutStateFn<'a, N, Ret> {
    pub fn eval<'b, const S: usize>(&mut self, state: &'b mut State<N, S>) -> Ret {
        match self {
            MutStateFn::Constant(f) => f(),
            MutStateFn::Time(f) => f(state.t),
            MutStateFn::TimeMut(f) => f(&mut state.t),
            MutStateFn::ODE(f) => f(state.x),
            MutStateFn::ODEMut(f) => f(&mut state.x),
            MutStateFn::ODE2(f) => f(state.t, state.x),
            MutStateFn::ODE2Mut(f) => f(&mut state.t, &mut state.x),
            MutStateFn::DDE(f) => f(state.t, state.x, state.coord_fns()),
            // StateFnMut::DDEMut(f) => {f(&mut state.t, &mut state.x, state.coord_fns()) } // Bad borrowing
        }
    }

    pub fn eval_at<'b, const S: usize>(&mut self, state: &'b State<N, S>, mut t: f64) -> Ret {
        match self {
            MutStateFn::Constant(f) => f(),
            MutStateFn::Time(f) => f(t),
            MutStateFn::TimeMut(f) => f(&mut t),
            MutStateFn::ODE(f) => f(state.eval_all(t)),
            MutStateFn::ODEMut(f) => f(&mut state.eval_all(t)),
            MutStateFn::ODE2(f) => f(t, state.eval_all(t)),
            MutStateFn::ODE2Mut(f) => {
                let t2 = t;
                f(&mut t, &mut state.eval_all(t2))
            }
            MutStateFn::DDE(f) => f(t, state.eval_all(t), state.coord_fns()),
            // StateFnMut::DDEMut(f) =>  {let t2 = t; f(&mut t, &mut state.eval_all(t2), state.coord_fns())},
        }
    }

    pub fn constant(f: impl 'a + FnMut() -> Ret) -> Self {
        MutStateFn::Constant(Box::new(f))
    }
    pub fn time(f: impl 'a + FnMut(f64) -> Ret) -> Self {
        MutStateFn::Time(Box::new(f))
    }
    pub fn time_mut(f: impl 'a + FnMut(&mut f64) -> Ret) -> Self {
        MutStateFn::TimeMut(Box::new(f))
    }
    pub fn ode(f: impl 'a + FnMut([f64; N]) -> Ret) -> Self {
        MutStateFn::ODE(Box::new(f))
    }
    pub fn ode_mut(f: impl 'a + FnMut(&mut [f64; N]) -> Ret) -> Self {
        MutStateFn::ODEMut(Box::new(f))
    }
    pub fn ode2(f: impl 'a + FnMut(f64, [f64; N]) -> Ret) -> Self {
        MutStateFn::ODE2(Box::new(f))
    }
    pub fn ode2_mut(f: impl 'a + FnMut(&mut f64, &mut [f64; N]) -> Ret) -> Self {
        MutStateFn::ODE2Mut(Box::new(f))
    }
    pub fn dde(
        f: impl 'a + FnMut(f64, [f64; N], [Box<dyn '_ + StateCoordFnTrait>; N]) -> Ret,
    ) -> Self {
        MutStateFn::DDE(Box::new(f))
    }
}

pub struct StateCoordFn<'a, const N: usize, const S: usize> {
    pub state: &'a State<'a, N, S>,
    pub coord: usize,
}

pub trait StateCoordFnTrait: Fn() -> f64 + Fn(f64) -> f64 {
    fn d(&self, t: f64) -> f64;
}

impl<'a, const N: usize, const S: usize> FnOnce<()> for StateCoordFn<'a, N, S> {
    type Output = f64;
    #[inline]
    extern "rust-call" fn call_once(self, _: ()) -> Self::Output {
        self.state.x[self.coord]
    }
}

impl<'a, const N: usize, const S: usize> FnMut<()> for StateCoordFn<'a, N, S> {
    #[inline]
    extern "rust-call" fn call_mut(&mut self, _: ()) -> Self::Output {
        self.state.x[self.coord]
    }
}

impl<'a, const N: usize, const S: usize> Fn<()> for StateCoordFn<'a, N, S> {
    #[inline]
    extern "rust-call" fn call(&self, _: ()) -> Self::Output {
        self.state.x[self.coord]
    }
}

impl<'a, const N: usize, const S: usize> FnOnce<(f64,)> for StateCoordFn<'a, N, S> {
    type Output = f64;
    #[inline]
    extern "rust-call" fn call_once(self, arg: (f64,)) -> Self::Output {
        self.state.eval(arg.0, self.coord)
    }
}

impl<'a, const N: usize, const S: usize> FnMut<(f64,)> for StateCoordFn<'a, N, S> {
    #[inline]
    extern "rust-call" fn call_mut(&mut self, arg: (f64,)) -> Self::Output {
        self.state.eval(arg.0, self.coord)
    }
}

impl<'a, const N: usize, const S: usize> Fn<(f64,)> for StateCoordFn<'a, N, S> {
    extern "rust-call" fn call(&self, arg: (f64,)) -> Self::Output {
        self.state.eval(arg.0, self.coord)
    }
}

impl<'a, const N: usize, const S: usize> StateCoordFnTrait for StateCoordFn<'a, N, S> {
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
