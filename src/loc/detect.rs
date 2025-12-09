use super::Loc;
use crate::{
    initial_condition::InitialCondition,
    state::{EvalStateFn, State},
    traits::RealVectorSpace,
};
use nalgebra::RealField;

pub trait Detect<T: RealField + Copy, Y: RealVectorSpace<T>, const S: usize, const I: usize, IC: InitialCondition<T, Y>> {
    fn detect(
        &mut self,
        state: &State<T, Y, S, I, IC>,
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
        impl<
            T: RealField + Copy, 
            Y: RealVectorSpace<T>, 
            const S: usize,
            const I: usize,
            IC: InitialCondition<T, Y>,
            L, 
            F: EvalStateFn<T, Y, S, I, IC, $type>, 
        > Detect<T, Y, S, I, IC> for Loc<T, Y, S, I, IC, F, $detect, L> {
            fn detect(
                &mut self,
                state: &State<T, Y, S, I, IC>,
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
