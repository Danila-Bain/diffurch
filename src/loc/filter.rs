use nalgebra::RealField;

use crate::{
    StateFn, StateRef,
    initial_condition::InitialCondition,
    loc::{detect::Detect, locate::Locate},
    state::EvalStateFn,
    traits::RealVectorSpace,
};

// FilterBeforeDetection
// FilterAfterDetection
// FilterLocated

pub struct FilterAfterDetection<T, Y, const S: usize, const I: usize, IC, L, F> {
    pub loc: L,
    pub filter: F,
    pub _state: std::marker::PhantomData<fn(T, Y, [(); S], [(); I], IC)>,
}

impl<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
    L,
    F,
> Detect<T, Y, S, I, IC> for FilterAfterDetection<T, Y, S, I, IC, L, F>
where
    L: Detect<T, Y, S, I, IC>,
    F: EvalStateFn<T, Y, S, I, IC, bool>,
{
    fn detect(&mut self, state: &crate::state::State<T, Y, S, I, IC>) -> bool {
        self.loc.detect(state) && self.filter.eval_curr(state)
    }
}

impl<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
    L,
    F,
> Locate<T, Y, S, I, IC> for FilterAfterDetection<T, Y, S, I, IC, L, F>
where
    L: Locate<T, Y, S, I, IC>,
    F: EvalStateFn<T, Y, S, I, IC, bool>,
{
    fn locate(&mut self, state: &crate::state::State<T, Y, S, I, IC>) -> T {
        self.loc.locate(state)
    }
}

pub struct FilterBeforeDetection<T, Y, const S: usize, const I: usize, IC, L, F> {
    pub loc: L,
    pub filter: F,
    pub _state: std::marker::PhantomData<fn(T, Y, [(); S], [(); I], IC)>,
}

impl<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
    L,
    F,
> Detect<T, Y, S, I, IC> for FilterBeforeDetection<T, Y, S, I, IC, L, F>
where
    L: Detect<T, Y, S, I, IC>,
    F: EvalStateFn<T, Y, S, I, IC, bool>,
{
    fn detect(&mut self, state: &crate::state::State<T, Y, S, I, IC>) -> bool {
        self.filter.eval_curr(state) && self.loc.detect(state)
    }
}

impl<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
    L,
    F,
> Locate<T, Y, S, I, IC> for FilterBeforeDetection<T, Y, S, I, IC, L, F>
where
    L: Locate<T, Y, S, I, IC>,
    F: EvalStateFn<T, Y, S, I, IC, bool>,
{
    fn locate(&mut self, state: &crate::state::State<T, Y, S, I, IC>) -> T {
        self.loc.locate(state)
    }
}

pub struct FilterLocated<T, Y, const S: usize, const I: usize, IC, L, F> {
    pub loc: L,
    pub filter: F,
    pub _state: std::marker::PhantomData<fn(T, Y, [(); S], [(); I], IC)>,
}

impl<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
    L,
    F,
> Detect<T, Y, S, I, IC> for FilterLocated<T, Y, S, I, IC, L, F>
where
    L: Locate<T, Y, S, I, IC>,
    F: EvalStateFn<T, Y, S, I, IC, bool>,
{
    fn detect(&mut self, state: &crate::state::State<T, Y, S, I, IC>) -> bool {
        if let Some(t) = self.loc.detect_and_locate(state)
            && self.filter.eval_at(state, t)
        {
            true
        } else {
            false
        }
    }
}

impl<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
    L: Locate<T, Y, S, I, IC>,
    F: EvalStateFn<T, Y, S, I, IC, bool>,
> Locate<T, Y, S, I, IC> for FilterLocated<T, Y, S, I, IC, L, F>
{
    fn locate(&mut self, state: &crate::state::State<T, Y, S, I, IC>) -> T {
        self.loc.locate(state)
    }
    fn detect_and_locate(&mut self, state: &crate::state::State<T, Y, S, I, IC>) -> Option<T> {
        if let Some(t) = self.loc.detect_and_locate(state)
            && self.filter.eval_at(state, t)
        {
            Some(t)
        } else {
            None
        }
    }
}

pub trait Filter<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
> where
    Self: Locate<T, Y, S, I, IC> + Sized,
{
    fn every(
        self,
        n: usize,
    ) -> FilterAfterDetection<T, Y, S, I, IC, Self, impl EvalStateFn<T, Y, S, I, IC, bool>> {
        let mut i = n - 1;
        FilterAfterDetection {
            loc: self,
            filter: StateFn::new(move |_s| {
                i += 1;
                if i >= n {
                    i = 0;
                    true
                } else {
                    false
                }
            }),
            _state: std::marker::PhantomData,
        }
    }

    fn separated_by(
        self,
        diff: T,
    ) -> FilterLocated<T, Y, S, I, IC, Self, impl EvalStateFn<T, Y, S, I, IC, bool>> {
        let mut prev = T::min_value().expect("Numerical type does not have mininimum value");
        FilterLocated {
            loc: self,
            filter: StateFn::new(move |&StateRef::<T, Y, S, I, IC> { t, .. }| {
                if t >= prev + diff {
                    prev = t;
                    true
                } else {
                    false
                }
            }),
            _state: std::marker::PhantomData,
        }
    }

    fn in_interval(
        self,
        interval: impl std::ops::RangeBounds<T>,
    ) -> FilterLocated<T, Y, S, I, IC, Self, impl EvalStateFn<T, Y, S, I, IC, bool>> {
        FilterLocated {
            loc: self,
            filter: StateFn::new(move |&StateRef::<T, Y, S, I, IC> { t, .. }| {
                interval.contains(&t)
            }),
            _state: std::marker::PhantomData,
        }
    }
    fn on_times(
        self,
        iter: impl IntoIterator<Item = usize>,
    ) -> FilterAfterDetection<T, Y, S, I, IC, Self, impl EvalStateFn<T, Y, S, I, IC, bool>> {
        let mut i: usize = 0;
        let mut iter = iter.into_iter().peekable();
        FilterAfterDetection {
            loc: self,
            filter: StateFn::new(move |_s| {
                if let Some(next) = iter.peek()
                    && i == *next
                {
                    iter.next();
                    i += 1;
                    true
                } else {
                    i += 1;
                    false
                }
            }),
            _state: std::marker::PhantomData,
        }
    }
}

impl<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
    L: Locate<T, Y, S, I, IC>,
> Filter<T, Y, S, I, IC> for L
{
}
