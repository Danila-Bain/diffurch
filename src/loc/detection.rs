use super::Loc;
use crate::state::{State, StateFnMut};

/// Trait for event detection methods
pub trait Detect<const N: usize> {
    /// Determine whether event is occured on the current step
    fn detect(&mut self, state: &impl State<N>) -> bool;
}
/// Detection method marker for detection of change of sign of the state function.
pub struct Sign;
/// Detection method marker for detection of state function value becoming positive.
pub struct Pos;
/// Detection method marker for detection of state function value becoming negative.
pub struct Neg;
/// Detection method marker for detection of state function value being positive.
pub struct WhilePos;
/// Detection method marker for detection of state function value being negative.
pub struct WhileNeg;
/// Detection method marker for detection of change of bool value of the state function.
pub struct Bool;
/// Detection method marker for detection of state function value becoming `true`.
pub struct True;
/// Detection method marker for detection of state function value becoming `false`.
pub struct False;
/// Detection method marker for detection of state function value being `true`.
pub struct WhileTrue;
/// Detection method marker for detection of state function value being `false`.
pub struct WhileFalse;

impl<const N: usize, F: StateFnMut<N, Output = f64>, L> Detect<N> for Loc<F, Sign, L> {
    fn detect(&mut self, state: &impl State<N>) -> bool {
        let curr = self.0.eval(state);
        let prev = self.0.eval_prev(state);
        curr >= 0. && prev < 0. || curr <= 0. && prev > 0.
    }
}
impl<const N: usize, F: StateFnMut<N, Output = f64>, L> Detect<N> for Loc<F, Pos, L> {
    fn detect(&mut self, state: &impl State<N>) -> bool {
        let curr = self.0.eval(state);
        let prev = self.0.eval_prev(state);
        curr > 0. && prev <= 0.
    }
}
impl<const N: usize, F: StateFnMut<N, Output = f64>, L> Detect<N> for Loc<F, Neg, L> {
    fn detect(&mut self, state: &impl State<N>) -> bool {
        let curr = self.0.eval(state);
        let prev = self.0.eval_prev(state);
        curr < 0. && prev >= 0.
    }
}
impl<const N: usize, F: StateFnMut<N, Output = f64>, L> Detect<N> for Loc<F, WhilePos, L> {
    fn detect(&mut self, state: &impl State<N>) -> bool {
        let curr = self.0.eval(state);
        curr >= 0.
    }
}
impl<const N: usize, F: StateFnMut<N, Output = f64>, L> Detect<N> for Loc<F, WhileNeg, L> {
    fn detect(&mut self, state: &impl State<N>) -> bool {
        let curr = self.0.eval(state);
        curr <= 0.
    }
}
impl<const N: usize, F: StateFnMut<N, Output = bool>, L> Detect<N> for Loc<F, Bool, L> {
    fn detect(&mut self, state: &impl State<N>) -> bool {
        self.0.eval(state) != self.0.eval_prev(state)
    }
}
impl<const N: usize, F: StateFnMut<N, Output = bool>, L> Detect<N> for Loc<F, True, L> {
    fn detect(&mut self, state: &impl State<N>) -> bool {
        self.0.eval(state) && !self.0.eval_prev(state)
    }
}
impl<const N: usize, F: StateFnMut<N, Output = bool>, L> Detect<N> for Loc<F, False, L> {
    fn detect(&mut self, state: &impl State<N>) -> bool {
        !self.0.eval(state) && self.0.eval_prev(state)
    }
}
impl<const N: usize, F: StateFnMut<N, Output = bool>, L> Detect<N> for Loc<F, WhileTrue, L> {
    fn detect(&mut self, state: &impl State<N>) -> bool {
        self.0.eval(state)
    }
}
impl<const N: usize, F: StateFnMut<N, Output = bool>, L> Detect<N> for Loc<F, WhileFalse, L> {
    fn detect(&mut self, state: &impl State<N>) -> bool {
        !self.0.eval(state)
    }
}
