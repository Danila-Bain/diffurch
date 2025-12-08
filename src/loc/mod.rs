use crate::StateFn;
use crate::StateRef;
use crate::state::EvalStateFn;
use crate::traits::RealVectorSpace;
use nalgebra::RealField;
use num::Float;

pub struct Loc<F = (), D = (), L = ()> {
    pub function: F,
    pub detection: D,
    pub location: L,
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
        pub fn $fn<T: RealField + Copy, Y: RealVectorSpace<T>, F: FnMut(&StateRef<T, Y>) -> $type>(
            f: F,
        ) -> Loc<impl EvalStateFn<T, Y, $type>, detect::$detection, locate::$location>
        where
            T: Float,
        {
            Loc {
                function: StateFn::<T, Y, $type, F, false>::new(f),
                detection: detect::$detection,
                location: locate::$location,
            }
        }
    };
}

impl Loc {
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

impl<F, D, L> Loc<F, D, L> {
    pub fn using<LL>(self, l: LL) -> Loc<F, D, LL> {
        Loc {
            function: self.function,
            detection: self.detection,
            location: l,
        }
    }
}
