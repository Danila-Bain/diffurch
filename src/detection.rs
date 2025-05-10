use std::mem::swap;

use crate::{State, StateFn};

pub enum Detection<'a, const N: usize> {
    Sign(StateFn<'a, N, f64>),
    SignToPos(StateFn<'a, N, f64>),
    SignToNeg(StateFn<'a, N, f64>),
    SignNeg(StateFn<'a, N, f64>),
    SignPos(StateFn<'a, N, f64>),
    Bool(StateFn<'a, N, bool>),
    BoolToTrue(StateFn<'a, N, bool>),
    BoolToFalse(StateFn<'a, N, bool>),
}

pub enum LocationMethod {
    StepBegin,
    StepEnd,
    StepMiddle,
    Lerp,
    Bisection,
    Brent,
}

pub struct EventLocator<'a, const N: usize> {
    pub detection: Detection<'a, N>,
    pub location: LocationMethod,
}

impl<'a, const N: usize> EventLocator<'a, N> {
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

    pub fn locate<const S: usize>(&self, state: &'a State<'a, N, S>) -> f64 {
        match self.location {
            LocationMethod::StepBegin => state.t_prev,
            LocationMethod::StepEnd => state.t,
            LocationMethod::StepMiddle => 0.5 * (state.t_prev + state.t),
            LocationMethod::Lerp => match &self.detection {
                Detection::Bool(_) | Detection::BoolToTrue(_) | Detection::BoolToFalse(_) => {
                    0.5 * (state.t_prev + state.t)
                }
                Detection::Sign(f) | Detection::SignToPos(f) | Detection::SignToNeg(f) => {
                    let curr = f.eval(state);
                    let prev = f.eval_prev(state);
                    (curr * state.t_prev - prev * state.t) / (curr - prev)
                }
                Detection::SignNeg(f) | Detection::SignPos(f) => {
                    let curr = f.eval(state);
                    let prev = f.eval_prev(state);
                    if (prev > 0.) != (curr > 0.) {
                        (curr * state.t_prev - prev * state.t) / (curr - prev)
                    } else {
                        state.t_prev
                    }
                }
            },
            LocationMethod::Bisection => match &self.detection {
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
                    return f64::max(l, r);
                }

                Detection::Sign(f)
                | Detection::SignToPos(f)
                | Detection::SignToNeg(f)
                | Detection::SignPos(f)
                | Detection::SignNeg(f) => {
                    let mut l = state.t_prev;
                    let mut r = state.t;
                    if f.eval_prev(state) > 0. {
                        swap(&mut l, &mut r);
                    }

                    for _ in 0..f64::MANTISSA_DIGITS {
                        let m = 0.5 * (l + r);
                        match f.eval_at(state, m) > 0. {
                            false => l = m,
                            true => r = m,
                        }
                    }
                    return f64::max(l, r);
                }
            },

            LocationMethod::Brent => todo!(),
        }
    }
}
