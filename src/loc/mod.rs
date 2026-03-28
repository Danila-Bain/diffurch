use crate::StateFn;
use crate::StateRef;
use crate::initial_condition::InitialCondition;
use crate::loc::propagation::Propagation;
use crate::loc::propagation::Propagator;
use crate::traits::RealVectorSpace;
use nalgebra::RealField;

pub struct Loc<T, Y, const S: usize, const I: usize, IC, F = (), D = (), L = ()> {
    pub function: F,
    pub detection: D,
    pub location: L,
    _state: std::marker::PhantomData<fn(T, Y, [(); S], [(); I], IC)>,
}

/// Detection of events
pub mod detect;
/// Location of events
pub mod locate;
/// Periodic events
pub mod periodic;
/// Delay-propagation of discontinuities
pub mod propagation;

pub mod filter;
pub use filter::Filter;

pub mod loc_callback;
pub mod loc_hlist;

macro_rules! loc_constructor {
    ($fn:ident, $type:ty, $detection:ident, $location:ident) => {
        pub fn $fn<F: FnMut(&StateRef<T, Y, S, I, IC>) -> $type>(
            f: F,
        ) -> Loc<
            T,
            Y,
            S,
            I,
            IC,
            StateFn<T, Y, $type, F, false>,
            detect::$detection,
            locate::$location,
        > {
            Loc {
                function: StateFn::<T, Y, $type, F, false>::new(f),
                detection: detect::$detection,
                location: locate::$location,
                _state: std::marker::PhantomData,
            }
        }
    };
}

impl<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
> Loc<T, Y, S, I, IC>
{
    pub fn all() -> Loc<T, Y, S, I, IC, (), detect::All, locate::StepEnd> {
        Loc {
            function: (),
            detection: detect::All,
            location: locate::StepEnd,
            _state: std::marker::PhantomData,
        }
    }
    loc_constructor! {zero,         T,    Zero,        Bisection}
    loc_constructor! {above_zero,   T,    AboveZero,   Bisection}
    loc_constructor! {below_zero,   T,    BelowZero,   Bisection}
    loc_constructor! {positive,     T,    Positive,    StepBegin}
    loc_constructor! {negative,     T,    Negative,    StepBegin}
    loc_constructor! {switch,       bool, Switch,      Bisection}
    loc_constructor! {switch_true,  bool, SwitchTrue,  Bisection}
    loc_constructor! {switch_false, bool, SwitchFalse, Bisection}
    loc_constructor! {is_true,      bool, IsTrue,      StepBegin}
    loc_constructor! {is_false,     bool, IsFalse,     StepBegin}

    pub fn propagated_discontinuity<F: FnMut(&StateRef<T, Y, S, I, IC>) -> T>(
        delayed: F,
        smoothing_order: usize,
    ) -> Loc<
        T,
        Y,
        S,
        I,
        IC,
        Propagator<T, StateFn<T, Y, T, F, false>>,
        Propagation,
        locate::Bisection,
    > {
        Loc {
            function: Propagator::new(StateFn::<T, Y, T, F, false>::new(delayed), smoothing_order),
            detection: Propagation,
            location: locate::Bisection,
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
    F,
    D,
    L,
> Loc<T, Y, S, I, IC, F, D, L>
{
    pub fn using<LL>(self, l: LL) -> Loc<T, Y, S, I, IC, F, D, LL> {
        Loc {
            function: self.function,
            detection: self.detection,
            location: l,
            _state: std::marker::PhantomData,
        }
    }
}

pub trait LocMaker {
    type LocOutput<T, Y, const S: usize, const I: usize, IC, F>;
    type FunctionOutput<T>;

    fn make<
        T: RealField + Copy,
        Y: RealVectorSpace<T>,
        const S: usize,
        const I: usize,
        IC: InitialCondition<T, Y>,
        F: FnMut(&StateRef<T, Y, S, I, IC>) -> Self::FunctionOutput<T>,
    >(
        f: F,
    ) -> Self::LocOutput<T, Y, S, I, IC, F>;
}

impl LocMaker for detect::All {
    type LocOutput<T, Y, const S: usize, const I: usize, IC, F> =
        Loc<T, Y, S, I, IC, (), detect::All, locate::StepEnd>;

    type FunctionOutput<T> = ();

    fn make<
        T: RealField + Copy,
        Y: RealVectorSpace<T>,
        const S: usize,
        const I: usize,
        IC: InitialCondition<T, Y>,
        F: FnMut(&StateRef<T, Y, S, I, IC>) -> Self::FunctionOutput<T>,
    >(
        _: F,
    ) -> Self::LocOutput<T, Y, S, I, IC, F> {
        Loc::all()
    }
}


impl LocMaker for detect::Zero {
    type LocOutput<T, Y, const S: usize, const I: usize, IC, F> =
        Loc<T, Y, S, I, IC, StateFn<T, Y, T, F>, detect::Zero, locate::Bisection>;
    type FunctionOutput<T> = T;

    fn make<
        T: RealField + Copy,
        Y: RealVectorSpace<T>,
        const S: usize,
        const I: usize,
        IC: InitialCondition<T, Y>,
        F: FnMut(&StateRef<T, Y, S, I, IC>) -> Self::FunctionOutput<T>,
    >(
        f: F,
    ) -> Self::LocOutput<T, Y, S, I, IC, F> {
        Loc::zero(f)
    }
}


impl LocMaker for detect::BelowZero {
    type LocOutput<T, Y, const S: usize, const I: usize, IC, F> =
        Loc<T, Y, S, I, IC, StateFn<T, Y, T, F>, detect::BelowZero, locate::Bisection>;
    type FunctionOutput<T> = T;

    fn make<
        T: RealField + Copy,
        Y: RealVectorSpace<T>,
        const S: usize,
        const I: usize,
        IC: InitialCondition<T, Y>,
        F: FnMut(&StateRef<T, Y, S, I, IC>) -> Self::FunctionOutput<T>,
    >(
        f: F,
    ) -> Self::LocOutput<T, Y, S, I, IC, F> {
        Loc::below_zero(f)
    }
}


impl LocMaker for detect::AboveZero {
    type LocOutput<T, Y, const S: usize, const I: usize, IC, F> =
        Loc<T, Y, S, I, IC, StateFn<T, Y, T, F>, detect::AboveZero, locate::Bisection>;
    type FunctionOutput<T> = T;

    fn make<
        T: RealField + Copy,
        Y: RealVectorSpace<T>,
        const S: usize,
        const I: usize,
        IC: InitialCondition<T, Y>,
        F: FnMut(&StateRef<T, Y, S, I, IC>) -> Self::FunctionOutput<T>,
    >(
        f: F,
    ) -> Self::LocOutput<T, Y, S, I, IC, F> {
        Loc::above_zero(f)
    }
}
