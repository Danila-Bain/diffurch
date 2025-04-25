use crate::rk::*;

use std::collections::VecDeque;

pub struct State<const N: usize, const S: usize, F: Fn(f64) -> [f64; N]> {
    pub t: f64,
    pub t_init: f64,
    pub t_prev: f64,
    pub t_step: f64,
    pub t_span: f64,
    pub t_seq: VecDeque<f64>,

    pub x: [f64; N],
    pub x_init: F,
    pub x_prev: [f64; N],
    pub x_err: [f64; N],
    pub x_seq: VecDeque<[f64; N]>,

    k: [[f64; N]; S],
    k_seq: VecDeque<[[f64; N]; S]>,

    rk: &'static RungeKuttaTable<'static, S>,
}


impl<const N: usize, const S: usize, F: Fn(f64) -> [f64; N]> State<N, S, F> {
    pub fn new(t_init: f64, x_init: F, rk: &'static RungeKuttaTable<S>) -> Self {
        let x = x_init(t_init);

        Self {
            t_init,
            t: t_init,
            t_prev: t_init,
            t_step: 0.,
            t_span: 0.,
            t_seq: VecDeque::from([t_init]),

            x_init,
            x,
            x_prev: x.clone(),
            x_err: [0.; N],
            x_seq: VecDeque::from([x.clone()]),

            k: [[0.; N]; S],
            k_seq: VecDeque::new(),

            rk: &rk,
        }
    }
}

impl<const N: usize, const S: usize, F: Fn(f64) -> [f64; N]> State<N, S, F> {
    pub fn push_current(&mut self) {
        self.t_seq.push_back(self.t);
        self.x_seq.push_back(self.x);
        self.k_seq.push_back(self.k);
        while self.t - self.t_span
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

    pub fn make_step<Args, RHS>(&mut self, rhs: &RHS)
    where
        Args: std::marker::Tuple + for<'a> FromState<&'a Self>,
        RHS: Fn<Args, Output = [f64; N]>,
    {
        self.t_prev = self.t;
        self.x_prev = self.x;

        for i in 0..S {
            self.t = self.t_prev + self.rk.c[i] * self.t_step;

            self.x = std::array::from_fn(|k| {
                self.x_prev[k]
                    + self.t_step * (0..i).fold(0., |acc, j| acc + self.rk.a[i][j] * self.k[j][k])
            });
            self.k[i] = rhs.call(Args::from_state(self));
        }

        self.x = std::array::from_fn(|k| {
            self.x_prev[k]
                + self.t_step * (0..S).fold(0., |acc, j| acc + self.rk.b[j] * self.k[j][k])
        });
        self.t = self.t_prev + self.t_step;
    }

    pub fn make_zero_step(&mut self) {
        self.t_prev = self.t;
        self.x_prev = self.x;
        self.k = [[0.; N]; S];
        self.push_current();
    }

    pub fn eval(&self, t: f64) -> [f64; N] {
        if t < self.t_init {
            return (self.x_init)(t);
        } else {
            let i = self.t_seq.partition_point(|t_i| t_i < &t); // first i : t_seq[i] >= t
            if i == 0 {
                panic!("evaluation of state in deleted time range");
            } else if i == self.t_seq.len() {
                panic!("evaluation of state in a not yet computed time range");
            }

            let x_prev = &self.x_seq[i - 1];
            let k = &self.k_seq[i - 1];
            let t_prev = self.t_seq[i - 1];
            let t_next = self.t_seq[i - 1];
            let t_step = t_prev - t_next;
            let theta = (t - t_prev) / t_step;

            return std::array::from_fn(|i| {
                x_prev[i] + t_step * (0..S).fold(0., |acc, j| acc + self.rk.bi[j](theta) * k[j][i])
            });
        }
    }

    pub fn eval_i(&self, t: f64, coordinate: usize) -> f64 {
        if t < self.t_init {
            return (self.x_init)(t)[coordinate];
        } else {
            let i = self.t_seq.partition_point(|t_i| t_i < &t); // first i : t_seq[i] >= t
            if i == 0 {
                panic!("evaluation of state in deleted time range");
            } else if i == self.t_seq.len() {
                panic!("evaluation of state in a not yet computed time range");
            }

            let x_prev = &self.x_seq[i - 1][coordinate];
            let k = &self.k_seq[i - 1];
            let t_prev = self.t_seq[i - 1];
            let t_next = self.t_seq[i - 1];
            let t_step = t_prev - t_next;
            let theta = (t - t_prev) / t_step;
            return x_prev
                + t_step * (0..S).fold(0., |acc, j| acc + self.rk.bi[j](theta) * k[j][coordinate]);
        }
    }

    // pub fn x_eval_i(&self, coodrdinate: usize) -> Box<dyn Fn(f64) -> f64> {
    //     Box::new(move |t| (&self).eval_i(t, coodrdinate))
    // }

    pub fn eval_derivative(&self, _t: f64) -> [f64; N] {
        todo!()
    }

    pub fn eval_nth_derivative<const ORDER: usize>(&self, _t: f64) -> [f64; N] {
        todo!()
    }
}

pub trait FromState<T> {
    fn from_state(t: T) -> Self;
}

impl<const N: usize, const S: usize, F: Fn(f64) -> [f64; N]> FromState<&State<N, S, F>>
    for (f64, [f64; N])
{
    fn from_state(state: &State<N, S, F>) -> Self {
        (state.t, state.x)
    }
}

impl<const N: usize, const S: usize, F: Fn(f64) -> [f64; N]> FromState<(&State<N, S, F>,)>
    for (f64, [f64; N])
{
    fn from_state(state: (&State<N, S, F>,)) -> Self {
        let state = state.0;
        (state.t, state.x)
    }
}

impl<const N: usize, const S: usize, F: Fn(f64) -> [f64; N]> FromState<&State<N, S, F>>
    for ([f64; N],)
{
    fn from_state(state: &State<N, S, F>) -> Self {
        (state.x,)
    }
}

impl<const N: usize, const S: usize, F: Fn(f64) -> [f64; N]> FromState<(&State<N, S, F>,)>
    for ([f64; N],)
{
    fn from_state(state: (&State<N, S, F>,)) -> Self {
        let state = state.0;
        (state.x,)
    }
}

// impl<'a, const N: usize, const S: usize, F: Fn(f64) -> [f64; N]> FromState<(&'a State<N, S, F>,)>
//     for (f64, [f64; N], [Box<dyn Fn(f64) -> f64 + 'a>; N])
// {
//     fn from_state(state: (&'a State<N, S, F>,)) -> Self {
//         let state = state.0;
//         (
//             state.t,
//             state.x,
//             std::array::from_fn(|i| {
//                 let f: Box<dyn Fn(f64) -> f64 + 'a> = Box::new(move |t: f64| state.eval_i(t, i));
//                 f
//             }),
//         )
//     }
// }
