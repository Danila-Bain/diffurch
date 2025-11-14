use crate::initial_condition::InitialCondition;
use std::collections::VecDeque;

pub struct State<'rk, const N: usize, const S: usize, const S2: usize, T, IC> {
    pub t_init: T,
    pub t_curr: T,
    pub t_prev: T,
    pub t_span: T,

    pub x_init: IC,
    pub x_curr: [T; N],
    pub x_prev: [T; N],

    pub rk: &'rk crate::rk::ExplicitRungeKuttaTable<S, S2, T>,
    pub k_curr: [[T; N]; S],

    pub t_deque: VecDeque<T>,
    pub x_deque: VecDeque<[T; N]>,
    pub k_deque: VecDeque<[[T; N]; S]>,
    pub disco_deque: VecDeque<T>,
}

impl<
    'rk,
    T: num::Float + std::fmt::Debug,
    const N: usize,
    const S: usize,
    const S2: usize,
    IC: InitialCondition<N, T>,
> State<'rk, N, S, S2, T, IC>
{
    pub fn new(
        t_init: T,
        x_init: IC,
        rk: &'rk crate::rk::ExplicitRungeKuttaTable<S, S2, T>,
    ) -> Self {
        let x = x_init.eval::<0>(t_init);
        Self {
            t_init,
            t_curr: t_init,
            t_prev: t_init,
            t_span: T::zero(),
            x_init,
            x_curr: x,
            x_prev: x,
            rk,
            k_curr: [[T::zero(); N]; S],
            t_deque: VecDeque::new(),
            x_deque: VecDeque::new(),
            k_deque: VecDeque::new(),
            disco_deque: VecDeque::new(),
        }
    }

    pub fn eval(&self, t: T) -> [T; N] {
        if t <= self.t_init {
            self.x_init.eval::<0>(t)
        } else if self.t_prev <= t && t <= self.t_curr {
            let x_prev = self.x_prev;
            let k = self.k_curr;
            let t_prev = self.t_prev;
            let t_next = self.t_curr;
            let t_step = t_next - t_prev;
            if t_step == T::zero() {
                return x_prev;
            }
            let theta = (t - t_prev) / t_step;
            return std::array::from_fn(|i| {
                x_prev[i]
                    + t_step
                        * (0..S).fold(T::zero(), |acc, j| acc + (self.rk.bi[j].0)(theta) * k[j][i])
            });
        } else {
            let i = self.t_deque.partition_point(|t_i| t_i <= &t); // first i : t_seq[i] > t
            if i == 0 {
                panic!(
                    "Evaluation of state at {t:?} in deleted time range (before {:?}). Try setting .max_delay({:?}) or larger.",
                    self.t_deque.front(),
                    self.t_curr - t
                );
            } else if i == self.t_deque.len() {
                panic!(
                    "Evaluation of state in a not yet computed time range at {t:?} while state.t is {:?}.",
                    self.t_curr
                );
            }
            let x_prev = &self.x_deque[i - 1];
            let k = &self.k_deque[i - 1];
            let t_prev = self.t_deque[i - 1];
            let t_next = self.t_deque[i];
            let t_step = t_next - t_prev;
            if t_step == T::zero() {
                return *x_prev;
            }
            let theta = (t - t_prev) / t_step;

            return std::array::from_fn(|i| {
                x_prev[i]
                    + t_step
                        * (0..S).fold(T::zero(), |acc, j| acc + (self.rk.bi[j].0)(theta) * k[j][i])
            });
        }
    }

    pub fn make_step(
        &mut self,
        rhs: &impl EvalStateFn<N, T, [T; N]>,
        t_step: T,
    ) {
        self.t_prev = self.t_curr;
        self.x_prev = self.x_curr;

        let mut a_i = 0;
        for i in 0..S {
            self.t_curr = self.t_prev + self.rk.c[i] * t_step;

            self.x_curr = std::array::from_fn(|k| {
                self.x_prev[k]
                    + t_step
                        * (0..i).fold(T::zero(), |acc, j| {
                            acc + self.rk.a[a_i + j] * self.k_curr[j][k]
                        })
            });
            a_i += i;
            self.k_curr[i] = rhs.eval_curr(self);
        }

        self.x_curr = std::array::from_fn(|k| {
            self.x_prev[k]
                + t_step * (0..S).fold(T::zero(), |acc, j| acc + self.rk.b[j] * self.k_curr[j][k])
        });
        self.t_curr = self.t_prev + t_step;
    }

    pub fn commit_step(&mut self) {
        self.t_deque.push_back(self.t_curr);
        self.x_deque.push_back(self.x_curr);
        self.k_deque.push_back(self.k_curr);
        let t_tail = self.t_prev - self.t_span;
        while let Some(second_t) = self.t_deque.get(1)
            && *second_t < t_tail
        {
            self.t_deque.pop_front();
            self.x_deque.pop_front();
            self.k_deque.pop_front();
        }
        // while let Some((t, _order)) = self.disco_seq.front()
        //     && t < &t_tail
        // {
        //     self.disco_seq.pop_front();
        // }
    }

    pub fn make_zero_step(&mut self) {
        self.t_prev = self.t_curr;
        self.x_prev = self.x_curr;
        self.k_curr = [[T::zero(); N]; S];
    }

    pub fn undo_step(&mut self) {
        self.t_curr = self.t_prev;
        self.x_curr = self.x_prev;
    }
}

pub struct StateRef<'s, T, const N: usize> {
    /// Reference to time of a state
    pub t: &'s T,
    /// Reference to position of a state
    pub x: &'s [T; N],

    pub h: &'s dyn Fn(T) -> [T; N],
}

pub struct StateRefMut<'s, T, const N: usize> {
    /// Reference to time of a state
    pub t: &'s mut T,
    /// Reference to position of a state
    pub x: &'s mut [T; N],

    pub h: &'s dyn Fn(T) -> [T; N],
}

#[allow(unused)]
pub struct StateFn<const N: usize, T, Output, F> {
    f: F,
    _phantom_f: std::marker::PhantomData<fn(&StateRef<T, N>) -> Output>,
}

impl<T: num::Float + std::fmt::Debug, const N: usize, Output, F: Fn(&StateRef<T, N>) -> Output>
    StateFn<N, T, Output, F>
{
    pub fn new(f: F) -> Self {
        Self {
            f,
            _phantom_f: std::marker::PhantomData,
        }
    }
}

impl<
    T: num::Float + std::fmt::Debug,
    const N: usize,
    Output,
    F: FnMut(&mut StateRefMut<T, N>) -> Output,
> StateFn<N, T, Output, F>
{
    pub fn new_mut(f: F) -> Self {
        Self {
            f,
            _phantom_f: std::marker::PhantomData,
        }
    }
}

// abstract F parameter away
pub trait EvalStateFn<const N: usize, T, Output> {
    fn eval_curr<'s, const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &self,
        state: &'s State<N, S, S2, T, IC>,
    ) -> Output;

    fn eval_prev<'s, const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &self,
        state: &'s State<N, S, S2, T, IC>,
    ) -> Output;

    fn eval_at<'s, const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &self,
        state: &'s State<N, S, S2, T, IC>,
        t: T,
    ) -> Output;
}

impl<T: num::Float + std::fmt::Debug, const N: usize, Output, F: Fn(&StateRef<T, N>) -> Output>
    EvalStateFn<N, T, Output> for StateFn<N, T, Output, F>
{
    fn eval_curr<'s, const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &self,
        state: &'s State<N, S, S2, T, IC>,
    ) -> Output {
        (self.f)(&StateRef {
            t: &state.t_curr,
            x: &state.x_curr,
            h: &|t: T| state.eval(t),
        })
    }
    fn eval_prev<'s, const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &self,
        state: &'s State<N, S, S2, T, IC>,
    ) -> Output {
        (self.f)(&StateRef {
            t: &state.t_prev,
            x: &state.x_prev,
            h: &|t: T| state.eval(t),
        })
    }
    fn eval_at<'s, const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &self,
        state: &'s State<N, S, S2, T, IC>,
        t: T,
    ) -> Output {
        (self.f)(&StateRef {
            t: &t,
            x: &state.eval(t),
            h: &|t: T| state.eval(t),
        })
    }
}

pub trait EvalMutStateFn<T: num::Float + std::fmt::Debug, const N: usize, Output> {
    fn eval_mut<'s, const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &'s mut State<N, S, S2, T, IC>,
    ) -> Output;
}

impl<
    T: num::Float + std::fmt::Debug,
    const N: usize,
    Output,
    F: FnMut(&mut StateRefMut<T, N>) -> Output,
> EvalMutStateFn<T, N, Output> for StateFn<N, T, Output, F>
{
    fn eval_mut<'s, const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &'s mut State<N, S, S2, T, IC>,
    ) -> Output {
        (self.f)(&mut StateRefMut {
            t: &mut state.t_curr,
            x: &mut state.x_curr,
            h: &|t: T| [t; N],
        })
    }
}


trait_hlist::TraitHList! {
    pub EvalStateFnHList for trait EvalStateFn<const N: usize, T, Output> {
        fn eval_curr<'s, const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
            &self,
            state: &'s State<N, S, S2, T, IC>,
        ) -> Output where T: 's, IC: 's;
    }
}

// impl<T: num::Float + std::fmt::Debug,
//     const N: usize,
//     F: Fn(&StateRef<T, N>) -> (),
//     Filter,
// > StateFn<N, T, (), F, Filter> {
//     pub fn eval_filtered<'s, const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
//         &self,
//         state: &'s State<N, S, S2, T, IC>,
//     ) {
//         // if self.filter.all() {
//            self.eval_curr(&state)
//         // }
//     }
//
// }

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn basic_inference() {
    //     let sigma = 0.;
    //     let sum = StateFn::new(|state| {
    //         let (t, [x, y]) = (state.t, state.x);
    //         t + x + y + sigma
    //     });
    //     let s = State {
    //         t_curr: 0.,
    //         x_curr: [1., 2.],
    //         t_prev: 0.,
    //         x_prev: [-1., -2.],
    //     };
    //
    //     assert_eq!(sum.eval(&s), 0. + 1. + 2.);
    //     assert_eq!(sum.eval_prev(&s), 0. - 1. - 2.);
    // }

    #[test]
    fn lorenz() {
        let sigma = 10.;
        let rho = 28.;
        let beta = 8. / 3.;

        let rk = crate::rk::euler();

        let mut state = State::new(0., [0., 0., 0.], &rk);

        let lorenz_rhs = StateFn::new(|&StateRef { x: [x, y, z], .. }| {
            [sigma * (y - x), x * (rho - z) - y, x * y - beta * z]
        });
        lorenz_rhs.eval_curr(&state);

        let mut event = StateFn::new_mut(|state| {
            let ref mut t = *state.t;
            *t += 4.2;
        });
        event.eval_mut(&mut state);

        let mut event = StateFn::new_mut(
            |&mut StateRefMut {
                 ref mut t,
                 x: [x, _, _],
                 ..
             }| {
                let ref mut t = **t;
                *x = 15.;
                *t = 5.;
            },
        );
        event.eval_mut(&mut state);
        assert_eq!(state.t_curr, 5.);
        assert_eq!(state.x_curr[0], 15.);

        // panic!("{:?}, {:?}", state.t_curr, state.x_curr)
    }
}
