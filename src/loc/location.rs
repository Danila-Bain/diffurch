use super::{Detect, Loc};
use crate::state::{State, StateFnMut};

/// Trait for event location methods
pub trait Locate<const N: usize> {
    /// Locate the event if it is detected on the current step.
    ///
    /// Returns `None` if event were not detected.
    fn locate(&mut self, state: &impl State<N>) -> Option<f64>;
}

/// Use the previous step time as the location of event
pub struct StepBegin;
/// Use the current step time as the location of event
pub struct StepEnd;
/// Use the middle between previous and current step time as the location of event
pub struct StepHalf;
/// Use the linear interpolation as an approximation for the location of event for `f64`-valued
/// detection functions (not supported for `bool` detection functions)
pub struct Lerp;

/// Use bisection method to find the location of event for `f64`-valued detection functions. See also: [BisectionBool].
pub struct Bisection;
/// Use bisection method to find the location of event for `bool`-valued detection functions. See also: [Bisection].
pub struct BisectionBool;
/// Use regula falsi method to find the location of event for `f64`-valued detection functions. See also: [Bisection]. Current implementation is not as reliable as [Bisection].
pub struct RegulaFalsi;

impl<const N: usize, F, D> Locate<N> for Loc<F, D, StepBegin>
where
    Self: Detect<N>,
{
    fn locate(&mut self, state: &impl State<N>) -> Option<f64> {
        self.detect(state).then_some(state.t_prev())
    }
}
impl<const N: usize, F, D> Locate<N> for Loc<F, D, StepEnd>
where
    Self: Detect<N>,
{
    fn locate(&mut self, state: &impl State<N>) -> Option<f64> {
        self.detect(state).then_some(state.t())
    }
}
impl<const N: usize, F, D> Locate<N> for Loc<F, D, StepHalf>
where
    Self: Detect<N>,
{
    fn locate(&mut self, state: &impl State<N>) -> Option<f64> {
        self.detect(state)
            .then(|| 0.5 * (state.t_prev() + state.t()))
    }
}
impl<const N: usize, F: StateFnMut<N, Output = f64>, D> Locate<N> for Loc<F, D, Lerp>
where
    Self: Detect<N>,
{
    fn locate(&mut self, state: &impl State<N>) -> Option<f64> {
        self.detect(state).then(|| {
            let curr = self.0.eval(state);
            let prev = self.0.eval_prev(state);
            (curr * state.t_prev() - prev * state.t()) / (curr - prev)
        })
    }
}

impl<const N: usize, F: StateFnMut<N, Output = bool>, D> Locate<N> for Loc<F, D, BisectionBool>
where
    Self: Detect<N>,
{
    fn locate(&mut self, state: &impl State<N>) -> Option<f64> {
        self.detect(state).then(|| {
            let mut l = state.t_prev();
            let mut r = state.t();
            if self.0.eval_prev(state) {
                std::mem::swap(&mut l, &mut r);
            }

            for _ in 0..f64::MANTISSA_DIGITS {
                // optimizable by stop conditions
                let m = 0.5 * (l + r);
                match self.0.eval_at(state, m) {
                    false => l = m,
                    true => r = m,
                }
            }
            f64::max(l, r)
        })
    }
}

impl<const N: usize, F: StateFnMut<N, Output = f64>, D> Locate<N> for Loc<F, D, RegulaFalsi>
where
    Self: Detect<N>,
{
    fn locate(&mut self, state: &impl State<N>) -> Option<f64> {
        self.detect(state).then(|| {
            let mut l = state.t_prev();
            let mut r = state.t();
            if self.0.eval(state) < 0. {
                std::mem::swap(&mut l, &mut r);
            }

            let mut m = 0.;
            for _ in 0..f64::MANTISSA_DIGITS {
                let f_l = self.0.eval_at(state, l);
                let f_r = self.0.eval_at(state, r);
                m = (f_r * l - f_l * r) / (f_r - f_l);
                let f_m = self.0.eval_at(state, m);
                match f_m < 0. {
                    true => {
                        l = m;
                    }
                    false => {
                        r = m;
                    }
                }
                if f_m.abs() < f64::EPSILON {
                    break;
                }
            }
            m
        })
    }
}

impl<const N: usize, F: StateFnMut<N, Output = f64>, D> Locate<N> for Loc<F, D, Bisection>
where
    Self: Detect<N>,
{
    fn locate(&mut self, state: &impl State<N>) -> Option<f64> {
        self.detect(state).then(|| {
            let mut l = state.t_prev();
            let mut r = state.t();
            if self.0.eval(state) < 0. {
                std::mem::swap(&mut l, &mut r);
            }

            for _ in 0..f64::MANTISSA_DIGITS {
                let m = 0.5 * (l + r);
                match self.0.eval_at(state, m) < 0. {
                    true => l = m,
                    false => r = m,
                }
            }
            f64::max(l, r)
        })
    }
}
