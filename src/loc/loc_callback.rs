use nalgebra::RealField;

use crate::{
    initial_condition::InitialCondition,
    loc::locate::Locate,
    state::{EvalMutStateFn, EvalStateFn, State},
    traits::RealVectorSpace,
};

pub struct LocCallback<L, C>(L, C);

impl<L, C> From<(L, C)> for LocCallback<L, C> {
    fn from((l, c): (L, C)) -> Self {
        LocCallback(l, c)
    }
}

impl<T: RealField + Copy, Y: RealVectorSpace<T>, L: Locate<T, Y>, Other> Locate<T, Y> for LocCallback<L, Other> {
    fn locate<const S: usize, const S2: usize, IC: InitialCondition<T, Y>>(
        &mut self,
        state: &State<T, Y, S, S2, IC>,
    ) -> Option<T> {
        self.0.locate(state)
    }
}

impl<T: RealField + Copy, Y: RealVectorSpace<T>, Output, C: EvalStateFn<T, Y, Output>, Other> EvalStateFn<T, Y, Output>
    for LocCallback<Other, C>
{
    fn eval_curr<'s, const S: usize, const S2: usize, IC: InitialCondition<T, Y>>(
        &mut self,
        state: &'s State<T, Y, S, S2, IC>,
    ) -> Output {
        self.1.eval_curr(state)
    }

    fn eval_prev<'s, const S: usize, const S2: usize, IC: InitialCondition<T, Y>>(
        &mut self,
        state: &'s State<T, Y, S, S2, IC>,
    ) -> Output {
        self.1.eval_prev(state)
    }

    fn eval_at<'s, const S: usize, const S2: usize, IC: InitialCondition<T, Y>>(
        &mut self,
        state: &'s State<T, Y, S, S2, IC>,
        t: T,
    ) -> Output {
        self.1.eval_at(state, t)
    }
}

impl<T: RealField + Copy, Y: RealVectorSpace<T>, Output, C: EvalMutStateFn<T, Y, Output>, Other>
    EvalMutStateFn<T, Y, Output> for LocCallback<Other, C>
{
    fn eval_mut<'s, const S: usize, const S2: usize, IC: InitialCondition<T, Y>>(
        &mut self,
        state: &'s mut State<T, Y, S, S2, IC>,
    ) -> Output {
        self.1.eval_mut(state)
    }
}
