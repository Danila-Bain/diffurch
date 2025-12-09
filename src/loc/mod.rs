use crate::StateFn;
use crate::StateRef;
use crate::initial_condition::InitialCondition;
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
