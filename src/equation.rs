use crate::state::{State, FromState};

pub struct Equation<const N: usize = 1, RHS = (), Events = ()> {
    pub rhs: RHS,
    pub events: Events,
    pub max_delay: f64,
}

impl Equation {
    pub fn new<const N: usize, Args, F>(rhs: F) -> Equation<N, F, ()>
    where
        // for<'a> &'a crate::state::State<N, 1, fn(f64) -> [f64; N]>: crate::state::StateInto<Args>,
        F: Fn<Args, Output = [f64; N]>,
        Args: std::marker::Tuple + for<'a> FromState<&'a State<N, 1, fn(f64) -> [f64; N]>>,
    {
        Equation::<N, F, ()> { rhs, events: (), max_delay: f64::NAN }
    }

    // ordinary differential equation
    pub fn ode<const N: usize, F>(rhs: F) -> Equation<N, F, ()>
    where
        F: Fn<([f64; N],), Output = [f64; N]>,
    {
        Equation::<N, F, ()> { rhs, events: (), max_delay: 0. }
    }

    pub fn ode2<const N: usize, F>(rhs: F) -> Equation<N, F, ()>
    where
        F: Fn(f64, [f64; N]) -> [f64; N],
    {
        Equation::<N, F, ()> { rhs, events: (), max_delay: 0. }
    }

    pub fn dde<const N: usize, F, X>(rhs: F) -> Equation<N, F, ()>
    where
        F: Fn(f64, [f64; N], [X; N]) -> [f64; N],
        X: Fn(f64) -> f64
    {
        Equation::<N, F, ()> { rhs, events: () , max_delay: f64::NAN }
    }

    pub fn max_delay(self, value: f64) -> Self {
        Self {
            rhs: self.rhs,
            events: self.events,
            max_delay: value,
        }
    }
}
