use num::Float;

use crate::{initial_condition::InitialCondition, loc::locate::Locate, state::{EvalMutStateFn, EvalStateFn, State}};

pub struct LocCallback<L, C>(L, C);

impl<L, C> From<(L, C)> for LocCallback<L, C> {
    fn from((l, c): (L, C)) -> Self {
        LocCallback(l, c)
    }
}

impl<const N: usize, T: Float, L: Locate<N, T>, Other> Locate<N, T> for LocCallback<L, Other> {
    fn locate<const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &State<N, S, S2, T, IC>,
    ) -> Option<T> {
        self.0.locate(state)
    }
}

impl<const N: usize, T: Float, Output, C: EvalStateFn<N, T, Output>, Other> EvalStateFn<N, T, Output> for LocCallback<Other, C> {
    fn eval_curr<'s, const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &'s State<N, S, S2, T, IC>,
    ) -> Output {
        self.1.eval_curr(state)
    }

    fn eval_prev<'s, const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &'s State<N, S, S2, T, IC>,
    ) -> Output {
        self.1.eval_prev(state)
    }

    fn eval_at<'s, const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &'s State<N, S, S2, T, IC>,
        t: T,
    ) -> Output {
        self.1.eval_at(state, t)
    }
}

impl<const N: usize, T: Float, Output, C: EvalMutStateFn<N, T, Output>, Other> EvalMutStateFn<N, T, Output> for LocCallback<Other, C> {
    fn eval_mut<'s, const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &'s mut State<N, S, S2, T, IC>,
    ) -> Output {
        self.1.eval_mut(state)
    }
}
