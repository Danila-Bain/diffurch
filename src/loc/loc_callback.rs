use nalgebra::RealField;

use crate::{
    initial_condition::InitialCondition,
    loc::locate::Locate,
    state::{EvalMutStateFn, EvalStateFn, State},
    traits::RealVectorSpace,
};

pub struct LocCallback<L, C>(pub L, pub C);

impl<L, C> From<(L, C)> for LocCallback<L, C> {
    fn from((l, c): (L, C)) -> Self {
        LocCallback(l, c)
    }
}

impl<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
    L: Locate<T, Y, S, I, IC>,
    Other,
> Locate<T, Y, S, I, IC> for LocCallback<L, Other>
{
    fn locate(&mut self, state: &State<T, Y, S, I, IC>) -> Option<T> {
        self.0.locate(state)
    }
}

impl<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
    Output,
    C: EvalStateFn<T, Y, S, I, IC, Output>,
    Other,
> EvalStateFn<T, Y, S, I, IC, Output> for LocCallback<Other, C>
{
    fn eval_curr(&mut self, state: &State<T, Y, S, I, IC>) -> Output {
        self.1.eval_curr(state)
    }

    fn eval_prev(&mut self, state: &State<T, Y, S, I, IC>) -> Output {
        self.1.eval_prev(state)
    }

    fn eval_at(&mut self, state: &State<T, Y, S, I, IC>, t: T) -> Output {
        self.1.eval_at(state, t)
    }
}

impl<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
    Output,
    C: EvalMutStateFn<T, Y, S, I, IC, Output>,
    Other,
> EvalMutStateFn<T, Y, S, I, IC, Output> for LocCallback<Other, C>
{
    fn eval_mut(&mut self, state: &mut State<T, Y, S, I, IC>) -> Output {
        self.1.eval_mut(state)
    }
}
