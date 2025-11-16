use crate::StateFn;
use crate::StateRef;
use crate::state::EvalStateFn;
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
// /// Delay-propagation of discontinuities
// pub mod propagation;
//
// pub mod periodic;
//
// pub use detection::Detect;
// pub use location::Locate;
// pub use propagation::*;
// pub use periodic::*;
//
// Loc::zero(|&StateRef{x, ..}| x[0] - x[1]).with(loc::Bisection)
// Loc::zero_above(...

macro_rules! loc_constructor {
    ($fn:ident, $type:ty, $detection:ident, $location:ident) => {
        pub fn $fn<const N: usize, T, Output, F: FnMut(&StateRef<T, N>) -> $type>(
            f: F,
        ) -> Loc<impl EvalStateFn<N, T, $type>, detect::$detection, locate::$location>
        where
            T: Float + std::fmt::Debug,
        {
            Loc {
                function: StateFn::<N, T, $type, F, false>::new(f),
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
    loc_constructor! {positive,     T,    Positive,    Bisection}
    loc_constructor! {negative,     T,    Negative,    Bisection}
    loc_constructor! {switch,       bool, Switch,      Bisection}
    loc_constructor! {switch_true,  bool, SwitchTrue,  Bisection}
    loc_constructor! {switch_false, bool, SwitchFalse, Bisection}
    loc_constructor! {is_true,      bool, IsTrue,      Bisection}
    loc_constructor! {is_false,     bool, IsFalse,     Bisection}
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
