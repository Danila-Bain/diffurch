use nalgebra::RealField;

use crate::{
    initial_condition::InitialCondition,
    state::{StateHistory, state::State},
    traits::RealVectorSpace,
};
use impl_tools::autoimpl;

#[autoimpl(Clone where T: Clone)]
#[autoimpl(Copy where T: Copy)]
#[autoimpl(Debug where T: std::fmt::Debug, P: std::fmt::Debug)]
pub struct StateRef<
    's,
    T: RealField + Copy,
    P: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, P>,
> {
    /// Time of a state
    pub t: T,
    /// Position of a state
    pub p: &'s P,

    /// Derivative of a state
    pub d: &'s P,

    /// Time of a state at the begining of the step
    pub t_prev: T,

    pub p_prev: &'s P,

    history: &'s StateHistory<T, P, S, I, IC>,
}

impl<
    's,
    T: RealField + Copy,
    P: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, P>,
> StateRef<'s, T, P, S, I, IC>
{
    pub fn p(&self, t: T) -> P {
        self.history.eval::<0>(t)
    }
    pub fn d(&self, t: T) -> P {
        self.history.eval::<1>(t)
    }
}

#[autoimpl(Debug where T: std::fmt::Debug, P: std::fmt::Debug)]
pub struct StateRefMut<
    's,
    T: RealField + Copy,
    P: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, P>,
> {
    /// Reference to time of a state
    pub t: &'s mut T,
    /// Reference to position of a state
    pub p: &'s mut P,

    pub d: &'s P,

    pub t_prev: T,

    pub p_prev: &'s P,

    history: &'s mut StateHistory<T, P, S, I, IC>,
}

impl<
    's,
    T: RealField + Copy,
    P: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, P>,
> StateRefMut<'s, T, P, S, I, IC>
{
    pub fn p(&self, t: T) -> P {
        self.history.eval::<0>(t)
    }
    pub fn d(&self, t: T) -> P {
        self.history.eval::<1>(t)
    }
    pub fn stop_integration(&mut self)
    where
        T: num_traits::Float,
    {
        *self.t = T::infinity();
    }
}

#[allow(unused)]
pub struct StateFn<T, P, Output, F, const MUT: bool = false> {
    f: F,
    _phantom_f: std::marker::PhantomData<fn(&T, &P) -> Output>,
}

impl<T, P, Output, F> StateFn<T, P, Output, F, false> {
    pub fn new<const S: usize, const I: usize, IC>(f: F) -> Self
    where
        F: FnMut(&StateRef<T, P, S, I, IC>) -> Output,
    {
        Self {
            f,
            _phantom_f: std::marker::PhantomData,
        }
    }
}

impl<T, P, Output, F> StateFn<T, P, Output, F, true> {
    pub fn new_mut<const S: usize, const I: usize, IC>(f: F) -> Self
    where
        F: FnMut(&mut StateRefMut<T, P, S, I, IC>) -> Output,
    {
        Self {
            f,
            _phantom_f: std::marker::PhantomData,
        }
    }
}

// abstract F parameter away
pub trait EvalState<
    T: RealField + Copy,
    P: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, P>,
    Output,
>
{
    fn eval_curr(&mut self, state: &State<T, P, S, I, IC>) -> Output;
    fn eval_prev(&mut self, state: &State<T, P, S, I, IC>) -> Output;
    fn eval_at(&mut self, state: &State<T, P, S, I, IC>, t: T) -> Output;
}

impl<
    T: RealField + Copy,
    P: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, P>,
> EvalState<T, P, S, I, IC, ()> for ()
{
    fn eval_curr(&mut self, _: &State<T, P, S, I, IC>) {}

    fn eval_prev(&mut self, _: &State<T, P, S, I, IC>) {}

    fn eval_at(&mut self, _: &State<T, P, S, I, IC>, _: T) {}
}

impl<
    T: RealField + Copy,
    P: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, P>,
    Output,
    F: FnMut(&StateRef<T, P, S, I, IC>) -> Output,
> EvalState<T, P, S, I, IC, Output> for StateFn<T, P, Output, F, false>
{
    fn eval_curr(&mut self, state: &State<T, P, S, I, IC>) -> Output {
        (self.f)(&StateRef {
            t: state.t_curr,
            t_prev: state.t_prev,
            p: &state.p_curr,
            p_prev: &state.p_prev,
            d: &state.d_curr,
            history: &state.history,
        })
    }
    fn eval_prev(&mut self, state: &State<T, P, S, I, IC>) -> Output {
        (self.f)(&StateRef {
            t: state.t_prev,
            t_prev: state.t_prev,
            p: &state.p_prev,
            p_prev: &state.p_prev,
            d: &state.d_prev,
            history: &state.history,
        })
    }
    fn eval_at(&mut self, state: &State<T, P, S, I, IC>, t: T) -> Output {
        let y = &state.eval::<0>(t);
        let dy = &state.eval::<1>(t);
        (self.f)(&StateRef {
            t,
            t_prev: t,
            p: y,
            p_prev: y,
            d: dy,
            history: &state.history,
        })
    }
}
pub trait EvalMutState<
    T: RealField + Copy,
    P: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, P>,
    Output,
>
{
    fn eval_mut(&mut self, state: &mut State<T, P, S, I, IC>) -> Output;
}

impl<
    T: RealField + Copy,
    P: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, P>,
> EvalMutState<T, P, S, I, IC, ()> for ()
{
    fn eval_mut(&mut self, _: &mut State<T, P, S, I, IC>) {}
}

impl<
    T: RealField + Copy,
    P: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, P>,
    Output,
    F: FnMut(&mut StateRefMut<T, P, S, I, IC>) -> Output,
> EvalMutState<T, P, S, I, IC, Output> for StateFn<T, P, Output, F, true>
{
    fn eval_mut(&mut self, state: &mut State<T, P, S, I, IC>) -> Output {
        (self.f)(&mut StateRefMut {
            t: &mut state.t_curr,
            t_prev: state.t_prev,
            p: &mut state.p_curr,
            p_prev: &state.p_prev,
            d: &state.d_curr,
            history: &mut state.history,
        })
    }
}

impl<
    T: RealField + Copy,
    P: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, P>,
    Output,
    F: FnMut(&StateRef<T, P, S, I, IC>) -> Output,
> EvalMutState<T, P, S, I, IC, Output> for StateFn<T, P, Output, F, false>
{
    fn eval_mut<'a>(&mut self, state: &mut State<T, P, S, I, IC>) -> Output {
        self.eval_curr(state)
    }
}

hlist2_trait_macro::TraitHList! {
    pub EvalStateFnHList for
        trait EvalState<
            T: RealField + Copy,
            P: RealVectorSpace<T>,
            const S: usize,
            const I: usize,
            IC: InitialCondition<T, P>,
            Output,
        > {
        fn eval_curr(
            &mut self,
            state: &State<T, P, S, I, IC>,
        ) -> Output;
    }
}

hlist2_trait_macro::TraitHList! {
    pub EvalMutStateFnHList for
        trait EvalMutState<
            T: RealField + Copy,
            P: RealVectorSpace<T>,
            const S: usize,
            const I: usize,
            IC: InitialCondition<T, P>,
            Output,
        > {
        fn eval_mut(
            &mut self,
            state: &mut State<T, P, S, I, IC>,
        ) -> Output;
    }
}
