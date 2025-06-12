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
    fn detect<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> bool
    where
        [(); S * (S - 1) / 2]:;
}
pub trait Locate<const N: usize> {
    fn locate<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Option<f64>
    where
        [(); S * (S - 1) / 2]:;
}

pub mod detection {
    use super::*;

    pub struct Sign<const N: usize, F: StateFnMut<N, f64>>(pub F);
    impl<const N: usize, F: StateFnMut<N, f64>> Detect<N> for Sign<N, F> {
        fn detect<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> bool
        where
            [(); S * (S - 1) / 2]:,
        {
            let curr = self.0.eval(state);
            let prev = self.0.eval_prev(state);
            curr > 0. && prev <= 0. || curr < 0. && prev >= 0.
        }
    }
    impl_deref!(Sign);

    pub struct Pos<const N: usize, F: StateFnMut<N, f64>>(pub F);
    impl<const N: usize, F: StateFnMut<N, f64>> Detect<N> for Pos<N, F> {
        fn detect<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> bool
        where
            [(); S * (S - 1) / 2]:,
        {
            let curr = self.0.eval(state);
            let prev = self.0.eval_prev(state);
            curr > 0. && prev <= 0.
        }
    }
    impl_deref!(Pos);

    pub struct Neg<const N: usize, F: StateFnMut<N, f64>>(pub F);
    impl<const N: usize, F: StateFnMut<N, f64>> Detect<N> for Neg<N, F> {
        fn detect<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> bool
        where
            [(); S * (S - 1) / 2]:,
        {
            let curr = self.0.eval(state);
            let prev = self.0.eval_prev(state);
            curr < 0. && prev >= 0.
        }
    }
    impl_deref!(Neg);

    pub struct WhilePos<const N: usize, F: StateFnMut<N, f64>>(pub F);
    impl<const N: usize, F: StateFnMut<N, f64>> Detect<N> for WhilePos<N, F> {
        fn detect<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> bool
        where
            [(); S * (S - 1) / 2]:,
        {
            self.0.eval(state) >= 0.
        }
    }
    impl_deref!(WhilePos);

    pub struct WhileNeg<const N: usize, F: StateFnMut<N, f64>>(pub F);
    impl<const N: usize, F: StateFnMut<N, f64>> Detect<N> for WhileNeg<N, F> {
        fn detect<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> bool
        where
            [(); S * (S - 1) / 2]:,
        {
            self.0.eval(state) <= 0.
        }
    }
    impl_deref!(WhileNeg);

    pub struct Bool<const N: usize, F: StateFnMut<N, bool>>(pub F);
    impl<const N: usize, F: StateFnMut<N, bool>> Detect<N> for Bool<N, F> {
        fn detect<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> bool
        where
            [(); S * (S - 1) / 2]:,
        {
            self.0.eval(state) != self.0.eval_prev(state)
        }
    }
    impl_deref!(Bool, bool);

    pub struct True<const N: usize, F: StateFnMut<N, bool>>(pub F);
    impl<const N: usize, F: StateFnMut<N, bool>> Detect<N> for True<N, F> {
        fn detect<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> bool where [(); S * (S - 1) / 2]: {
            self.0.eval(state) && !self.0.eval_prev(state)
        }
    }
    impl_deref!(True, bool);

    pub struct False<const N: usize, F: StateFnMut<N, bool>>(pub F);
    impl<const N: usize, F: StateFnMut<N, bool>> Detect<N> for False<N, F> {
        fn detect<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> bool where [(); S * (S - 1) / 2]: {
            !self.0.eval(state) && self.0.eval_prev(state)
        }
    }
    impl_deref!(False, bool);

    pub struct WhileTrue<const N: usize, F: StateFnMut<N, bool>>(pub F);
    impl<const N: usize, F: StateFnMut<N, bool>> Detect<N> for WhileTrue<N, F> {
        fn detect<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> bool where [(); S * (S - 1) / 2]: {
            self.0.eval(state)
        }
    }
    impl_deref!(WhileTrue, bool);

    pub struct WhileFalse<const N: usize, F: StateFnMut<N, bool>>(pub F);
    impl<const N: usize, F: StateFnMut<N, bool>> Detect<N> for WhileFalse<N, F> {
        fn detect<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> bool where [(); S * (S - 1) / 2]: {
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
    fn locate<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Option<f64> where [(); S * (S - 1) / 2]: {
        self.0.detect(state).then_some(state.t_prev)
    }
}
impl<const N: usize, D: Detect<N>> Locate<N> for Loc<D, StepEnd> {
    fn locate<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Option<f64> where [(); S * (S - 1) / 2]: {
        self.0.detect(state).then_some(state.t)
    }
}
impl<const N: usize, D: Detect<N>> Locate<N> for Loc<D, HalfStep> {
    fn locate<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Option<f64> where [(); S * (S - 1) / 2]: {
        self.0.detect(state).then(|| 0.5 * (state.t_prev + state.t))
    }
}
impl<const N: usize, D> Locate<N> for Loc<D, Lerp>
where
    D: Detect<N> + DerefMut,
    <D as Deref>::Target: StateFnMut<N, f64>,
{
    fn locate<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Option<f64>
    where
        [(); S * (S - 1) / 2]:,
    {
        self.0.detect(state).then(|| {
            let curr = self.0.eval(state);
            let prev = self.0.eval_prev(state);
            (curr * state.t_prev - prev * state.t) / (curr - prev)
        })
    }
}

impl<const N: usize, D> Locate<N> for Loc<D, BisectionBool>
where
    D: Detect<N> + DerefMut,
    <D as Deref>::Target: StateFnMut<N, bool>,
{
    fn locate<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Option<f64>
    where
        [(); S * (S - 1) / 2]:,
    {
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

impl<const N: usize, D> Locate<N> for Loc<D, RegulaFalsi>
where
    D: Detect<N> + DerefMut,
    <D as Deref>::Target: StateFnMut<N, f64>,
{
    fn locate<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Option<f64>
    where
        [(); S * (S - 1) / 2]:,
    {
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

impl<const N: usize, D> Locate<N> for Loc<D, Bisection>
where
    D: Detect<N> + DerefMut,
    <D as Deref>::Target: StateFnMut<N, f64>,
{
    fn locate<'b, const S: usize>(&mut self, state: &'b State<'b, N, S>) -> Option<f64>
    where
        [(); S * (S - 1) / 2]:,
    {
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
