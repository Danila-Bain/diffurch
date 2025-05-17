//! Defines [crate::Filter] trait for filtering callbacks in [crate::Event] and [crate::Loc].

use crate::StateFn;

/// Trait manages adding filtering functions to a vector field of a class, such as
/// [crate::Event] and [crate::Loc].
pub trait Filter<'a, const N: usize>
where
    Self: Sized + 'a,
{
    /// push a new [StateFn] which returns `bool` to `self` and return `self` for chained syntax.
    fn filter(self, f: StateFn<'a, N, bool>) -> Self;

    /// push a new [StateFn::Constant], constructed from a given closure
    fn filter_constant(self, f: impl 'a + FnMut() -> bool) -> Self {
        self.filter(StateFn::Constant(Box::new(f)))
    }
    /// push a new [StateFn::Time], constructed from a given closure
    fn filter_time(self, f: impl 'a + FnMut(f64) -> bool) -> Self {
        self.filter(StateFn::Time(Box::new(f)))
    }
    /// push a new [StateFn::ODE], constructed from a given closure
    fn filter_ode(self, f: impl 'a + FnMut([f64; N]) -> bool) -> Self {
        self.filter(StateFn::ODE(Box::new(f)))
    }
    /// push a new [StateFn::ODE2], constructed from a given closure
    fn filter_ode2(self, f: impl 'a + FnMut(f64, [f64; N]) -> bool) -> Self {
        self.filter(StateFn::ODE2(Box::new(f)))
    }

    fn every(self, n: usize) -> Self {
        let mut counter = n - 1;
        self.filter_constant(move || {
            counter += 1;
            counter -= n * (counter >= n) as usize;
            return counter == 0;
        })
    }

    fn separated_by(self, delta: f64) -> Self {
        let mut last_trigger = f64::NEG_INFINITY;
        self.filter_time(move |t| {
            if t >= last_trigger + delta {
                last_trigger = t;
                true
            } else {
                false
            }
        })
    }

    fn in_range(self, interval: impl 'a + std::ops::RangeBounds<f64>) -> Self {
        self.filter_time(move |t| interval.contains(&t))
    }

    fn once(self) -> Self {
        let mut flag = true;
        self.filter_constant(move || {
            if flag {
                flag = false;
                true
            } else {
                false
            }
        })
    }

    fn take(self, n: usize) -> Self {
        let mut counter = 0;
        self.filter_constant(move || {
            counter += 1;
            counter <= n
        })
    }

    fn times(self, range: impl 'a + std::ops::RangeBounds<usize>) -> Self {
        let mut counter = 0;
        self.filter_constant(move || {
            let ret = range.contains(&counter);
            counter += 1;
            ret
        })
    }
}
