use super::Loc;
use crate::state::{State, StateFnMut};

/// Trait for event detection methods
pub trait Detect<const N: usize> {
    /// Determine whether event is occured on the current step
    fn detect(&mut self, state: &impl State<N>) -> bool;
}

pub struct Sign;
pub struct Pos;
pub struct Neg;
pub struct WhilePos;
pub struct WhileNeg;
pub struct Bool;
pub struct True;
pub struct False;
pub struct WhileTrue;
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
