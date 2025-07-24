use crate::State;

use super::{Detect, Locate};

/// Periodically located event.
///
/// # Example
///
/// ```rust
/// #![feature(generic_const_exprs)]
/// use diffurch::*;
///
/// let mut ts = vec![];
///
/// Solver::new()
///     .equation(state_fn!(|| [0.]))
///     .initial([0.])
///     .interval(-10. ..10.)
///     .on(Periodic { period: 3., offset: 1. }, event!(|t| ts.push(t)))
///     .run();
///
/// assert_eq!(ts, vec![-8., -5., -2., 1., 4., 7., 10.])
/// ```
pub struct Periodic {
    pub period: f64,
    pub offset: f64,
}

impl<const N: usize> Detect<N> for Periodic {
    fn detect(&mut self, state: &impl State<N>) -> bool {
        (state.t_prev() - self.offset).div_euclid(self.period)
            < (state.t() - self.offset).div_euclid(self.period)
    }
}

impl<const N: usize> Locate<N> for Periodic {
    fn locate(&mut self, state: &impl State<N>) -> Option<f64> {
        self.detect(state).then_some(
            state.t_prev() - (state.t_prev() - self.offset).rem_euclid(self.period) + self.period,
        )
    }
}
