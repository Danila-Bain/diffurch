use crate::rk_table::rk4::RK4;
use crate::rk_table::rk4::RungeKuttaTable;

use std::collections::VecDeque;

pub struct State<const N: usize, const S: usize, RK>
where
    RK: RungeKuttaTable<S>,
{
    t: f64,
    t_init: f64,
    t_prev: f64,
    t_step: f64,
    t_span: f64,
    t_seq: VecDeque<f64>,

    x: [f64; N],
    x_init: [f64; N],
    x_prev: [f64; N],
    x_err: [f64; N],
    x_seq: VecDeque<[f64; N]>,

    k: [[f64; N]; S],
    k_seq: VecDeque<[[f64; N]; S]>,
}

impl<const N: usize, const S: usize, RK> State<N, S, RK> where RK: RungeKuttaTable<S> {
    fn new(t_init: f64, x_init: [f64; N]) -> Self {
        Self {
            t_init,
            t: t_init,
            t_prev: t_init,
            t_step: 0.,
            t_span: 0.,
            t_seq: VecDeque::from([t_init]),

            x_init,
            x: x_init,
            x_prev: x_init,
            x_err: [0.; N],
            x_seq: VecDeque::from([x_init]),

            k: [[0.; N]; S],
            k_seq: VecDeque::new(),
        }
    }

    fn push_current(&mut self) {
        self.t_seq.push_back(self.t);
        self.x_seq.push_back(self.x);
        self.k_seq.push_back(self.k);
        while self.t - self.t_span > *self.t_seq.front().expect("Last element won't pop for non-negative t_span") {
            self.t_seq.pop_front();
            self.x_seq.pop_front();
            self.k_seq.pop_front();
        }         
    }

    fn make_step<F>(&mut self, rhs: &F)
    where
        F: Fn(&Self) -> [f64; N],
    {
        for i in 0..S {
            self.t = self.t_prev + RK::C[i] * self.t_step;

            self.x = std::array::from_fn(|k| {
                self.x_prev[k]
                    + self.t_step * (0..i).fold(0., |acc, j| acc + RK::A[i][j] * self.k[j][k])
            });
            self.k[i] = rhs(&self);
        }

        self.x = std::array::from_fn(|k| {
            self.x_prev[k] + self.t_step * (0..S).fold(0., |acc, j| acc + RK::B[j] * self.k[j][k])
        });
    }

    fn make_zero_step(&mut self) {
        self.t_prev = self.t;
        self.x_prev = self.x;
        self.k = [[0.; N]; S];
        self.push_current();
    }

    fn eval(&self, t: f64) -> [f64; N] {
        if t < self.t_init {
            return self.x_init;
        } else {
            unimplemented!("Eval function unimplemented. Polynomial is required");
        }
    }
}
