//! Defines [Loc] struct, which is an event locator

use std::mem::swap;

use std::ops::{Deref, DerefMut};

use crate::state::*;

macro_rules! impl_deref {
    ($name:ident) => {
        impl_deref!($name, f64);
    };
    ($name:ident, $type:ident) => {
        impl<const N: usize, F: StateFnMut<N, $type>> Deref for $name<N, F> {
            type Target = F;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl<const N: usize, F: StateFnMut<N, $type>> DerefMut for $name<N, F> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

pub trait Detect<const N: usize> {
    fn detect<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> bool;
}
pub trait Locate<const N: usize> {
    fn locate<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Option<f64>;
}

pub mod detection {
    use super::*;

    pub struct Sign<const N: usize, F: StateFnMut<N, f64>>(pub F);
    impl<const N: usize, F: StateFnMut<N, f64>> Detect<N> for Sign<N, F> {
        fn detect<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> bool {
            let curr = self.0.eval(state);
            let prev = self.0.eval_prev(state);
            curr > 0. && prev <= 0. || curr < 0. && prev >= 0.
        }
    }
    impl_deref!(Sign);

    pub struct Pos<const N: usize, F: StateFnMut<N, f64>>(pub F);
    impl<const N: usize, F: StateFnMut<N, f64>> Detect<N> for Pos<N, F> {
        fn detect<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> bool {
            let curr = self.0.eval(state);
            let prev = self.0.eval_prev(state);
            curr > 0. && prev <= 0.
        }
    }
    impl_deref!(Pos);

    pub struct Neg<const N: usize, F: StateFnMut<N, f64>>(pub F);
    impl<const N: usize, F: StateFnMut<N, f64>> Detect<N> for Neg<N, F> {
        fn detect<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> bool {
            let curr = self.0.eval(state);
            let prev = self.0.eval_prev(state);
            curr < 0. && prev >= 0.
        }
    }
    impl_deref!(Neg);

    pub struct WhilePos<const N: usize, F: StateFnMut<N, f64>>(pub F);
    impl<const N: usize, F: StateFnMut<N, f64>> Detect<N> for WhilePos<N, F> {
        fn detect<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> bool {
            self.0.eval(state) >= 0.
        }
    }
    impl_deref!(WhilePos);

    pub struct WhileNeg<const N: usize, F: StateFnMut<N, f64>>(pub F);
    impl<const N: usize, F: StateFnMut<N, f64>> Detect<N> for WhileNeg<N, F> {
        fn detect<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> bool {
            self.0.eval(state) <= 0.
        }
    }
    impl_deref!(WhileNeg);

    pub struct Bool<const N: usize, F: StateFnMut<N, bool>>(pub F);
    impl<const N: usize, F: StateFnMut<N, bool>> Detect<N> for Bool<N, F> {
        fn detect<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> bool {
            self.0.eval(state) != self.0.eval_prev(state)
        }
    }
    impl_deref!(Bool, bool);

    pub struct True<const N: usize, F: StateFnMut<N, bool>>(pub F);
    impl<const N: usize, F: StateFnMut<N, bool>> Detect<N> for True<N, F> {
        fn detect<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> bool {
            self.0.eval(state) && !self.0.eval_prev(state)
        }
    }
    impl_deref!(True, bool);

    pub struct False<const N: usize, F: StateFnMut<N, bool>>(pub F);
    impl<const N: usize, F: StateFnMut<N, bool>> Detect<N> for False<N, F> {
        fn detect<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> bool {
            !self.0.eval(state) && self.0.eval_prev(state)
        }
    }
    impl_deref!(False, bool);

    pub struct WhileTrue<const N: usize, F: StateFnMut<N, bool>>(pub F);
    impl<const N: usize, F: StateFnMut<N, bool>> Detect<N> for WhileTrue<N, F> {
        fn detect<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> bool {
            self.0.eval(state)
        }
    }
    impl_deref!(WhileTrue, bool);

    pub struct WhileFalse<const N: usize, F: StateFnMut<N, bool>>(pub F);
    impl<const N: usize, F: StateFnMut<N, bool>> Detect<N> for WhileFalse<N, F> {
        fn detect<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> bool {
            !self.0.eval(state)
        }
    }
    impl_deref!(WhileFalse, bool);
}

pub mod location {
    pub struct StepBegin;
    pub struct StepEnd;
    pub struct HalfStep;

    pub struct Bisection;
    pub struct BisectionBool;
    pub struct RegulaFalsi;

    pub struct Lerp;
    pub struct Brent;
}
pub use location::*;

pub struct Loc<D, L>(pub D, pub L);

impl<const N: usize, D: Detect<N>> Locate<N> for Loc<D, StepBegin> {
    fn locate<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Option<f64> {
        self.0.detect(state).then_some(state.t_prev)
    }
}
impl<const N: usize, D: Detect<N>> Locate<N> for Loc<D, StepEnd> {
    fn locate<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Option<f64> {
        self.0.detect(state).then_some(state.t)
    }
}
impl<const N: usize, D: Detect<N>> Locate<N> for Loc<D, HalfStep> {
    fn locate<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Option<f64> {
        self.0.detect(state).then(|| 0.5 * (state.t_prev + state.t))
    }
}
impl<const N: usize, D: Detect<N> + DerefMut> Locate<N> for Loc<D, Lerp>
where
    <D as Deref>::Target: StateFnMut<N, f64>,
{
    fn locate<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Option<f64> {
        self.0.detect(state).then(|| {
            let curr = self.0.eval(state);
            let prev = self.0.eval_prev(state);
            (curr * state.t_prev - prev * state.t) / (curr - prev)
        })
    }
}

impl<const N: usize, D: Detect<N> + DerefMut> Locate<N> for Loc<D, BisectionBool>
where
    <D as Deref>::Target: StateFnMut<N, bool>,
{
    fn locate<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Option<f64> {
        self.0.detect(state).then(|| {
            let mut l = state.t_prev;
            let mut r = state.t;
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

impl<const N: usize, D: Detect<N> + DerefMut> Locate<N> for Loc<D, RegulaFalsi>
where
    <D as Deref>::Target: StateFnMut<N, f64>,
{
    fn locate<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Option<f64> {
        self.0.detect(state).then(|| {
            let mut l = state.t_prev;
            let mut r = state.t;
            if self.0.eval(state) < 0. {
                swap(&mut l, &mut r);
            }

            for _ in 0..f64::MANTISSA_DIGITS {
                let f_l = self.0.eval_at(state, l);
                let f_r = self.0.eval_at(state, r);
                let m = (f_r * l - f_l * r) / (f_r - f_l);
                match self.0.eval_at(state, m) < 0. {
                    true => l = m,
                    false => r = m,
                }
            }
            f64::max(l, r)
        })
    }
}

impl<const N: usize, D: Detect<N> + DerefMut> Locate<N> for Loc<D, Bisection>
where
    <D as Deref>::Target: StateFnMut<N, f64>,
{
    fn locate<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Option<f64> {
        self.0.detect(state).then(|| {
            let mut l = state.t_prev;
            let mut r = state.t;
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

// pub struct Loc<'a, const N: usize> {
//     /// detection part of the event
//     pub detection: Detection<'a, N>,
//     /// location part of the event
//     pub location: LocMethod,
//     /// Functions, that filter detection, they are called if [Loc::detection] returns true, and
//     /// if any one function in [Loc::filter] is false, the event is considered undetected.
//     pub filter: Vec<crate::StateFnMut<'a, N, bool>>,
// }
//
// impl<'a, const N: usize> crate::Filter<'a, N> for Loc<'a, N> {
//     fn filter(mut self, f: crate::StateFnMut<'a, N, bool>) -> Self {
//         self.filter.push(f);
//         self
//     }
// }
//
// impl<'a, const N: usize> Loc<'a, N> {
//     /// Constructor for [Detection::Sign] variant. Defaults location to [LocMethod::Bisection].
//     pub fn zero(f: StateFnMut<'a, N, f64>) -> Self {
//         Self {
//             detection: Detection::Sign(f),
//             location: LocMethod::Bisection,
//             filter: Vec::new(),
//         }
//     }
//     /// Constructor for [Detection::SignToPos] variant. Defaults location to [LocMethod::Bisection].
//     pub fn to_pos(f: StateFnMut<'a, N, f64>) -> Self {
//         Self {
//             detection: Detection::SignToPos(f),
//             location: LocMethod::Bisection,
//             filter: Vec::new(),
//         }
//     }
//     /// Constructor for [Detection::SignToNeg] variant. Defaults location to [LocMethod::Bisection].
//     pub fn to_neg(f: StateFnMut<'a, N, f64>) -> Self {
//         Self {
//             detection: Detection::SignToNeg(f),
//             location: LocMethod::Bisection,
//             filter: Vec::new(),
//         }
//     }
//     /// Constructor for [Detection::SignNeg] variant. Defaults location to [LocMethod::Bisection].
//     pub fn neg(f: StateFnMut<'a, N, f64>) -> Self {
//         Self {
//             detection: Detection::SignNeg(f),
//             location: LocMethod::Bisection,
//             filter: Vec::new(),
//         }
//     }
//     /// Constructor for [Detection::Pos] variant. Defaults location to [LocMethod::Bisection].
//     pub fn pos(f: StateFnMut<'a, N, f64>) -> Self {
//         Self {
//             detection: Detection::SignPos(f),
//             location: LocMethod::Bisection,
//             filter: Vec::new(),
//         }
//     }
//     /// Constructor for [Detection::Bool] variant. Defaults location to [LocMethod::Bisection].
//     pub fn flip(f: StateFnMut<'a, N, bool>) -> Self {
//         Self {
//             detection: Detection::Bool(f),
//             location: LocMethod::Bisection,
//             filter: Vec::new(),
//         }
//     }
//     /// Constructor for [Detection::BoolToTrue] variant. Defaults location to [LocMethod::Bisection].
//     pub fn to_true(f: StateFnMut<'a, N, bool>) -> Self {
//         Self {
//             detection: Detection::BoolToTrue(f),
//             location: LocMethod::Bisection,
//             filter: Vec::new(),
//         }
//     }
//     /// Constructor for [Detection::BoolToFalse] variant. Defaults location to [LocMethod::Bisection].
//     pub fn to_false(f: StateFnMut<'a, N, bool>) -> Self {
//         Self {
//             detection: Detection::BoolToFalse(f),
//             location: LocMethod::Bisection,
//             filter: Vec::new(),
//         }
//     }
//
//     /// Implements detection for all [Detection] variants. Returns `true` if the event is
//     /// detected between the last and current step of state.
//     pub fn detect<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> bool {
//         match &mut self.detection {
//             Detection::Sign(f) => {
//                 let curr = f.eval(state);
//                 let prev = f.eval_prev(state);
//                 curr > 0. && prev <= 0. || curr < 0. && prev >= 0.
//             }
//             Detection::SignToPos(f) => {
//                 let curr = f.eval(state);
//                 let prev = f.eval_prev(state);
//                 curr > 0. && prev <= 0.
//             }
//             Detection::SignToNeg(f) => {
//                 let curr = f.eval(state);
//                 let prev = f.eval_prev(state);
//                 curr < 0. && prev >= 0.
//             }
//             Detection::SignNeg(f) => f.eval(state) <= 0.,
//             Detection::SignPos(f) => f.eval(state) >= 0.,
//             Detection::Bool(f) => f.eval(state) != f.eval_prev(state),
//             Detection::BoolToTrue(f) => f.eval(state) && !f.eval_prev(state),
//             Detection::BoolToFalse(f) => !f.eval(state) && f.eval_prev(state),
//         }
//     }
//
//     /// Implements location methods for all [LocMethod] variants, utilizing functions provided by
//     /// [Detection]. Returns the time at which event is approximated to be located.
//     pub fn locate<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Option<f64> {
//         if !self.detect(state) || !self.filter.iter_mut().all(|f| f.eval(state)) {
//             return None;
//         } else {
//             match self.location {
//                 LocMethod::StepBegin => Some(state.t_prev),
//                 LocMethod::StepEnd => Some(state.t),
//                 LocMethod::StepMiddle => Some(0.5 * (state.t_prev + state.t)),
//                 LocMethod::Lerp => match &mut self.detection {
//                     Detection::Bool(_) | Detection::BoolToTrue(_) | Detection::BoolToFalse(_) => {
//                         Some(0.5 * (state.t_prev + state.t))
//                     }
//                     Detection::Sign(f) | Detection::SignToPos(f) | Detection::SignToNeg(f) => {
//                         let curr = f.eval(state);
//                         let prev = f.eval_prev(state);
//                         Some((curr * state.t_prev - prev * state.t) / (curr - prev))
//                     }
//                     Detection::SignNeg(f) | Detection::SignPos(f) => {
//                         let curr = f.eval(state);
//                         let prev = f.eval_prev(state);
//                         if (prev > 0.) != (curr > 0.) {
//                             Some((curr * state.t_prev - prev * state.t) / (curr - prev))
//                         } else {
//                             Some(state.t_prev)
//                         }
//                     }
//                 },
//                 LocMethod::Bisection => match &mut self.detection {
//                     Detection::Bool(f) | Detection::BoolToTrue(f) | Detection::BoolToFalse(f) => {
//                         let mut l = state.t_prev;
//                         let mut r = state.t;
//                         if f.eval_prev(state) {
//                             swap(&mut l, &mut r);
//                         }
//
//                         for _ in 0..f64::MANTISSA_DIGITS {
//                             let m = 0.5 * (l + r);
//                             match f.eval_at(state, m) {
//                                 false => l = m,
//                                 true => r = m,
//                             }
//                         }
//                         Some(f64::max(l, r))
//                     }
//
//                     Detection::Sign(f)
//                     | Detection::SignToPos(f)
//                     | Detection::SignToNeg(f)
//                     | Detection::SignPos(f)
//                     | Detection::SignNeg(f) => {
//                         let mut l = state.t_prev;
//                         let mut r = state.t;
//                         if f.eval(state) < 0. {
//                             swap(&mut l, &mut r);
//                         }
//
//                         for _ in 0..f64::MANTISSA_DIGITS {
//                             let m = 0.5 * (l + r);
//                             match f.eval_at(state, m) < 0. {
//                                 true => l = m,
//                                 false => r = m,
//                             }
//                         }
//                         Some(f64::max(l, r))
//                     }
//                 },
//
//                 LocMethod::Brent => todo!(),
//             }
//         }
//     }
// }
