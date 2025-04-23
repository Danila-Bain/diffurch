use crate::rk_table::*;

use std::collections::VecDeque;

pub trait State<const N: usize> {
    fn t(&self) -> f64;
    fn x(&self) -> [f64; N];

    fn push_current(&mut self);
    fn make_step(&mut self, rhs: &impl Fn(&Self) -> [f64; N]);
    fn make_zero_step(&mut self);

    fn eval(&self, t: f64) -> [f64; N];
    fn eval_derivative(&self, t: f64) -> [f64; N];
    fn eval_nth_derivative<const ORDER: usize>(&self, t: f64) -> [f64; N];
}

pub struct RKState<const N: usize, RK, F: Fn(f64) -> [f64; N]>
where
    RK: RungeKuttaTable,
    [(); RK::S]:,
{
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

    k: [[f64; N]; RK::S],
    k_seq: VecDeque<[[f64; N]; RK::S]>,
}

impl<const N: usize, RK, F: Fn(f64) -> [f64; N]> std::fmt::Debug for RKState<N, RK, F>
where
    RK: RungeKuttaTable,
    [(); RK::S]:,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "State {{t: {:?}, x: {:?}, t_prev: {:?}, x_prev: {:?}}}", self.t, self.x, self.t_prev, self.x_prev)
    }
}

impl<const N: usize, RK, F: Fn(f64) -> [f64; N]> RKState<N, RK, F>
where
    RK: RungeKuttaTable,
    [(); RK::S]:,
{
    pub fn new(t_init: f64, x_init: F) -> Self {
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

            k: [[0.; N]; RK::S],
            k_seq: VecDeque::new(),
        }
    }
}

impl<const N: usize, RK, F: Fn(f64) -> [f64; N]> State<N> for RKState<N, RK, F>
where
    RK: RungeKuttaTable,
    [(); RK::S]:,
{
    fn t(&self) -> f64 {
        self.t
    }
    fn x(&self) -> [f64; N] {
        self.x
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
    

    fn make_step(&mut self, rhs: &impl Fn(&Self) -> [f64; N]) {
        self.t_prev = self.t;
        self.x_prev = self.x;

        for i in 0..RK::S {
            self.t = self.t_prev + RK::C[i] * self.t_step;

            self.x = std::array::from_fn(|k| {
                self.x_prev[k]
                    + self.t_step * (0..i).fold(0., |acc, j| acc + RK::A[i][j] * self.k[j][k])
            });
            self.k[i] = rhs(&self);
        }

        self.x = std::array::from_fn(|k| {
            self.x_prev[k]
                + self.t_step * (0..RK::S).fold(0., |acc, j| acc + RK::B[j] * self.k[j][k])
        });
        self.t = self.t_prev + self.t_step;
    }

    fn make_zero_step(&mut self) {
        self.t_prev = self.t;
        self.x_prev = self.x;
        self.k = [[0.; N]; RK::S];
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
                x_prev[i] + t_step * (0..RK::S).fold(0., |acc, j| acc + RK::BI[j](theta) * k[j][i])
            });
        }
    }

    fn eval_derivative(&self, t: f64) -> [f64; N] {
        todo!()
    }

    fn eval_nth_derivative<const ORDER: usize>(&self, t: f64) -> [f64; N] {
        todo!()
    }
}
