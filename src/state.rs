use crate::rk::*;

use std::collections::VecDeque;

pub trait State<const N: usize> {
    const N: usize = N;
    fn t(&self) -> &f64;
    fn x(&self) -> &[f64; N];

    fn push_current(&mut self);
    fn make_step<Args, RHS>(&mut self, rhs: &RHS)
    where
        Args: std::marker::Tuple,
        RHS: Fn<Args, Output = [f64; N]>,
        for<'a> &'a Self: StateInto<Args>;
    fn make_zero_step(&mut self);

    fn eval(&self, t: f64) -> [f64; N];
    fn eval_derivative(&self, t: f64) -> [f64; N];
    fn eval_nth_derivative<const ORDER: usize>(&self, t: f64) -> [f64; N];
}

pub trait StateInto<T> {
    fn state_into(self) -> T;
}

// impl<'a, T> StateInto<&'a T> for &'a T {
//     fn state_into(self) -> Self {
//         self
//     }
// }
//
// impl<'a, T> StateInto<(&'a T,)> for &'a T {
//     fn state_into(self) -> (Self,) {
//         (self,)
//     }
// }
//
// impl<'a, T, const N: usize> StateInto<(&'a f64, &'a [f64; N])> for &'a T
// where
//     T: State<N>,
// {
//     fn state_into(self) -> (&'a f64, &'a [f64; N]) {
//         (self.t(), self.x())
//     }
// }
//
// impl<'a, T, const N: usize> StateInto<(&'a f64, &'a [f64; N])> for &'a mut T
// where
//     T: State<N>,
// {
//     fn state_into(self) -> (&'a f64, &'a [f64; N]) {
//         (self.t(), self.x())
//     }
// }

pub struct RKState<const N: usize, const S: usize, F: Fn(f64) -> [f64; N]> {
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

impl<'a, const N: usize, const S: usize, F: Fn(f64) -> [f64; N]> StateInto<(f64, [f64; N])>
    for &RKState<N, S, F>
{
    fn state_into(self) -> (f64, [f64; N]) {
        (self.t, self.x)
    }
}


impl<'a, const N: usize, const S: usize, F: Fn(f64) -> [f64; N]> StateInto<(f64, [f64; N])>
    for (&RKState<N, S, F>, )
{
    fn state_into(self) -> (f64, [f64; N]) {
        (self.0.t, self.0.x)
    }
}


impl<'a, const N: usize, const S: usize, F: Fn(f64) -> [f64; N]> StateInto<([f64; N], )>
    for (&RKState<N, S, F>, )
{
    fn state_into(self) -> ([f64; N], ) {
        (self.0.x, )
    }
}

// impl<const N: usize, const S: usize, F: Fn(f64) -> [f64; N]> std::fmt::Debug for RKState<N, RK, F>
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "State {{t: {:?}, x: {:?}, t_prev: {:?}, x_prev: {:?}}}", self.t, self.x, self.t_prev, self.x_prev)
//     }
// }

impl<const N: usize, const S: usize, F: Fn(f64) -> [f64; N]> RKState<N, S, F> {
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

impl<const N: usize, const S: usize, F: Fn(f64) -> [f64; N]> State<N> for RKState<N, S, F> {
    fn t(&self) -> &f64 {
        &self.t
    }
    fn x(&self) -> &[f64; N] {
        &self.x
    }

    fn push_current(&mut self) {
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

    fn make_step<Args, RHS>(&mut self, rhs: &RHS)
    where
        Args: std::marker::Tuple,
        RHS: Fn<Args, Output = [f64; N]>,
        for<'a> &'a Self: StateInto<Args>,
    {
        self.t_prev = self.t;
        self.x_prev = self.x;

        for i in 0..S {
            self.t = self.t_prev + self.rk.c[i] * self.t_step;

            self.x = std::array::from_fn(|k| {
                self.x_prev[k]
                    + self.t_step * (0..i).fold(0., |acc, j| acc + self.rk.a[i][j] * self.k[j][k])
            });
            self.k[i] = rhs.call(self.state_into());
        }

        self.x = std::array::from_fn(|k| {
            self.x_prev[k]
                + self.t_step * (0..S).fold(0., |acc, j| acc + self.rk.b[j] * self.k[j][k])
        });
        self.t = self.t_prev + self.t_step;
    }

    fn make_zero_step(&mut self) {
        self.t_prev = self.t;
        self.x_prev = self.x;
        self.k = [[0.; N]; S];
        self.push_current();
    }

    fn eval(&self, t: f64) -> [f64; N] {
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

    fn eval_derivative(&self, _t: f64) -> [f64; N] {
        todo!()
    }

    fn eval_nth_derivative<const ORDER: usize>(&self, _t: f64) -> [f64; N] {
        todo!()
    }
}
