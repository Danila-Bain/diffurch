use super::Loc;
use crate::{
    initial_condition::InitialCondition,
    loc::detect::Detect,
    state::{EvalStateFn, State},
};
use num::Float;

pub trait Locate<const N: usize, T> {
    fn locate<const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &State<N, S, S2, T, IC>,
    ) -> Option<T>;
}

// /// Trait for event location methods
// pub trait Locate<const N: usize> {
//     /// Locate the event if it is detected on the current step.
//     ///
//     /// Returns `None` if event were not detected.
//     fn locate(&mut self, state: &impl State<N>) -> Option<f64>;
// }

/// Use the previous step time as the location of event
pub struct StepBegin;
/// Use the current step time as the location of event
pub struct StepEnd;
/// Use the middle between previous and current step time as the location of event
pub struct StepMiddle;
/// Use the linear interpolation as an approximation for the location of event for `f64`-valued
/// detection functions (not supported for `bool` detection functions)
pub struct Lerp;

/// Use bisection method to find the location of event for `f64`-valued detection functions. See also: [BisectionBool].
pub struct Bisection;
/// Use bisection method to find the location of event for `bool`-valued detection functions. See also: [Bisection].
pub struct BisectionBool;
/// Use regula falsi method to find the location of event for `f64`-valued detection functions. See also: [Bisection]. Current implementation is not as reliable as [Bisection].
pub struct RegulaFalsi;

// macro_rules! impl_locate(
//     ($type:ty, $detect:ident, |$curr:ident $(, $prev:ident)?| $body:expr) => {
//         impl<const N: usize, T: Float, L, F: EvalStateFn<N, T, $type>> Detect<N, T> for Loc<F, $detect, L> {
//             fn detect<const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
//                 &mut self,
//                 state: &State<N, S, S2, T, IC>,
//             ) -> bool {
//                 let $curr = self.function.eval_curr(state);
//                 $(let $prev = self.function.eval_prev(state);)?
//                 $body
//             }
//         }
//     }
// );
impl<const N: usize, T: Float, F, D> Locate<N, T> for Loc<F, D, StepBegin>
where
    Self: Detect<N, T>,
{
    fn locate<const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &State<N, S, S2, T, IC>,
    ) -> Option<T> {
        self.detect(state).then(|| state.t_prev)
    }
}
impl<const N: usize, T: Float, F, D> Locate<N, T> for Loc<F, D, StepEnd>
where
    Self: Detect<N, T>,
{
    fn locate<const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &State<N, S, S2, T, IC>,
    ) -> Option<T> {
        self.detect(state).then(|| state.t_curr)
    }
}
impl<const N: usize, T: Float, F, D> Locate<N, T> for Loc<F, D, StepMiddle>
where
    Self: Detect<N, T>,
{
    fn locate<const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &State<N, S, S2, T, IC>,
    ) -> Option<T> {
        self.detect(state).then(|| T::from(0.5).unwrap()*(state.t_curr - state.t_prev))
    }
}
