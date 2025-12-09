use nalgebra::RealField;

use crate::{
    initial_condition::InitialCondition,
    state::{StateHistory, state::State},
    traits::RealVectorSpace,
};
use impl_tools::autoimpl;

#[autoimpl(Clone where T: Clone)]
#[autoimpl(Copy where T: Copy)]
#[autoimpl(Debug where T: std::fmt::Debug, Y: std::fmt::Debug)]
pub struct StateRef<
    's,
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
> {
    /// Time of a state
    pub t: T,
    /// Position of a state
    pub y: &'s Y,

    /// Derivative of a state
    pub dy: &'s Y,

    history: &'s StateHistory<T, Y, S, I, IC>,
}

impl<
    's,
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
> StateRef<'s, T, Y, S, I, IC>
{
    pub fn y(&self, t: T) -> Y {
        self.history.eval::<0>(t)
    }
    pub fn dy(&self, t: T) -> Y {
        self.history.eval::<1>(t)
    }
}

#[autoimpl(Debug where T: std::fmt::Debug, Y: std::fmt::Debug)]
pub struct StateRefMut<
    's,
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
> {
    /// Reference to time of a state
    pub t: &'s mut T,
    /// Reference to position of a state
    pub y: &'s mut Y,

    pub dy: &'s Y,

    history: &'s mut StateHistory<T, Y, S, I, IC>,
}

impl<
    's,
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
> StateRefMut<'s, T, Y, S, I, IC>
{
    pub fn y(&self, t: T) -> Y {
        self.history.eval::<0>(t)
    }
    pub fn dy(&self, t: T) -> Y {
        self.history.eval::<1>(t)
    }
    pub fn stop_integration(&mut self) {
        *self.t = T::max_value().unwrap();
    }
}

#[allow(unused)]
pub struct StateFn<T, Y, Output, F, const MUT: bool = false> {
    f: F,
    _phantom_f: std::marker::PhantomData<fn(&T, &Y) -> Output>,
}

impl<T, Y, Output, F> StateFn<T, Y, Output, F, false> {
    pub fn new<const S: usize, const I: usize, IC>(f: F) -> Self
    where
        F: FnMut(&StateRef<T, Y, S, I, IC>) -> Output,
    {
        Self {
            f,
            _phantom_f: std::marker::PhantomData,
        }
    }
}

impl<T, Y, Output, F> StateFn<T, Y, Output, F, true> {
    pub fn new_mut<const S: usize, const I: usize, IC>(f: F) -> Self
    where
        F: FnMut(&mut StateRefMut<T, Y, S, I, IC>) -> Output,
    {
        Self {
            f,
            _phantom_f: std::marker::PhantomData,
        }
    }
}

// abstract F parameter away
pub trait EvalStateFn<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
    Output,
>
{
    fn eval_curr(&mut self, state: &State<T, Y, S, I, IC>) -> Output;
    fn eval_prev(&mut self, state: &State<T, Y, S, I, IC>) -> Output;
    fn eval_at(&mut self, state: &State<T, Y, S, I, IC>, t: T) -> Output;
}

impl<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
    Output,
    F: FnMut(&StateRef<T, Y, S, I, IC>) -> Output,
> EvalStateFn<T, Y, S, I, IC, Output> for StateFn<T, Y, Output, F, false>
{
    fn eval_curr(&mut self, state: &State<T, Y, S, I, IC>) -> Output {
        (self.f)(&StateRef {
            t: state.t_curr,
            y: &state.y_curr,
            dy: &state.dy_curr,
            history: &state.history,
        })
    }
    fn eval_prev(&mut self, state: &State<T, Y, S, I, IC>) -> Output {
        (self.f)(&StateRef {
            t: state.t_prev,
            y: &state.y_prev,
            dy: &state.dy_prev,
            history: &state.history,
        })
    }
    fn eval_at(&mut self, state: &State<T, Y, S, I, IC>, t: T) -> Output {
        (self.f)(&StateRef {
            t: t,
            y: &state.eval::<0>(t),
            dy: &state.eval::<1>(t),
            history: &state.history,
        })
    }
}
pub trait EvalMutStateFn<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
    Output,
>
{
    fn eval_mut(&mut self, state: &mut State<T, Y, S, I, IC>) -> Output;
}

impl<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
    Output,
    F: FnMut(&mut StateRefMut<T, Y, S, I, IC>) -> Output,
> EvalMutStateFn<T, Y, S, I, IC, Output> for StateFn<T, Y, Output, F, true>
{
    fn eval_mut(&mut self, state: &mut State<T, Y, S, I, IC>) -> Output {
        (self.f)(&mut StateRefMut {
            t: &mut state.t_curr,
            y: &mut state.y_curr,
            dy: &state.dy_curr,
            history: &mut state.history,
        })
    }
}

impl<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
    Output,
    F: FnMut(&StateRef<T, Y, S, I, IC>) -> Output,
> EvalMutStateFn<T, Y, S, I, IC, Output> for StateFn<T, Y, Output, F, false>
{
    fn eval_mut<'a>(&mut self, state: &mut State<T, Y, S, I, IC>) -> Output {
        self.eval_curr(state)
    }
}

hlist2_trait_macro::TraitHList! {
    pub EvalStateFnHList for
        trait EvalStateFn<
            T: RealField + Copy,
            Y: RealVectorSpace<T>,
            const S: usize,
            const I: usize,
            IC: InitialCondition<T, Y>,
            Output,
        > {
        fn eval_curr(
            &mut self,
            state: &State<T, Y, S, I, IC>,
        ) -> Output;
    }
}

hlist2_trait_macro::TraitHList! {
    pub EvalMutStateFnHList for
        trait EvalMutStateFn<
            T: RealField + Copy,
            Y: RealVectorSpace<T>,
            const S: usize,
            const I: usize,
            IC: InitialCondition<T, Y>,
            Output,
        > {
        fn eval_mut(
            &mut self,
            state: &mut State<T, Y, S, I, IC>,
        ) -> Output;
    }
}
