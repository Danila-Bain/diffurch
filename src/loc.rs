//! Defines [Loc] struct, which is an event locator

use std::mem::swap;

use crate::{State, StateFn};

/// This enum contains various detection methods, which determine, whether an event
/// occured on last step.
pub enum Detection<'a, const N: usize> {
    /// Detect the change of sign of the value
    Sign(StateFn<'a, N, f64>),
    /// Detect the change of sign of the value from negative to positive
    SignToPos(StateFn<'a, N, f64>),
    /// Detect the change of sign of the value from positive to negative
    SignToNeg(StateFn<'a, N, f64>),
    /// Detect the negative sign of the value
    ///
    /// Will retrigger, if the value continues to be negative after the event
    SignNeg(StateFn<'a, N, f64>),
    /// Detect the positive sign of the value
    ///
    /// Will retrigger, if the value continues to be positive after the event
    SignPos(StateFn<'a, N, f64>),
    /// Detect the change of the bool value
    Bool(StateFn<'a, N, bool>),
    /// Detect the change of the bool value from false to true
    BoolToTrue(StateFn<'a, N, bool>),
    /// Detect the change of the bool value from true to false
    BoolToFalse(StateFn<'a, N, bool>),
}

/// This enum lists methods of location for the detected event. I.e. producing the exact (or not so
/// exact) time when event happend.
pub enum LocMethod {
    /// Take the last step position as event location
    StepBegin,
    /// Take the current step position as event location
    StepEnd,
    /// Take the position in the middle of a step as event location
    StepMiddle,
    /// Use linear interpolation to approximate event location. For [Detection::Bool] and similar
    /// variants, it is equivalent to [LocMethod::StepMiddle].
    Lerp,
    /// Use bisection methods to approximate event location. In current implementation, it always
    /// uses `f64::MANTISSA_DIGITS` iterations. Suitable for both [Detection::Sign] and
    /// [Detection::Bool] variants.
    Bisection,
    /// Use Brent method. (Not implemented yet).
    Brent,
}

/// Struct that holds [Detection] and [LocMethod] together.
pub struct Loc<'a, const N: usize> {
    /// detection part of the event
    pub detection: Detection<'a, N>,
    /// location part of the event
    pub location: LocMethod,
    /// Functions, that filter detection, they are called if [Loc::detection] returns true, and
    /// if any one function in [Loc::filter] is false, the event is considered undetected. 
    pub filter: Vec<crate::StateFnMut<'a, N, bool>>,
}

impl<'a, const N: usize> crate::Filter<'a, N> for Loc<'a, N> {
    fn filter(mut self, f: crate::StateFnMut<'a, N, bool>) -> Self {
        self.filter.push(f);
        self
    }
}

impl<'a, const N: usize> Loc<'a, N> {
    /// Constructor for [Detection::Sign] variant. Defaults location to [LocMethod::Bisection].
    pub fn zero(f: StateFn<'a, N, f64>) -> Self {
        Self {
            detection: Detection::Sign(f),
            location: LocMethod::Bisection,
            filter: Vec::new(),
        }
    }
    /// Constructor for [Detection::SignToPos] variant. Defaults location to [LocMethod::Bisection].
    pub fn to_pos(f: StateFn<'a, N, f64>) -> Self {
        Self {
            detection: Detection::SignToPos(f),
            location: LocMethod::Bisection,
            filter: Vec::new(),
        }
    }
    /// Constructor for [Detection::SignToNeg] variant. Defaults location to [LocMethod::Bisection].
    pub fn to_neg(f: StateFn<'a, N, f64>) -> Self {
        Self {
            detection: Detection::SignToNeg(f),
            location: LocMethod::Bisection,
            filter: Vec::new(),
        }
    }
    /// Constructor for [Detection::SignNeg] variant. Defaults location to [LocMethod::Bisection].
    pub fn neg(f: StateFn<'a, N, f64>) -> Self {
        Self {
            detection: Detection::SignNeg(f),
            location: LocMethod::Bisection,
            filter: Vec::new(),
        }
    }
    /// Constructor for [Detection::Pos] variant. Defaults location to [LocMethod::Bisection].
    pub fn pos(f: StateFn<'a, N, f64>) -> Self {
        Self {
            detection: Detection::SignPos(f),
            location: LocMethod::Bisection,
            filter: Vec::new(),
        }
    }
    /// Constructor for [Detection::Bool] variant. Defaults location to [LocMethod::Bisection].
    pub fn flip(f: StateFn<'a, N, bool>) -> Self {
        Self {
            detection: Detection::Bool(f),
            location: LocMethod::Bisection,
            filter: Vec::new(),
        }
    }
    /// Constructor for [Detection::BoolToTrue] variant. Defaults location to [LocMethod::Bisection].
    pub fn to_true(f: StateFn<'a, N, bool>) -> Self {
        Self {
            detection: Detection::BoolToTrue(f),
            location: LocMethod::Bisection,
            filter: Vec::new(),
        }
    }
    /// Constructor for [Detection::BoolToFalse] variant. Defaults location to [LocMethod::Bisection].
    pub fn to_false(f: StateFn<'a, N, bool>) -> Self {
        Self {
            detection: Detection::BoolToFalse(f),
            location: LocMethod::Bisection,
            filter: Vec::new(),
        }
    }

    /// Implements detection for all [Detection] variants. Returns `true` if the event is
    /// detected between the last and current step of state.
    pub fn detect<const S: usize>(&self, state: &'a State<'a, N, S>) -> bool {
        match &self.detection {
            Detection::Sign(f) => {
                let curr = f.eval(state);
                let prev = f.eval_prev(state);
                curr > 0. && prev <= 0. || curr < 0. && prev >= 0.
            }
            Detection::SignToPos(f) => {
                let curr = f.eval(state);
                let prev = f.eval_prev(state);
                curr > 0. && prev <= 0.
            }
            Detection::SignToNeg(f) => {
                let curr = f.eval(state);
                let prev = f.eval_prev(state);
                curr < 0. && prev >= 0.
            }
            Detection::SignNeg(f) => f.eval(state) <= 0.,
            Detection::SignPos(f) => f.eval(state) >= 0.,
            Detection::Bool(f) => f.eval(state) != f.eval_prev(state),
            Detection::BoolToTrue(f) => f.eval(state) && !f.eval_prev(state),
            Detection::BoolToFalse(f) => !f.eval(state) && f.eval_prev(state),
        }
    }

    /// Implements location methods for all [LocMethod] variants, utilizing functions provided by
    /// [Detection]. Returns the time at which event is approximated to be located.
    pub fn locate<'b, const S: usize>(&mut self, state: &'b State<'a, N, S>) -> Option<f64> {
        // if !self.detect(&state) || !self.filter.iter_mut().all(|f| f.eval(state)) {
        if !self.detect(&state) {
            return None;
        } else {
            match self.location {
                LocMethod::StepBegin => Some(state.t_prev),
                LocMethod::StepEnd => Some(state.t),
                LocMethod::StepMiddle => Some(0.5 * (state.t_prev + state.t)),
                LocMethod::Lerp => match &self.detection {
                    Detection::Bool(_) | Detection::BoolToTrue(_) | Detection::BoolToFalse(_) => {
                        Some(0.5 * (state.t_prev + state.t))
                    }
                    Detection::Sign(f) | Detection::SignToPos(f) | Detection::SignToNeg(f) => {
                        let curr = f.eval(state);
                        let prev = f.eval_prev(state);
                        Some((curr * state.t_prev - prev * state.t) / (curr - prev))
                    }
                    Detection::SignNeg(f) | Detection::SignPos(f) => {
                        let curr = f.eval(state);
                        let prev = f.eval_prev(state);
                        if (prev > 0.) != (curr > 0.) {
                            Some((curr * state.t_prev - prev * state.t) / (curr - prev))
                        } else {
                            Some(state.t_prev)
                        }
                    }
                },
                LocMethod::Bisection => match &self.detection {
                    Detection::Bool(f) | Detection::BoolToTrue(f) | Detection::BoolToFalse(f) => {
                        let mut l = state.t_prev;
                        let mut r = state.t;
                        if f.eval_prev(state) {
                            swap(&mut l, &mut r);
                        }

                        for _ in 0..f64::MANTISSA_DIGITS {
                            let m = 0.5 * (l + r);
                            match f.eval_at(state, m) {
                                false => l = m,
                                true => r = m,
                            }
                        }
                        Some(f64::max(l, r))
                    }

                    Detection::Sign(f)
                    | Detection::SignToPos(f)
                    | Detection::SignToNeg(f)
                    | Detection::SignPos(f)
                    | Detection::SignNeg(f) => {
                        let mut l = state.t_prev;
                        let mut r = state.t;
                        if f.eval(state) < 0. {
                            swap(&mut l, &mut r);
                        }

                        for _ in 0..f64::MANTISSA_DIGITS {
                            let m = 0.5 * (l + r);
                            match f.eval_at(state, m) < 0. {
                                true => l = m,
                                false => r = m,
                            }
                        }
                        Some(f64::max(l, r))
                    }
                },

                LocMethod::Brent => todo!(),
            }
        }
    }
}
