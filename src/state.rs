use crate::{Equation, InitialCondition, rk::RungeKuttaTable};

use std::collections::VecDeque;

pub struct State<'a, const N: usize, const S: usize> {
    pub t: f64,
    pub t_init: f64,
    pub t_prev: f64,
    pub t_span: f64,
    pub t_seq: VecDeque<f64>,

    pub x: [f64; N],
    pub x_init: InitialCondition<'a, N>,
    pub x_prev: [f64; N],
    pub x_err: [f64; N],
    pub x_seq: VecDeque<[f64; N]>,

    k: [[f64; N]; S],
    k_seq: VecDeque<[[f64; N]; S]>,

    rk: &'a RungeKuttaTable<'a, S>,
    rhs: StateFn<'a, N, [f64; N]>,
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
        x_init: InitialCondition<'a, N>,
        eq: Equation<'a, N>,
        rk: &'a RungeKuttaTable<'a, S>,
    ) -> Self {
        let x = match &x_init {
            &InitialCondition::Point(value) => value,
            InitialCondition::Function(f) | InitialCondition::FunctionWithDerivative(f, _) => {
                f(t_init)
            }
        };

        Self {
            t_init,
            t: t_init,
            t_prev: t_init,
            t_span: eq.max_delay,
            t_seq: VecDeque::from([t_init]),

            x_init,
            x,
            x_prev: x.clone(),
            x_err: [0.; N],
            x_seq: VecDeque::from([x.clone()]),

            k: [[0.; N]; S],
            k_seq: VecDeque::new(),

            rk,
            rhs: eq.rhs,
        }
    }
}

impl<'a, const N: usize, const S: usize> State<'a, N, S> {
    pub fn push_current(&mut self) {
        self.t_seq.push_back(self.t);
        self.x_seq.push_back(self.x);
        self.k_seq.push_back(self.k);
        while self.t_prev - self.t_span
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

    pub fn make_step(&mut self, t_step: f64) {
        self.t_prev = self.t;
        self.x_prev = self.x;

        for i in 0..S {
            self.t = self.t_prev + self.rk.c[i] * t_step;

            self.x = std::array::from_fn(|k| {
                self.x_prev[k]
                    + t_step * (0..i).fold(0., |acc, j| acc + self.rk.a[i][j] * self.k[j][k])
            });
            self.k[i] = self.rhs.eval(self);
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

    pub fn eval_all(&self, t: f64) -> [f64; N] {
        if t <= self.t_init {
            self.x_init.eval(t)
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
            let theta = (t - t_prev) / t_step;

            return std::array::from_fn(|i| {
                x_prev[i] + t_step * (0..S).fold(0., |acc, j| acc + self.rk.bi[j](theta) * k[j][i])
            });
        }
    }

    pub fn eval(&self, t: f64, coordinate: usize) -> f64 {
        if t <= self.t_init {
            self.x_init.eval(t)[coordinate]
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
            let theta = (t - t_prev) / t_step;
            return x_prev
                + t_step * (0..S).fold(0., |acc, j| acc + self.rk.bi[j](theta) * k[j][coordinate]);
        }
    }

    pub fn eval_derivative(&self, t: f64, coordinate: usize) -> f64 {
        if t <= self.t_init {
            self.x_init.eval_d(t)[coordinate]
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

            // let x_prev = &self.x_seq[i - 1][coordinate];
            let k = &self.k_seq[i - 1];
            let t_prev = self.t_seq[i - 1];
            let t_next = self.t_seq[i];
            let t_step = t_next - t_prev;
            let theta = (t - t_prev) / t_step;
            return (0..S).fold(0., |acc, j| acc + self.rk.bi[j].d(theta) * k[j][coordinate]);
        }
    }
}

pub enum StateFn<'a, const N: usize, Ret> {
    Constant(Box<dyn 'a + Fn() -> Ret>),
    Time(Box<dyn 'a + Fn(f64) -> Ret>),
    ODE(Box<dyn 'a + Fn([f64; N]) -> Ret>),
    ODE2(Box<dyn 'a + Fn(f64, [f64; N]) -> Ret>),
    DDE(Box<dyn 'a + Fn(f64, [f64; N], [Box<dyn '_ + StateCoordFnTrait>; N]) -> Ret>),
}

impl<'a, const N: usize, Ret> StateFn<'a, N, Ret> {
    pub fn eval<'b, const S: usize>(&self, state: &'b State<'b, N, S>) -> Ret {
        match self {
            StateFn::Constant(f) => f(),
            StateFn::Time(f) => f(state.t),
            StateFn::ODE(f) => f(state.x),
            StateFn::ODE2(f) => f(state.t, state.x),
            StateFn::DDE(f) => f(
                state.t,
                state.x,
                std::array::from_fn(|i| {
                    let coord_fn: Box<dyn '_ + StateCoordFnTrait> =
                        Box::new(StateCoordFn::<'b, N, S> { state, coord: i });
                    coord_fn
                }),
            ),
        }
    }

    pub fn eval_at<'b, const S: usize>(&self, state: &'b State<'b, N, S>, t: f64) -> Ret {
        match self {
            StateFn::Constant(f) => f(),
            StateFn::Time(f) => f(t),
            StateFn::ODE(f) => f(state.eval_all(t)),
            StateFn::ODE2(f) => f(t, state.eval_all(t)),
            StateFn::DDE(f) => f(
                t,
                state.eval_all(t),
                std::array::from_fn(|i| {
                    let coord_fn: Box<dyn '_ + StateCoordFnTrait> =
                        Box::new(StateCoordFn::<'b, N, S> { state, coord: i });
                    coord_fn
                }),
            ),
        }
    }
}

pub enum StateFnMut<'a, const N: usize, Ret> {
    Constant(Box<dyn 'a + FnMut() -> Ret>),
    Time(Box<dyn 'a + FnMut(f64) -> Ret>),
    ODE(Box<dyn 'a + FnMut([f64; N]) -> Ret>),
    ODE2(Box<dyn 'a + FnMut(f64, [f64; N]) -> Ret>),
}

impl<'a, const N: usize, Ret> StateFnMut<'a, N, Ret> {
    pub fn eval<const S: usize>(&mut self, state: &mut State<N, S>) -> Ret {
        match self {
            StateFnMut::Constant(f) => f(),
            StateFnMut::Time(f) => f(state.t),
            StateFnMut::ODE(f) => f(state.x),
            StateFnMut::ODE2(f) => f(state.t, state.x),
        }
    }

    pub fn eval_at<const S: usize>(&mut self, state: &State<N, S>, t: f64) -> Ret {
        match self {
            StateFnMut::Constant(f) => f(),
            StateFnMut::Time(f) => f(t),
            StateFnMut::ODE(f) => f(state.eval_all(t)),
            StateFnMut::ODE2(f) => f(t, state.eval_all(t)),
        }
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

