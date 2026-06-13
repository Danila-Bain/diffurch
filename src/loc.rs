use crate::StateFn;
use crate::StateRef;
use crate::initial_condition::InitialCondition;
use crate::state::EvalMutState;
use crate::state::EvalState;
use crate::state::State;
use crate::traits::RealVectorSpace;
use nalgebra::RealField;
use std::marker::PhantomData;

pub trait Detect<
    T: RealField + Copy,
    P: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, P>,
>
{
    fn detect(&mut self, state: &State<T, P, S, I, IC>) -> bool;
}

pub trait Locate<
    T: RealField + Copy,
    P: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, P>,
>: Detect<T, P, S, I, IC>
{
    fn locate(&mut self, state: &State<T, P, S, I, IC>) -> T;
    fn detect_and_locate(&mut self, state: &State<T, P, S, I, IC>) -> Option<T> {
        self.detect(state).then(|| self.locate(state))
    }
}

pub struct LocatorStateFn<T, P, Output, F, Detection, Location> {
    f: F,
    detection: Detection,
    location: Location,
    _phantom: PhantomData<(T, P, Output)>,
}

impl<
    T: RealField + Copy,
    P: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, P>,
    Output,
    F: EvalState<T, P, S, I, IC, Output>,
    Detection,
    Location,
> Detect<T, P, S, I, IC> for LocatorStateFn<T, P, Output, F, Detection, Location>
where
    Detection: detection_method::DetectionMethod<Output<T, P, S, I, IC> = Output>,
{
    fn detect(&mut self, state: &State<T, P, S, I, IC>) -> bool {
        self.detection.detect(&mut self.f, state)
    }
}

impl<
    T: RealField + Copy,
    P: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, P>,
    Output,
    F: EvalState<T, P, S, I, IC, Output>,
    Detection,
    Location,
> Locate<T, P, S, I, IC> for LocatorStateFn<T, P, Output, F, Detection, Location>
where
    Location: location_method::LocationMethod<T, P, S, I, IC, F>,
    Self: Detect<T, P, S, I, IC>,
{
    fn locate(&mut self, state: &State<T, P, S, I, IC>) -> T {
        self.location.locate(&mut self.f, state)
    }
}

pub mod detection_method {

    use super::*;

    pub trait DetectionMethod {
        type Output<T, P, const S: usize, const I: usize, IC>;
        fn detect<
            T: RealField + Copy,
            P: RealVectorSpace<T>,
            const S: usize,
            const I: usize,
            IC: InitialCondition<T, P>,
            F: EvalState<T, P, S, I, IC, Self::Output<T, P, S, I, IC>>,
        >(
            &mut self,
            f: &mut F,
            state: &State<T, P, S, I, IC>,
        ) -> bool;
    }

    macro_rules! impl_detection_method(
        ($type:ty, $detect:ident, |$($curr:ident $(, $prev:ident)?)?| $body:expr) => {
            pub struct $detect;
            impl DetectionMethod for $detect {
                type Output<T, P, const S: usize, const I: usize, IC> = $type;
                fn detect<
                    T: RealField + Copy,
                    P: RealVectorSpace<T>,
                    const S: usize,
                    const I: usize,
                    IC: InitialCondition<T, P>,
                    F: EvalState<T, P, S, I, IC, Self::Output<T, P, S, I, IC>>,
                >(
                    &mut self,
                    __f: &mut F,
                    __state: &State<T, P, S, I, IC>,
                ) -> bool {
                    $(let $curr = __f.eval_curr(__state);
                    $(let $prev = __f.eval_prev(__state);)?)?
                    $body
                }
            }
        }
    );

    impl_detection_method!(T, Zero, |curr, prev| {
        curr > T::zero() && prev <= T::zero() || curr <= T::zero() && prev > T::zero()
    });
    impl_detection_method!(T, AboveZero, |curr, prev| {
        curr > T::zero() && prev <= T::zero()
    });
    impl_detection_method!(T, BelowZero, |curr, prev| {
        curr < T::zero() && prev >= T::zero()
    });
    impl_detection_method!(T, Positive, |curr| curr >= T::zero());
    impl_detection_method!(T, Negative, |curr| curr <= T::zero());
    impl_detection_method!(bool, Switch, |curr, prev| curr != prev);
    impl_detection_method!(bool, SwitchTrue, |curr, prev| curr && !prev);
    impl_detection_method!(bool, SwitchFalse, |curr, prev| !curr && prev);
    impl_detection_method!(bool, IsTrue, |curr| curr);
    impl_detection_method!(bool, IsFalse, |curr| !curr);
    impl_detection_method!(bool, Step, |/*do not confuce with `||`*/| true);
}

pub mod location_method {

    use super::*;

    pub trait LocationMethod<
        T: RealField + Copy,
        P: RealVectorSpace<T>,
        const S: usize,
        const I: usize,
        IC: InitialCondition<T, P>,
        F,
    >
    {
        fn locate(&mut self, f: &mut F, state: &State<T, P, S, I, IC>) -> T;
    }

    /// Use the previous step time as the location of event
    pub struct StepBegin;
    /// Use the current step time as the location of event
    pub struct StepEnd;
    /// Use the middle between previous and current step time as the location of event
    pub struct StepMiddle;
    /// Use the linear interpolation as an approximation for the location of event for float-valued
    /// detection functions (not supported for `bool` detection functions)
    pub struct Lerp;
    /// Use bisection method to find the location of event for float-valued detection functions. See also: [BisectionBool].
    pub struct Bisection;
    /// Use bisection method to find the location of event for bool-valued detection functions. See also: [Bisection].
    pub struct BisectionBool;
    /// Use regula falsi method to find the location of event for float-valued detection functions. See also: [Bisection]. Current implementation is not as reliable as [Bisection].
    pub struct RegulaFalsi;

    macro_rules! impl_locate(
        ($locate:ident, $(Output = $fn_output:ty,)? |$f:ident, $state:ident| $body:expr) => {
        impl<
            T: RealField + Copy,
            P: RealVectorSpace<T>,
            const S: usize,
            const I: usize,
            IC: InitialCondition<T, P>,
            F $(: EvalState<T, P, S, I, IC, $fn_output>)?,
        > LocationMethod<T,P, S, I, IC, F> for $locate
         {
            fn locate(
                &mut self,
                $f: &mut F,
                $state: &State<T, P, S, I, IC>,
            ) -> T {
                $body
            }
        }
    }
);

    impl_locate!(StepBegin, |_f, state| state.t_prev);
    impl_locate!(StepEnd, |_f, state| state.t_curr);
    impl_locate!(StepMiddle, |_f, state| {
        T::from_f64(0.5).unwrap() * (state.t_curr - state.t_prev)
    });
    impl_locate!(Lerp, Output = T, |f, state| {
        let curr = f.eval_curr(state);
        let prev = f.eval_prev(state);
        (curr * state.t_prev - prev * state.t_curr) / (curr - prev)
    });
    impl_locate!(BisectionBool, Output = bool, |f, state| {
        let mut l = state.t_prev;
        let mut r = state.t_curr;

        let mut m = T::from_f64(0.5).unwrap() * (l + r);

        // guarantee f(l) is false and f(r) is true
        if f.eval_prev(state) {
            std::mem::swap(&mut l, &mut r);
        }

        let mut w = (r - l).abs();
        let mut w_prev = T::from_f64(2.).unwrap() * w;

        while w < w_prev {
            w_prev = w;
            match f.eval_at(state, m) {
                false => l = m,
                true => r = m,
            }
            m = T::from_f64(0.5).unwrap() * (l + r);
            w = (r - l).abs();
        }
        T::max(l, r)
    });
    impl_locate!(Bisection, Output = T, |f, state| {
        let mut l = state.t_prev;
        let mut r = state.t_curr;

        let mut m = T::from_f64(0.5).unwrap() * (l + r);

        if f.eval_curr(state) < T::zero() {
            std::mem::swap(&mut l, &mut r);
        }

        let mut w = (r - l).abs();
        let mut w_prev = T::from_f64(2.).unwrap() * w;

        while w < w_prev {
            w_prev = w;
            match f.eval_at(state, m) < T::zero() {
                true => l = m,
                false => r = m,
            }
            m = T::from_f64(0.5).unwrap() * (l + r);
            w = (r - l).abs();
        }

        T::max(l, r)
    });
    impl_locate!(RegulaFalsi, Output = T, |f, state| {
        let mut l = state.t_prev;
        let mut r = state.t_curr;

        // guarantee f(l) < 0 and f(r) > 0
        if f.eval_curr(state) < T::zero() {
            std::mem::swap(&mut l, &mut r);
        }

        let mut w = (r - l).abs();
        let mut w_prev = T::from_f64(2.).unwrap() * w;

        while w < w_prev {
            w_prev = w;
            let f_l = f.eval_at(state, l);
            let f_r = f.eval_at(state, r);
            let m = (f_r * l - f_l * r) / (f_r - f_l);
            let f_m = f.eval_at(state, m);
            match f_m < T::zero() {
                false => l = m,
                true => r = m,
            }
            w = (r - l).abs();
        }
        T::max(l, r)
    });
}
pub mod periodic {

    use super::*;
    use num::Float;

    pub struct Periodic<T> {
        pub period: T,
        pub offset: T,
    }

    impl<T: Float> Periodic<T> {
        pub fn new(period: T) -> Self {
            Periodic {
                period,
                offset: T::zero(),
            }
        }

        pub fn with_offset(self, offset: T) -> Self {
            Self { offset, ..self }
        }
    }

    impl<
        T: RealField + Copy,
        Y: RealVectorSpace<T>,
        const S: usize,
        const I: usize,
        IC: InitialCondition<T, Y>,
    > Detect<T, Y, S, I, IC> for Periodic<T>
    {
        fn detect(&mut self, state: &State<T, Y, S, I, IC>) -> bool {
            let prev = ((state.t_prev - self.offset) / (self.period)).floor();
            let curr = ((state.t_curr - self.offset) / (self.period)).floor();
            prev < curr
        }
    }
    impl<
        T: RealField + Copy,
        Y: RealVectorSpace<T>,
        const S: usize,
        const I: usize,
        IC: InitialCondition<T, Y>,
    > Locate<T, Y, S, I, IC> for Periodic<T>
    {
        fn locate(&mut self, state: &State<T, Y, S, I, IC>) -> T {
            ((state.t_curr - self.offset) / self.period).floor() * self.period + self.offset
        }
    }
}

pub mod propagation {

    use crate::{state::EvalMutState, util::partition_point_linear};

    use super::*;

    pub struct Propagation;

    use impl_tools::autoimpl;
    #[autoimpl(Debug ignore self.delayed_argument where T: std::fmt::Debug)]
    pub struct Propagator<T, Delayed> {
        pub delayed_argument: Delayed,
        pub smoothing_order: usize,
        pub tracked_disco_t: T,
        pub tracked_disco_order: usize,
        pub tracked_queue_index: usize,
    }

    impl<T: RealField, Delayed> Propagator<T, Delayed> {
        pub fn new(delayed: Delayed, smoothing_order: usize) -> Self {
            Self {
                delayed_argument: delayed,
                smoothing_order,
                tracked_disco_t: T::min_value().unwrap(),
                tracked_disco_order: usize::MAX,
                tracked_queue_index: 0,
            }
        }
    }

    impl<
        T: RealField + Copy,
        P: RealVectorSpace<T>,
        const S: usize,
        const I: usize,
        IC: InitialCondition<T, P>,
        Delayed: EvalState<T, P, S, I, IC, T>,
        L,
    > Detect<T, P, S, I, IC> for LocatorStateFn<T, P, T, Propagator<T, Delayed>, Propagation, L>
    {
        fn detect(&mut self, state: &State<T, P, S, I, IC>) -> bool {
            let propagator = &mut self.f;

            let prev = propagator.delayed_argument.eval_prev(state);
            let curr = propagator.delayed_argument.eval_curr(state);

            // detect if any element in state.disco_seq lies between prev and curr
            let partition_prev = partition_point_linear(
                &state.history.disco_deque,
                propagator.tracked_queue_index,
                |&(t, _order)| t <= prev,
            );
            let partition_curr = partition_point_linear(
                &state.history.disco_deque,
                partition_prev,
                |&(t, _order)| t <= curr,
            );

            propagator.tracked_queue_index = partition_prev.min(partition_curr);

            if partition_prev != partition_curr {
                let (t_disco, t_order) = *state
                    .history
                    .disco_deque
                    .get(propagator.tracked_queue_index)
                    .unwrap();
                (propagator.tracked_disco_t, propagator.tracked_disco_order) =
                    (t_disco, t_order + propagator.smoothing_order);
                true
            } else {
                false
            }
        }
    }

    // When used by Locate trait, it will lead to location of points, where delayed argument
    // is equal to past discontinuity
    impl<
        T: RealField + Copy,
        Y: RealVectorSpace<T>,
        const S: usize,
        const I: usize,
        IC: InitialCondition<T, Y>,
        Delayed: EvalState<T, Y, S, I, IC, T>,
    > EvalState<T, Y, S, I, IC, T> for Propagator<T, Delayed>
    {
        fn eval_curr(&mut self, state: &State<T, Y, S, I, IC>) -> T {
            self.delayed_argument.eval_curr(state) - self.tracked_disco_t
        }
        fn eval_prev(&mut self, state: &State<T, Y, S, I, IC>) -> T {
            self.delayed_argument.eval_prev(state) - self.tracked_disco_t
        }
        fn eval_at(&mut self, state: &State<T, Y, S, I, IC>, t: T) -> T {
            self.delayed_argument.eval_at(state, t) - self.tracked_disco_t
        }
    }

    impl<
        T: RealField + Copy,
        P: RealVectorSpace<T>,
        const S: usize,
        const I: usize,
        IC: InitialCondition<T, P>,
        Delayed: EvalState<T, P, S, I, IC, T>,
        L,
    > EvalMutState<T, P, S, I, IC, ()>
        for LocatorStateFn<T, P, T, Propagator<T, Delayed>, Propagation, L>
    {
        fn eval_mut(&mut self, state: &mut State<T, P, S, I, IC>) -> () {
            if self.f.tracked_disco_order < state.rk.order {
                state
                    .history
                    .disco_deque
                    .push_back((state.t_curr, self.f.tracked_disco_order))
            }
        }
    }
}

pub mod filter {

    use super::*;

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
        F: EvalState<T, Y, S, I, IC, bool>,
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
        F: EvalState<T, Y, S, I, IC, bool>,
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
        F: EvalState<T, Y, S, I, IC, bool>,
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
        F: EvalState<T, Y, S, I, IC, bool>,
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
        F: EvalState<T, Y, S, I, IC, bool>,
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
        F: EvalState<T, Y, S, I, IC, bool>,
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
    >
    where
        Self: Locate<T, Y, S, I, IC> + Sized,
    {
        fn every(
            self,
            n: usize,
        ) -> FilterAfterDetection<T, Y, S, I, IC, Self, impl EvalState<T, Y, S, I, IC, bool>>
        {
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
        ) -> FilterLocated<T, Y, S, I, IC, Self, impl EvalState<T, Y, S, I, IC, bool>> {
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
        ) -> FilterLocated<T, Y, S, I, IC, Self, impl EvalState<T, Y, S, I, IC, bool>> {
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
        ) -> FilterAfterDetection<T, Y, S, I, IC, Self, impl EvalState<T, Y, S, I, IC, bool>>
        {
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
}
pub use filter::Filter;

pub mod loc_callback {

    use crate::state::EvalMutState;

    use super::*;

    pub struct LocCallback<L, C>(pub L, pub C);

    impl<L, C> From<(L, C)> for LocCallback<L, C> {
        fn from((l, c): (L, C)) -> Self {
            LocCallback(l, c)
        }
    }

    impl<
        T: RealField + Copy,
        Y: RealVectorSpace<T>,
        const S: usize,
        const I: usize,
        IC: InitialCondition<T, Y>,
        L: Locate<T, Y, S, I, IC>,
        Other,
    > Detect<T, Y, S, I, IC> for LocCallback<L, Other>
    {
        fn detect(&mut self, state: &State<T, Y, S, I, IC>) -> bool {
            self.0.detect(state)
        }
    }

    impl<
        T: RealField + Copy,
        Y: RealVectorSpace<T>,
        const S: usize,
        const I: usize,
        IC: InitialCondition<T, Y>,
        L: Locate<T, Y, S, I, IC>,
        Other,
    > Locate<T, Y, S, I, IC> for LocCallback<L, Other>
    {
        fn locate(&mut self, state: &State<T, Y, S, I, IC>) -> T {
            self.0.locate(state)
        }
    }

    impl<
        T: RealField + Copy,
        Y: RealVectorSpace<T>,
        const S: usize,
        const I: usize,
        IC: InitialCondition<T, Y>,
        Output,
        C: EvalState<T, Y, S, I, IC, Output>,
        Other,
    > EvalState<T, Y, S, I, IC, Output> for LocCallback<Other, C>
    {
        fn eval_curr(&mut self, state: &State<T, Y, S, I, IC>) -> Output {
            self.1.eval_curr(state)
        }

        fn eval_prev(&mut self, state: &State<T, Y, S, I, IC>) -> Output {
            self.1.eval_prev(state)
        }

        fn eval_at(&mut self, state: &State<T, Y, S, I, IC>, t: T) -> Output {
            self.1.eval_at(state, t)
        }
    }

    impl<
        T: RealField + Copy,
        Y: RealVectorSpace<T>,
        const S: usize,
        const I: usize,
        IC: InitialCondition<T, Y>,
        Output,
        C: EvalMutState<T, Y, S, I, IC, Output>,
        Other,
    > EvalMutState<T, Y, S, I, IC, Output> for LocCallback<Other, C>
    {
        fn eval_mut(&mut self, state: &mut State<T, Y, S, I, IC>) -> Output {
            self.1.eval_mut(state)
        }
    }
}

pub mod loc_hlist {

    use hlist2_trait_macro::TraitHList;

    use super::*;

    pub trait LocateEarliestImpl<
        T: RealField + Copy,
        Y: RealVectorSpace<T>,
        const S: usize,
        const I: usize,
        IC: InitialCondition<T, Y>,
    >
    {
        fn locate_earliest_impl(
            &mut self,
            state: &State<T, Y, S, I, IC>,
            self_index: &mut usize,
            earliest_index: &mut usize,
            earliest_time: &mut Option<T>,
        );
    }

    impl<
        T: RealField + Copy,
        Y: RealVectorSpace<T>,
        const S: usize,
        const I: usize,
        IC: InitialCondition<T, Y>,
        L: Locate<T, Y, S, I, IC>,
    > LocateEarliestImpl<T, Y, S, I, IC> for L
    {
        fn locate_earliest_impl(
            &mut self,
            state: &State<T, Y, S, I, IC>,
            self_index: &mut usize,
            earliest_index: &mut usize,
            earliest_time: &mut Option<T>,
        ) {
            if let Some(self_time) = self.detect_and_locate(state)
                && earliest_time.is_none_or(|t| self_time < t)
            {
                *earliest_index = *self_index;
                *earliest_time = Some(self_time);
            }
            *self_index += 1;
        }
    }

    TraitHList! {
        pub HListLocateEarliestImpl for
            trait LocateEarliestImpl<
                T: RealField + Copy,
                Y: RealVectorSpace<T>,
                const S: usize,
                const I: usize,
                IC: InitialCondition<T, Y>,
            > {
            fn locate_earliest_impl(
                &mut self,
                state: &State<T, Y, S, I, IC>,
                self_index: &mut usize,
                earliest_index: &mut usize,
                earliest_time: &mut Option<T>,
            );
        }
    }

    pub trait HListLocateEarliest<
        T: RealField + Copy,
        Y: RealVectorSpace<T>,
        const S: usize,
        const I: usize,
        IC: InitialCondition<T, Y>,
    >
    {
        fn locate_earliest(&mut self, state: &State<T, Y, S, I, IC>) -> Option<(usize, T)>;
    }

    impl<
        T: RealField + Copy,
        Y: RealVectorSpace<T>,
        const S: usize,
        const I: usize,
        IC: InitialCondition<T, Y>,
        U: HListLocateEarliestImpl<T, Y, S, I, IC>,
    > HListLocateEarliest<T, Y, S, I, IC> for U
    {
        fn locate_earliest(&mut self, state: &State<T, Y, S, I, IC>) -> Option<(usize, T)> {
            let mut index = 0;
            let mut earliest_time = None;
            self.locate_earliest_impl(state, &mut 0, &mut index, &mut earliest_time);
            Some((index, earliest_time?))
        }
    }
}

macro_rules! loc_constructor {
    ($fn:ident, $type:ty, $detection:ident, $location:ident) => {
        pub fn $fn<
            const S: usize,
            const I: usize,
            IC: InitialCondition<T, P>,
            F: FnMut(&StateRef<T, P, S, I, IC>) -> $type,
        >(
            f: F,
        ) -> LocatorStateFn<
            T,
            P,
            $type,
            StateFn<T, P, $type, F, false>,
            detection_method::$detection,
            location_method::$location,
        > {
            LocatorStateFn {
                f: StateFn::new(f),
                detection: detection_method::$detection,
                location: location_method::$location,
                _phantom: PhantomData,
            }
        }
    };
}

pub struct Locator<T, P>(PhantomData<(T, P)>);
impl<T: RealField + Copy, P: RealVectorSpace<T>> Locator<T, P> {
    loc_constructor! {zero,         T,    Zero,        Bisection}
    loc_constructor! {above_zero,   T,    AboveZero,   Bisection}
    loc_constructor! {below_zero,   T,    BelowZero,   Bisection}
    loc_constructor! {positive,     T,    Positive,    StepBegin}
    loc_constructor! {negative,     T,    Negative,    StepBegin}
    loc_constructor! {switch,       bool, Switch,      BisectionBool}
    loc_constructor! {switch_true,  bool, SwitchTrue,  BisectionBool}
    loc_constructor! {switch_false, bool, SwitchFalse, BisectionBool}
    loc_constructor! {is_true,      bool, IsTrue,      BisectionBool}
    loc_constructor! {is_false,     bool, IsFalse,     BisectionBool}

    pub fn step<const S: usize, const I: usize, IC: InitialCondition<T, P>>() -> LocatorStateFn<
        T,
        P,
        bool,
        StateFn<T, P, bool, impl FnMut(&StateRef<T, P, S, I, IC>) -> bool, false>,
        detection_method::Step,
        location_method::StepEnd,
    > {
        LocatorStateFn {
            f: StateFn::new(|_| true),
            detection: detection_method::Step,
            location: location_method::StepEnd,
            _phantom: PhantomData,
        }
    }

    pub fn propagated_discontinuity<
        const S: usize,
        const I: usize,
        IC: InitialCondition<T, P>,
        F: FnMut(&StateRef<T, P, S, I, IC>) -> T,
    >(
        delayed: F,
        smoothing_order: usize,
    ) -> LocatorStateFn<
        T,
        P,
        T,
        propagation::Propagator<T, StateFn<T, P, T, F>>,
        propagation::Propagation,
        location_method::Bisection,
    > {
        LocatorStateFn {
            f: propagation::Propagator::new(
                StateFn::<T, P, T, F, false>::new(delayed),
                smoothing_order,
            ),
            detection: propagation::Propagation,
            location: location_method::Bisection,
            _phantom: std::marker::PhantomData,
        }
    }
}

pub struct DedupLocF<T, L> {
    pub last_call: Option<T>,
    pub loc_f: L,
}

impl<
    T: RealField + Copy,
    P: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, P>,
    L: Detect<T, P, S, I, IC>,
> Detect<T, P, S, I, IC> for DedupLocF<T, L>
{
    fn detect(&mut self, state: &State<T, P, S, I, IC>) -> bool {
        self.last_call
            .is_none_or(|last_call| state.t_prev > last_call)
            && self.detect(state)
    }
}
impl<
    T: RealField + Copy,
    P: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, P>,
    L: Locate<T, P, S, I, IC>,
> Locate<T, P, S, I, IC> for DedupLocF<T, L>
{
    fn locate(&mut self, state: &State<T, P, S, I, IC>) -> T {
        self.loc_f.locate(state)
    }
    fn detect_and_locate(&mut self, state: &State<T, P, S, I, IC>) -> Option<T> {
        if self
            .last_call
            .is_none_or(|last_call| state.t_prev > last_call)
        {
            self.loc_f.detect_and_locate(state)
        } else {
            None
        }
    }
}
impl<
    T: RealField + Copy,
    P: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, P>,
    Output,
    L: EvalMutState<T, P, S, I, IC, Output>,
> EvalMutState<T, P, S, I, IC, Output> for DedupLocF<T, L>
{
    fn eval_mut(&mut self, state: &mut State<T, P, S, I, IC>) -> Output {
        self.last_call = Some(state.t_curr);
        self.loc_f.eval_mut(state)
    }
}
impl<
    T: RealField + Copy,
    P: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, P>,
    Output,
    L: EvalState<T, P, S, I, IC, Output>,
> EvalState<T, P, S, I, IC, Output> for DedupLocF<T, L>
{
    fn eval_curr(&mut self, state: &State<T, P, S, I, IC>) -> Output {
        self.last_call = Some(state.t_curr);
        self.loc_f.eval_curr(state)
    }

    fn eval_prev(&mut self, state: &State<T, P, S, I, IC>) -> Output {
        self.last_call = Some(state.t_prev);
        self.loc_f.eval_prev(state)
    }

    fn eval_at(&mut self, state: &State<T, P, S, I, IC>, t: T) -> Output {
        self.last_call = Some(t);
        self.loc_f.eval_at(state, t)
    }
}
