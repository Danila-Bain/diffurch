use super::Loc;
use crate::{
    initial_condition::InitialCondition,
    state::{EvalStateFn, State},
};
use num::Float;

pub trait Detect<const N: usize, T> {
    fn detect<const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &State<N, S, S2, T, IC>,
    ) -> bool;
}

/// Detection method marker for detection of change of sign of the state function.
pub struct Zero;
/// Detection method marker for detection of state function value becoming positive.
pub struct AboveZero;
/// Detection method marker for detection of state function value becoming negative.
pub struct BelowZero;
/// Detection method marker for detection of state function value being positive.
pub struct Positive;
/// Detection method marker for detection of state function value being negative.
pub struct Negative;
/// Detection method marker for detection of change of bool value of the state function.
pub struct Switch;
/// Detection method marker for detection of state function value becoming `true`.
pub struct SwitchTrue;
/// Detection method marker for detection of state function value becoming `false`.
pub struct SwitchFalse;
/// Detection method marker for detection of state function value being `true`.
pub struct IsTrue;
/// Detection method marker for detection of state function value being `false`.
pub struct IsFalse;

macro_rules! impl_detect(
    ($type:ty, $detect:ident, |$curr:ident $(, $prev:ident)?| $body:expr) => {
        impl<const N: usize, T: Float, L, F: EvalStateFn<N, T, $type>> Detect<N, T> for Loc<F, $detect, L> {
            fn detect<const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
                &mut self,
                state: &State<N, S, S2, T, IC>,
            ) -> bool {
                  let $curr = self.function.eval_curr(state);
                $(let $prev = self.function.eval_prev(state);)?
                  $body
            }
        }
    }
);

impl_detect!(T, Zero, |curr, prev| {
    curr >= T::zero() && prev < T::zero() || curr <= T::zero() && prev > T::zero()
});
impl_detect!(T, AboveZero, |curr, prev| {
    curr >= T::zero() && prev < T::zero()
});
impl_detect!(T, BelowZero, |curr, prev| {
    curr <= T::zero() && prev > T::zero()
});
impl_detect!(T, Positive, |curr| curr >= T::zero());
impl_detect!(T, Negative, |curr| curr <= T::zero());
impl_detect!(bool, Switch, |curr, prev| curr != prev);
impl_detect!(bool, SwitchTrue, |curr, prev| curr && !prev);
impl_detect!(bool, SwitchFalse, |curr, prev| !curr && prev);
impl_detect!(bool, IsTrue, |curr| curr);
impl_detect!(bool, IsFalse, |curr| !curr);
