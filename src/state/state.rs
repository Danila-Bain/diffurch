use impl_tools::autoimpl;
use nalgebra::RealField;

use crate::{
    initial_condition::InitialCondition, state::state_fn::EvalStateFn, traits::RealVectorSpace,
};
use std::collections::VecDeque;

#[derive(Clone)]
#[autoimpl(Debug ignore self.y_init where T: std::fmt::Debug, Y: std::fmt::Debug)]
pub struct StateHistory<T: RealField, Y: RealVectorSpace<T>, const S: usize, const I: usize, IC> {
    pub t_span: T,

    pub t_init: T,
    pub y_init: IC,

    pub t_deque: VecDeque<T>,
    pub y_deque: VecDeque<Y>,
    pub k_deque: VecDeque<[Y; S]>,
    pub disco_deque: VecDeque<(T, usize)>,

    pub rk: crate::rk::ButcherTableu<T, S, I>,
}

#[autoimpl(Debug ignore self.history, self.rk, self.k_curr where T: std::fmt::Debug, Y: std::fmt::Debug)]
pub struct State<T: RealField + Copy, Y: RealVectorSpace<T>, const S: usize, const I: usize, IC> {
    pub history: StateHistory<T, Y, S, I, IC>,

    pub t_curr: T,
    pub t_prev: T,

    pub y_curr: Y,
    pub y_prev: Y,

    pub dy_curr: Y,
    pub dy_prev: Y,

    pub e_curr: Y,

    pub rk: crate::rk::ButcherTableu<T, S, I>,
    pub k_curr: [Y; S],
}

impl<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
> State<T, Y, S, I, IC>
{
    pub fn new(t_init: T, t_span: T, y_init: IC, disco_init: VecDeque<(T, usize)>, rk: crate::rk::ButcherTableu<T, S, I>) -> Self {
        let y = y_init.eval::<0>(t_init);
        Self {
            t_curr: t_init,
            t_prev: t_init,
            y_curr: y,
            y_prev: y,
            dy_curr: Y::zero(),
            dy_prev: Y::zero(),
            e_curr: Y::zero(),
            rk,
            k_curr: [Y::zero(); S],
            history: StateHistory {
                rk,
                t_init,
                t_span,
                y_init,
                t_deque: VecDeque::from([t_init]),
                y_deque: VecDeque::from([y]),
                k_deque: VecDeque::new(),
                disco_deque: disco_init,
            },
        }
    }

    pub fn eval<const D: usize>(&self, t: T) -> Y {
        if t >= self.t_prev && t <= self.t_curr {
            let t_step = self.t_curr - self.t_prev;
            let theta = (t - self.t_prev) / t_step;
            self.rk
                .dense_output::<D, Y>(&self.y_prev, t_step, theta, &self.k_curr)
        } else {
            self.history.eval::<D>(t)
        }
    }

    pub fn make_step(&mut self, rhs: &mut impl EvalStateFn<T, Y, S, I, IC, Y>, t_step: T) {
        if self.t_prev != self.t_curr {
            self.k_curr[0] = self.dy_curr;
        } else {
            self.k_curr[0] = rhs.eval_curr(self);
        }

        self.t_prev = self.t_curr;
        self.y_prev = self.y_curr;
        self.dy_prev = self.dy_curr;

        for i in 1..S {
            self.t_curr = self.t_prev + self.rk.c[i] * t_step;
            self.y_curr = self.y_prev
                + (0..i).fold(Y::zero(), |acc, j| acc + self.k_curr[j] * self.rk.a[i][j]) * t_step;
            self.k_curr[i] = rhs.eval_curr(self);
        }

        self.y_curr = self.y_prev
            + (0..S).fold(Y::zero(), |acc, j| acc + self.k_curr[j] * self.rk.b[j]) * t_step;
        self.t_curr = self.t_prev + t_step;
        self.dy_curr = rhs.eval_curr(self);

        self.e_curr = (0..S).fold(Y::zero(), |acc, j| acc + self.k_curr[j] * (self.rk.b2[j] - self.rk.b[j])) * t_step;
    }

    pub fn commit_step(&mut self) {
        self.history.t_deque.push_back(self.t_curr);
        self.history.y_deque.push_back(self.y_curr);
        self.history.k_deque.push_back(self.k_curr);
        let t_tail = self.t_prev - self.history.t_span;
        while let Some(second_t) = self.history.t_deque.get(1)
            && *second_t < t_tail
        {
            self.history.t_deque.pop_front();
            self.history.y_deque.pop_front();
            self.history.k_deque.pop_front();
        }
        while let Some((t, _order)) = self.history.disco_deque.front()
            && t < &t_tail
        {
            self.history.disco_deque.pop_front();
        }
    }

    pub fn make_zero_step(&mut self) {
        self.t_prev = self.t_curr;
        self.y_prev = self.y_curr;
        self.k_curr = [Y::zero(); S];
        // self.dy_curr = Y::zero();
        // self.dy_curr = self.dy_curr;
    }

    pub fn undo_step(&mut self) {
        self.t_curr = self.t_prev;
        self.y_curr = self.y_prev;
        self.dy_curr = self.dy_prev;
    }
}

impl<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
> StateHistory<T, Y, S, I, IC>
{
    pub fn eval<const D: usize>(&self, t: T) -> Y {
        if t <= self.t_init {
            self.y_init.eval::<D>(t)
        } else {
            let i = self.t_deque.partition_point(|t_i| t_i <= &t); // first i : t_seq[i] > t
            if i == 0 {
                panic!(
                    "Evaluation of state at {:?} in deleted time range (before {:?})",
                    maybe_debug::maybe_debug(&t),
                    self.t_deque.front().map(maybe_debug::maybe_debug),
                );
            } else if i == self.t_deque.len() {
                panic!(
                    "Evaluation of state in a not yet computed time range at {:?} while most recent time in history is {:?}.",
                    maybe_debug::maybe_debug(&t),
                    self.t_deque.back().map(maybe_debug::maybe_debug)
                );
            }
            let x_prev = &self.y_deque[i - 1];
            let k = &self.k_deque[i - 1];
            let t_prev = self.t_deque[i - 1];
            let t_next = self.t_deque[i];
            let t_step = t_next - t_prev;
            let theta = (t - t_prev) / t_step;
            self.rk.dense_output::<D, Y>(x_prev, t_step, theta, k)
        }
    }
}
