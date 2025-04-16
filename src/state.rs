use crate::rk_table::*;

use std::collections::VecDeque;

pub struct State<const N: usize, RK, const S: usize>
where
    RK: RungeKuttaTable<S>,
    [(); RK::S]:,
{
    t: f64,
    t_init: f64,
    t_prev: f64,
    t_step: f64,
    t_span: f64,
    t_seq: VecDeque<f64>,

    x: [f64; N],
    x_init: fn(f64) -> [f64; N],
    x_prev: [f64; N],
    x_err: [f64; N],
    x_seq: VecDeque<[f64; N]>,

    k: [[f64; N]; RK::S],
    k_seq: VecDeque<[[f64; N]; RK::S]>,
}

impl<const N: usize, const S: usize, RK> State<N, RK, S>
where
    RK: RungeKuttaTable<S>,
    [(); RK::S]:,
{
    fn new(t_init: f64, x_init: fn(f64) -> [f64; N]) -> Self {
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

    fn make_step<F>(&mut self, rhs: &F)
    where
        F: Fn(&Self) -> [f64; N],
    {
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
            let t_prev = self.t_seq[i-1];
            let t_next = self.t_seq[i-1];
            let t_step = t_prev - t_next;
            let theta = (t - t_prev) / t_step;

            return std::array::from_fn(|i| {
                x_prev[i] + t_step * (0..RK::S).fold(0., |acc, j| acc + RK::BI[j](theta) * k[j][i])
            });
        }
    }
}
