use super::Loc;
use crate::{
    initial_condition::InitialCondition,
    loc::detect::Detect,
    state::{EvalStateFn, State},
    traits::RealVectorSpace,
};
use nalgebra::RealField;

pub trait Locate<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
>
{
    fn locate(&mut self, state: &State<T, Y, S, I, IC>) -> Option<T>;
}

/// Use the previous step time as the location of event
pub struct StepBegin;
/// Use the current step time as the location of event
pub struct StepEnd;
/// Use the middle between previous and current step time as the location of event
pub struct StepMiddle;
/// Use the linear interpolation as an approximation for the location of event for float-valued
/// detection functions (not supported for `bool` detection functions)
pub struct Lerp;
/// Use bisection method to find the location of event for float-valued detection functions. See also: [BisectionBool].
pub struct Bisection;
/// Use bisection method to find the location of event for bool-valued detection functions. See also: [Bisection].
pub struct BisectionBool;
/// Use regula falsi method to find the location of event for float-valued detection functions. See also: [Bisection]. Current implementation is not as reliable as [Bisection].
pub struct RegulaFalsi;

macro_rules! impl_locate(
    ($locate:ident, $(Output = $fn_output:ty,)? |$self:ident, $state:ident| $body:expr) => {
        impl<
            T: RealField + Copy,
            Y: RealVectorSpace<T>,
            const S: usize,
            const I: usize,
            IC: InitialCondition<T, Y>,
            D, 
            F $(: EvalStateFn<T, Y, S, I, IC, $fn_output>)?
        >
            Locate<T, Y, S, I, IC> for Loc<T, Y, S, I, IC, F, D, $locate> where Self: Detect<T, Y, S, I, IC>, {
            fn locate(&mut $self, $state: &State<T, Y, S, I, IC>) -> Option<T> {
                $self.detect($state).then(|| $body)
            }
        }
    }
);

impl_locate!(StepBegin, |self, state| { state.t_prev });
impl_locate!(StepEnd, |self, state| { state.t_curr });
impl_locate!(StepMiddle, |self, state| {
    T::from_f64(0.5).unwrap() * (state.t_curr - state.t_prev)
});
impl_locate!(Lerp, Output = T, |self, state| {
    let curr = self.function.eval_curr(state);
    let prev = self.function.eval_prev(state);
    (curr * state.t_prev - prev * state.t_curr) / (curr - prev)
});
impl_locate!(BisectionBool, Output = bool, |self, state| {
    let mut l = state.t_prev;
    let mut r = state.t_curr;

    let mut m = T::from_f64(0.5).unwrap() * (l + r);

    // guarantee f(l) is false and f(r) is true
    if self.function.eval_prev(state) {
        std::mem::swap(&mut l, &mut r);
    }

    let mut w = (r - l).abs();
    let mut w_prev = T::from_f64(2.).unwrap() * w;

    while w < w_prev {
        w_prev = w;
        match self.function.eval_at(state, m) {
            false => l = m,
            true => r = m,
        }
        m = T::from_f64(0.5).unwrap() * (l + r);
        w = (r - l).abs();
    }
    T::max(l, r)
});
impl_locate!(Bisection, Output = T, |self, state| {
    let mut l = state.t_prev;
    let mut r = state.t_curr;

    let mut m = T::from_f64(0.5).unwrap() * (l + r);

    if self.function.eval_curr(state) < T::zero() {
        std::mem::swap(&mut l, &mut r);
    }

    let mut w = (r - l).abs();
    let mut w_prev = T::from_f64(2.).unwrap() * w;


    while w < w_prev {
        w_prev = w;
        match self.function.eval_at(state, m) < T::zero() {
            true => l = m,
            false => r = m,
        }
        m = T::from_f64(0.5).unwrap() * (l + r);
        w = (r - l).abs();
    }
    T::max(l, r)
});
impl_locate!(RegulaFalsi, Output = T, |self, state| {
    let mut l = state.t_prev;
    let mut r = state.t_curr;

    // guarantee f(l) < 0 and f(r) > 0
    if self.function.eval_curr(state) < T::zero() {
        std::mem::swap(&mut l, &mut r);
    }

    let mut w = (r - l).abs();
    let mut w_prev = T::from_f64(2.).unwrap() * w;

    while w < w_prev {
        w_prev = w;
        let f_l = self.function.eval_at(state, l);
        let f_r = self.function.eval_at(state, r);
        let m = (f_r * l - f_l * r) / (f_r - f_l);
        let f_m = self.function.eval_at(state, m);
        match f_m < T::zero() {
            false => l = m,
            true => r = m,
        }
        w = (r - l).abs();
    }
    T::max(l, r)
});
