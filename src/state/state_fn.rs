use nalgebra::RealField;

use crate::{initial_condition::InitialCondition, state::state::State, traits::RealVectorSpace};

pub struct StateRef<'s, T, Y> {
    /// Time of a state
    pub t: T,
    /// Position of a state
    pub y: &'s Y,

    /// Derivative of a state
    pub dy: &'s Y,

    pub h: &'s dyn Fn(T) -> Y,
}

pub struct StateRefMut<'s, T, Y> {
    /// Reference to time of a state
    pub t: &'s mut T,
    /// Reference to position of a state
    pub y: &'s mut Y,

    pub dy: &'s Y,

    pub h: &'s dyn Fn(T) -> Y,
}

#[allow(unused)]
pub struct StateFn<T, Y, Output, F, const MUT: bool = false> {
    f: F,
    _phantom_f: std::marker::PhantomData<fn(&StateRef<T, Y>) -> Output>,
}

impl<T, Y, Output, F: FnMut(&StateRef<T, Y>) -> Output> StateFn<T, Y, Output, F, false> {
    pub fn new(f: F) -> Self {
        Self {
            f,
            _phantom_f: std::marker::PhantomData,
        }
    }
}

impl<T, Y, Output, F: FnMut(&mut StateRefMut<T, Y>) -> Output> StateFn<T, Y, Output, F, true> {
    pub fn new_mut(f: F) -> Self {
        Self {
            f,
            _phantom_f: std::marker::PhantomData,
        }
    }
}

// abstract F parameter away
pub trait EvalStateFn<T: RealField + Copy, Y: RealVectorSpace<T>, Output> {
    fn eval_curr<'s, const S: usize, const I: usize, IC: InitialCondition<T, Y>>(
        &mut self,
        state: &'s State<T, Y, S, I, IC>,
    ) -> Output;

    fn eval_prev<'s, const S: usize, const I: usize, IC: InitialCondition<T, Y>>(
        &mut self,
        state: &'s State<T, Y, S, I, IC>,
    ) -> Output;

    fn eval_at<'s, const S: usize, const I: usize, IC: InitialCondition<T, Y>>(
        &mut self,
        state: &'s State<T, Y, S, I, IC>,
        t: T,
    ) -> Output;
}

impl<T: RealField + Copy, Y: RealVectorSpace<T>, Output, F: FnMut(&StateRef<T, Y>) -> Output>
    EvalStateFn<T, Y, Output> for StateFn<T, Y, Output, F, false>
{
    fn eval_curr<'s, const S: usize, const I: usize, IC: InitialCondition<T, Y>>(
        &mut self,
        state: &'s State<T, Y, S, I, IC>,
    ) -> Output {
        (self.f)(&StateRef {
            t: state.t_curr,
            y: &state.y_curr,
            dy: &state.dy_curr,
            h: &|t: T| state.eval::<0>(t),
        })
    }
    fn eval_prev<'s, const S: usize, const I: usize, IC: InitialCondition<T, Y>>(
        &mut self,
        state: &'s State<T, Y, S, I, IC>,
    ) -> Output {
        (self.f)(&StateRef {
            t: state.t_prev,
            y: &state.y_prev,
            dy: &state.dy_prev,
            h: &|t: T| state.eval::<0>(t),
        })
    }
    fn eval_at<'s, const S: usize, const I: usize, IC: InitialCondition<T, Y>>(
        &mut self,
        state: &'s State<T, Y, S, I, IC>,
        t: T,
    ) -> Output {
        (self.f)(&StateRef {
            t: t,
            y: &state.eval::<0>(t),
            dy: &state.eval::<1>(t),
            h: &|t: T| state.eval::<0>(t),
        })
    }
}
pub trait EvalMutStateFn<T: RealField + Copy, Y: RealVectorSpace<T>, Output> {
    fn eval_mut<'s, const S: usize, const I: usize, IC: InitialCondition<T, Y>>(
        &mut self,
        state: &'s mut State<T, Y, S, I, IC>,
    ) -> Output;
}

impl<T: RealField + Copy, Y: RealVectorSpace<T>, Output, F: FnMut(&mut StateRefMut<T, Y>) -> Output>
    EvalMutStateFn<T, Y, Output> for StateFn<T, Y, Output, F, true>
{
    fn eval_mut<'s, const S: usize, const I: usize, IC: InitialCondition<T, Y>>(
        &mut self,
        state: &'s mut State<T, Y, S, I, IC>,
    ) -> Output {
        (self.f)(&mut StateRefMut {
            t: &mut state.t_curr,
            y: &mut state.y_curr,
            dy: &state.dy_curr,
            h: &|t: T| state.history.eval::<0>(t),
        })
    }
}

impl<T: RealField + Copy, Y: RealVectorSpace<T>, Output, F: FnMut(&StateRef<T, Y>) -> Output>
    EvalMutStateFn<T, Y, Output> for StateFn<T, Y, Output, F, false>
{
    fn eval_mut<'s, const S: usize, const I: usize, IC: InitialCondition<T, Y>>(
        &mut self,
        state: &'s mut State<T, Y, S, I, IC>,
    ) -> Output {
        self.eval_curr(state)
    }
}

hlist2_trait_macro::TraitHList! {
    pub EvalStateFnHList for trait EvalStateFn<T: RealField + Copy, Y: RealVectorSpace<T>, Output> {
        fn eval_curr<'s, const S: usize, const I: usize, IC: InitialCondition<T, Y>>(
            &mut self,
            state: &'s State<T, Y, S, I, IC>,
        ) -> Output where T: 's, Y: 's, IC: 's;
    }
}

hlist2_trait_macro::TraitHList! {
    pub EvalMutStateFnHList for trait EvalMutStateFn<T: RealField + Copy, Y: RealVectorSpace<T>, Output> {
        fn eval_mut<'s, const S: usize, const I: usize, IC: InitialCondition<T, Y>>(
            &mut self,
            state: &'s mut State<T, Y, S, I, IC>,
        ) -> Output where T: 's, Y: 's, IC: 's;
    }
}

pub trait IntoStateFn<T: RealField + Copy, Y: RealVectorSpace<T>, Output>: Sized {
    type Output: EvalStateFn<T, Y, Output>;
    fn into(self) -> Self::Output;
}
pub trait IntoMutStateFn<T: RealField + Copy, Y: RealVectorSpace<T>, Output>: Sized {
    type Output: EvalMutStateFn<T, Y, Output>;
    fn into(self) -> Self::Output;
}

// pub trait IntoStateFn<const N: usize, T, Output>: Sized {
//     fn into(self) -> StateFn<T, Y, Output, Self, false>;
// }
// pub trait IntoMutStateFn<const N: usize, T, Output>: Sized {
//     fn into(self) -> StateFn<T, Y, Output, Self, true>;
// }

impl<T: RealField + Copy, Y: RealVectorSpace<T>, Output, F: FnMut(&StateRef<T, Y>) -> Output>
    IntoStateFn<T, Y, Output> for F
{
    type Output = StateFn<T, Y, Output, Self, false>;
    fn into(self) -> Self::Output {
        StateFn::new(self)
    }
}
impl<T: RealField + Copy, Y: RealVectorSpace<T>, Output, F: FnMut(&mut StateRefMut<T, Y>) -> Output>
    IntoMutStateFn<T, Y, Output> for F
{
    type Output = StateFn<T, Y, Output, Self, true>;
    fn into(self) -> Self::Output {
        StateFn::new_mut(self)
    }
}

impl<T: RealField + Copy, Y: RealVectorSpace<T>, Output, F: FnMut(&StateRef<T, Y>) -> Output>
    IntoStateFn<T, Y, Output> for StateFn<T, Y, Output, F, false>
{
    type Output = StateFn<T, Y, Output, F, false>;
    fn into(self) -> Self::Output {
        self
    }
}
impl<T: RealField + Copy, Y: RealVectorSpace<T>, Output, F: FnMut(&mut StateRefMut<T, Y>) -> Output>
    IntoMutStateFn<T, Y, Output> for StateFn<T, Y, Output, F, true>
{
    type Output = StateFn<T, Y, Output, F, true>;
    fn into(self) -> Self::Output {
        self
    }
}
