//! Detection and location of events

/// Struct that conatins detection + location methods.
pub struct Loc<F = (), D = (), L = ()>(pub F, pub D, pub L);

/// Detection of events
pub mod detection;
/// Location of events
pub mod location;
/// Delay-propagation of discontinuities
pub mod propagation;

pub mod periodic;

pub use detection::Detect;
pub use location::Locate;
pub use propagation::*;
pub use periodic::*;

/// Convenience macro for change-of-sign event locator
#[macro_export]
macro_rules! loc_sign {
    ($($expr:tt)*) => {
        $crate::Loc($crate::state_fn!($($expr)*),
                    $crate::loc::detection::Sign,
                    $crate::loc::location::Bisection)
    };
}

/// Convenience macro for change-of-bool event locator
#[macro_export]
macro_rules! loc_bool {
    ($($expr:tt)*) => {
        $crate::Loc($crate::state_fn!($($expr)*),
                    $crate::loc::detection::Bool,
                    $crate::loc::location::BisectionBool)
    };
}

impl Loc {
    /// Constructor for Loc, that defaults detection and location fields to `()`
    pub fn new<F>(f: F) -> Loc<F> {
        Loc(f, (), ())
    }
}

impl<F, D, L> Loc<F, D, L> {
    /// Self-consuming setter of detection method [detection::Sign]
    pub fn sign(self) -> Loc<F, detection::Sign, L> {
        Loc(self.0, detection::Sign, self.2)
    }
    /// Self-consuming setter of detection method [detection::Pos]
    pub fn pos(self) -> Loc<F, detection::Pos, L> {
        Loc(self.0, detection::Pos, self.2)
    }
    /// Self-consuming setter of detection method [detection::Neg]
    pub fn neg(self) -> Loc<F, detection::Neg, L> {
        Loc(self.0, detection::Neg, self.2)
    }
    /// Self-consuming setter of detection method [detection::WhilePos]
    pub fn while_pos(self) -> Loc<F, detection::WhilePos, L> {
        Loc(self.0, detection::WhilePos, self.2)
    }
    /// Self-consuming setter of detection method [detection::WhileNeg]
    pub fn while_neg(self) -> Loc<F, detection::WhileNeg, L> {
        Loc(self.0, detection::WhileNeg, self.2)
    }
    /// Self-consuming setter of detection method [detection::Bool]
    pub fn bool(self) -> Loc<F, detection::Bool, L> {
        Loc(self.0, detection::Bool, self.2)
    }
    /// Self-consuming setter of detection method [detection::True]
    pub fn true_(self) -> Loc<F, detection::True, L> {
        Loc(self.0, detection::True, self.2)
    }
    /// Self-consuming setter of detection method [detection::False]
    pub fn false_(self) -> Loc<F, detection::False, L> {
        Loc(self.0, detection::False, self.2)
    }
    /// Self-consuming setter of detection method [detection::WhileTrue]
    pub fn while_true(self) -> Loc<F, detection::WhileTrue, L> {
        Loc(self.0, detection::WhileTrue, self.2)
    }
    /// Self-consuming setter of detection method [detection::WhileFalse]
    pub fn while_false(self) -> Loc<F, detection::WhileFalse, L> {
        Loc(self.0, detection::WhileFalse, self.2)
    }

    /// Self-consuming setter of location method [location::StepBegin]
    pub fn step_begin(self) -> Loc<F, D, location::StepBegin> {
        Loc(self.0, self.1, location::StepBegin)
    }
    /// Self-consuming setter of location method [location::StepEnd]
    pub fn step_end(self) -> Loc<F, D, location::StepEnd> {
        Loc(self.0, self.1, location::StepEnd)
    }
    /// Self-consuming setter of location method [location::StepHalf]
    pub fn step_half(self) -> Loc<F, D, location::StepHalf> {
        Loc(self.0, self.1, location::StepHalf)
    }
    /// Self-consuming setter of location method [location::Lerp]
    pub fn lerp(self) -> Loc<F, D, location::Lerp> {
        Loc(self.0, self.1, location::Lerp)
    }
    /// Self-consuming setter of location method [location::Bisection]
    pub fn bisection(self) -> Loc<F, D, location::Bisection> {
        Loc(self.0, self.1, location::Bisection)
    }
    /// Self-consuming setter of location method [location::BisectionBool]
    pub fn bisection_bool(self) -> Loc<F, D, location::BisectionBool> {
        Loc(self.0, self.1, location::BisectionBool)
    }
    /// Self-consuming setter of location method [location::RegulaFalsi]
    pub fn regula_falsi(self) -> Loc<F, D, location::RegulaFalsi> {
        Loc(self.0, self.1, location::RegulaFalsi)
    }
}
