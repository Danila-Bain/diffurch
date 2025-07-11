//! Defines [Loc] struct, which is an event locator

use std::mem::swap;

use std::ops::{Deref, DerefMut};

use crate::{EventCall, state::*};

macro_rules! impl_deref {
    ($name:ident) => {
        impl_deref!($name, f64);
    };
    ($name:ident, $type:ident) => {
        impl<const N: usize, F: StateFnMut<N, Output = $type>> Deref for $name<N, F> {
            type Target = F;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl<const N: usize, F: StateFnMut<N, Output = $type>> DerefMut for $name<N, F> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

/// Trait for event detection methods
pub trait Detect<const N: usize> {
    /// Determine whether event is occured on the current step
    fn detect(&mut self, state: &impl State<N>) -> bool;
}
/// Trait for event location methods
pub trait Locate<const N: usize> {
    /// Locate the event if it is detected on the current step.
    ///
    /// Returns `None` if event were not detected.
    fn locate(&mut self, state: &impl State<N>) -> Option<f64>;
}

/// Detect event if sign of an `f64`-valued function is changed between current and previous step.
///
/// Event is never detected if current value is zero.
///
/// Event is always detected if previous value is zero.
///
pub struct Sign<const N: usize, F: StateFnMut<N, Output = f64>>(pub F);
impl<const N: usize, F: StateFnMut<N, Output = f64>> Detect<N> for Sign<N, F> {
    fn detect(&mut self, state: &impl State<N>) -> bool {
        let curr = self.0.eval(state);
        let prev = self.0.eval_prev(state);
        curr > 0. && prev <= 0. || curr < 0. && prev >= 0.
    }
}
impl_deref!(Sign);

/// Detect event if the value of a function turns positive from non-positive.
pub struct Pos<const N: usize, F: StateFnMut<N, Output = f64>>(pub F);
impl<const N: usize, F: StateFnMut<N, Output = f64>> Detect<N> for Pos<N, F> {
    fn detect(&mut self, state: &impl State<N>) -> bool {
        let curr = self.0.eval(state);
        let prev = self.0.eval_prev(state);
        curr > 0. && prev <= 0.
    }
}
impl_deref!(Pos);

/// Detect event if the value of a function turns negative from non-negative.
pub struct Neg<const N: usize, F: StateFnMut<N, Output = f64>>(pub F);
impl<const N: usize, F: StateFnMut<N, Output = f64>> Detect<N> for Neg<N, F> {
    fn detect(&mut self, state: &impl State<N>) -> bool {
        let curr = self.0.eval(state);
        let prev = self.0.eval_prev(state);
        curr < 0. && prev >= 0.
    }
}
impl_deref!(Neg);

/// Detect event if the value of a function is positive. Contrary to [Pos], it retriggers if value
/// stays positive in next steps.
pub struct WhilePos<const N: usize, F: StateFnMut<N, Output = f64>>(pub F);
impl<const N: usize, F: StateFnMut<N, Output = f64>> Detect<N> for WhilePos<N, F> {
    fn detect(&mut self, state: &impl State<N>) -> bool {
        self.0.eval(state) >= 0.
    }
}
impl_deref!(WhilePos);

/// Detect event if the value of a function is negative. Contrary to [Neg], it retriggers if value
/// stays negative in next steps.
pub struct WhileNeg<const N: usize, F: StateFnMut<N, Output = f64>>(pub F);
impl<const N: usize, F: StateFnMut<N, Output = f64>> Detect<N> for WhileNeg<N, F> {
    fn detect(&mut self, state: &impl State<N>) -> bool {
        self.0.eval(state) <= 0.
    }
}
impl_deref!(WhileNeg);

/// Detect event if bool value changes between current and previous steps.
pub struct Bool<const N: usize, F: StateFnMut<N, Output = bool>>(pub F);
impl<const N: usize, F: StateFnMut<N, Output = bool>> Detect<N> for Bool<N, F> {
    fn detect(&mut self, state: &impl State<N>) -> bool {
        self.0.eval(state) != self.0.eval_prev(state)
    }
}
impl_deref!(Bool, bool);

/// Detect event if bool value changes from false to true between previous and current steps.
pub struct True<const N: usize, F: StateFnMut<N, Output = bool>>(pub F);
impl<const N: usize, F: StateFnMut<N, Output = bool>> Detect<N> for True<N, F> {
    fn detect(&mut self, state: &impl State<N>) -> bool {
        self.0.eval(state) && !self.0.eval_prev(state)
    }
}
impl_deref!(True, bool);

/// Detect event if bool value changes from true to false between previous and current steps.
pub struct False<const N: usize, F: StateFnMut<N, Output = bool>>(pub F);
impl<const N: usize, F: StateFnMut<N, Output = bool>> Detect<N> for False<N, F> {
    fn detect(&mut self, state: &impl State<N>) -> bool {
        !self.0.eval(state) && self.0.eval_prev(state)
    }
}
impl_deref!(False, bool);

/// Detect event if the bool value is true on the current state.
pub struct WhileTrue<const N: usize, F: StateFnMut<N, Output = bool>>(pub F);
impl<const N: usize, F: StateFnMut<N, Output = bool>> Detect<N> for WhileTrue<N, F> {
    fn detect(&mut self, state: &impl State<N>) -> bool {
        self.0.eval(state)
    }
}
impl_deref!(WhileTrue, bool);

/// Detect event if the bool value is false on the current state.
pub struct WhileFalse<const N: usize, F: StateFnMut<N, Output = bool>>(pub F);
impl<const N: usize, F: StateFnMut<N, Output = bool>> Detect<N> for WhileFalse<N, F> {
    fn detect(&mut self, state: &impl State<N>) -> bool {
        !self.0.eval(state)
    }
}
impl_deref!(WhileFalse, bool);

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

/// Struct that conatins detection + location methods.
pub struct Loc<D = (), L = ()>(pub D, pub L);

impl Loc {
    /// Constructor for detection method [Sign] and location method [Bisection]
    pub fn sign<const N: usize, F: StateFnMut<N, Output = f64>>(
        f: F,
    ) -> Loc<Sign<N, F>, Bisection> {
        Loc(Sign(f), Bisection)
    }
    /// Constructor for detection method [Pos] and location method [Bisection]
    pub fn pos<const N: usize, F: StateFnMut<N, Output = f64>>(f: F) -> Loc<Pos<N, F>, Bisection> {
        Loc(Pos(f), Bisection)
    }
    /// Constructor for detection method [Neg] and location method [Bisection]
    pub fn neg<const N: usize, F: StateFnMut<N, Output = f64>>(f: F) -> Loc<Neg<N, F>, Bisection> {
        Loc(Neg(f), Bisection)
    }
    /// Constructor for detection method [WhilePos] and location method [StepEnd]
    pub fn while_pos<const N: usize, F: StateFnMut<N, Output = f64>>(
        f: F,
    ) -> Loc<WhilePos<N, F>, StepEnd> {
        Loc(WhilePos(f), StepEnd)
    }
    /// Constructor for detection method [WhileNeg] and location method [StepEnd]
    pub fn while_neg<const N: usize, F: StateFnMut<N, Output = f64>>(
        f: F,
    ) -> Loc<WhileNeg<N, F>, StepEnd> {
        Loc(WhileNeg(f), StepEnd)
    }

    /// Constructor for detection method [Bool] and location method [BisectionBool]
    pub fn bool<const N: usize, F: StateFnMut<N, Output = bool>>(
        f: F,
    ) -> Loc<Bool<N, F>, BisectionBool> {
        Loc(Bool(f), BisectionBool)
    }
    /// Constructor for detection method [True] and location method [BisectionBool]
    pub fn true_<const N: usize, F: StateFnMut<N, Output = bool>>(
        f: F,
    ) -> Loc<True<N, F>, BisectionBool> {
        Loc(True(f), BisectionBool)
    }
    /// Constructor for detection method [False] and location method [BisectionBool]
    pub fn false_<const N: usize, F: StateFnMut<N, Output = bool>>(
        f: F,
    ) -> Loc<False<N, F>, BisectionBool> {
        Loc(False(f), BisectionBool)
    }
    /// Constructor for detection method [WhileTrue] and location method [StepEnd]
    pub fn while_true<const N: usize, F: StateFnMut<N, Output = bool>>(
        f: F,
    ) -> Loc<WhileTrue<N, F>, StepEnd> {
        Loc(WhileTrue(f), StepEnd)
    }
    /// Constructor for detection method [WhileFalse] and location method [StepEnd]
    pub fn while_false<const N: usize, F: StateFnMut<N, Output = bool>>(
        f: F,
    ) -> Loc<WhileFalse<N, F>, StepEnd> {
        Loc(WhileFalse(f), StepEnd)
    }
}

impl<D, L> Loc<D, L> {
    /// Self-consuming setter of location method [StepBegin]
    pub fn step_begin(self) -> Loc<D, StepBegin> {
        Loc(self.0, StepBegin)
    }
    /// Self-consuming setter of location method [StepEnd]
    pub fn step_end(self) -> Loc<D, StepEnd> {
        Loc(self.0, StepEnd)
    }
    /// Self-consuming setter of location method [StepHalf]
    pub fn step_half(self) -> Loc<D, StepHalf> {
        Loc(self.0, StepHalf)
    }
    /// Self-consuming setter of location method [Lerp]
    pub fn lerp(self) -> Loc<D, Lerp> {
        Loc(self.0, Lerp)
    }
    /// Self-consuming setter of location method [Bisection]
    pub fn bisection(self) -> Loc<D, Bisection> {
        Loc(self.0, Bisection)
    }
    /// Self-consuming setter of location method [BisectionBool]
    pub fn bisection_bool(self) -> Loc<D, BisectionBool> {
        Loc(self.0, BisectionBool)
    }
    /// Self-consuming setter of location method [RegulaFalsi]
    pub fn regula_falsi(self) -> Loc<D, RegulaFalsi> {
        Loc(self.0, RegulaFalsi)
    }
}

impl<const N: usize, D: Detect<N>> Locate<N> for Loc<D, StepBegin> {
    fn locate(&mut self, state: &impl State<N>) -> Option<f64> {
        self.0.detect(state).then_some(state.t_prev())
    }
}
impl<const N: usize, D: Detect<N>> Locate<N> for Loc<D, StepEnd> {
    fn locate(&mut self, state: &impl State<N>) -> Option<f64> {
        self.0.detect(state).then_some(state.t())
    }
}
impl<const N: usize, D: Detect<N>> Locate<N> for Loc<D, StepHalf> {
    fn locate(&mut self, state: &impl State<N>) -> Option<f64> {
        self.0
            .detect(state)
            .then(|| 0.5 * (state.t_prev() + state.t()))
    }
}
impl<const N: usize, D> Locate<N> for Loc<D, Lerp>
where
    D: Detect<N> + DerefMut,
    <D as Deref>::Target: StateFnMut<N, Output = f64>,
{
    fn locate(&mut self, state: &impl State<N>) -> Option<f64> {
        self.0.detect(state).then(|| {
            let curr = self.0.eval(state);
            let prev = self.0.eval_prev(state);
            (curr * state.t_prev() - prev * state.t()) / (curr - prev)
        })
    }
}

impl<const N: usize, D> Locate<N> for Loc<D, BisectionBool>
where
    D: Detect<N> + DerefMut,
    <D as Deref>::Target: StateFnMut<N, Output = bool>,
{
    fn locate(&mut self, state: &impl State<N>) -> Option<f64> {
        self.0.detect(state).then(|| {
            let mut l = state.t_prev();
            let mut r = state.t();
            if self.0.eval_prev(state) {
                swap(&mut l, &mut r);
            }

            for _ in 0..f64::MANTISSA_DIGITS {
                // optimizable
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

impl<const N: usize, D> Locate<N> for Loc<D, RegulaFalsi>
where
    D: Detect<N> + DerefMut,
    <D as Deref>::Target: StateFnMut<N, Output = f64>,
{
    fn locate(&mut self, state: &impl State<N>) -> Option<f64> {
        self.0.detect(state).then(|| {
            let mut l = state.t_prev();
            let mut r = state.t();
            if self.0.eval(state) < 0. {
                swap(&mut l, &mut r);
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

impl<const N: usize, D> Locate<N> for Loc<D, Bisection>
where
    D: Detect<N> + DerefMut,
    <D as Deref>::Target: StateFnMut<N, Output = f64>,
{
    fn locate(&mut self, state: &impl State<N>) -> Option<f64> {
        self.0.detect(state).then(|| {
            let mut l = state.t_prev();
            let mut r = state.t();
            if self.0.eval(state) < 0. {
                swap(&mut l, &mut r);
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

// struct DelayedArgument<const N: usize, F: StateFnMut<N, f64>> {
//     f: F
// }

// pub trait DelayedArgument<const N: usize>: StateFnMut<N, f64> {
//
// }

pub struct Propagated<Alpha> {
    /// at which order we shall stop propagating
    pub order: usize,
    /// index into state disco queue, it is assumed,
    /// that previous evaluation is on the half-open interval
    /// [ state.disco()[disco_idx - 1], state.disco[disco_idx] )
    /// where state.disco()[-1] is f64::NEG_INFINITY
    pub disco_idx: isize,
    pub last_t: f64,
    pub order_increase: usize,
    /// Deviated argument function
    pub alpha: Alpha,
}

impl<Alpha> Propagated<Alpha> {
    pub fn new(alpha: Alpha) -> Self {
        Propagated {
            order: 0,
            alpha,
            disco_idx: 0,
            last_t: f64::NEG_INFINITY,
            order_increase: 1,
        }
    }
}

impl<const N: usize, Alpha: StateFnMut<N, Output = f64>> Locate<N> for Propagated<Alpha> {
    fn locate(&mut self, state: &impl State<N>) -> Option<f64> {
        // we assume that delay function is continuous, because otherwise
        // additional events need to be introduced externally anyway

        let alpha_value = self.alpha.eval(state);

        for (idx, dir) in [(self.disco_idx - 1, -1isize), (self.disco_idx, 1)].iter() {
            if let Some((t, order)) = state.disco().get(*idx as usize)
                && alpha_value < *t
            {
                let t_loc = Loc(
                    Sign(StateFnMutComposition(
                        &mut |alpha_| alpha_ - *t,
                        &mut self.alpha,
                    )),
                    Bisection,
                )
                .locate(state);

                if let Some(t_loc) = t_loc {
                    self.order = *order;
                    self.last_t = t_loc;
                    self.disco_idx += dir;
                    return Some(t_loc)
                } else {
                    return None
                }
            }
        }
        None
    }
}

impl<const N: usize, Alpha: StateFnMut<N, Output = f64>> EventCall<N>
    for Propagated<Alpha>
{
    fn call(&mut self, state: &mut impl State<N>) {
        let t = state.t();
        state.disco_mut().push_back((t, self.order + self.order_increase))
    }
}
