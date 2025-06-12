//! Defines [crate::Filter] trait for filtering callbacks in [crate::Event] and [crate::Loc].

use crate::state::*;

/// Trait manages adding filtering functions to a vector field of a class, such as
/// [crate::Event] and [crate::Loc].
pub trait Filter<const N: usize>: Sized {
    type Output<T>;
    /// push a new [StateFn] which returns `bool` to `self` and return `self` for chained syntax.
    fn filter<S: StateFnMut<N, bool>>(self, f: S) -> Self::Output<S>;

    /// push a new [StateFn::Constant], constructed from a given closure
    fn filter_constant<F: FnMut() -> bool>(self, f: F) -> Self::Output<impl StateFnMut<N, bool>> {
        self.filter(ConstantStateFnMut(f))
    }
    /// push a new [StateFn::Time], constructed from a given closure
    fn filter_time<F: FnMut(f64) -> bool>(self, f: F) -> Self::Output<impl StateFnMut<N, bool>> {
        self.filter(TimeStateFnMut(f))
    }
    /// push a new [StateFn::ODE], constructed from a given closure
    fn filter_ode<F: FnMut([f64; N]) -> bool>(
        self,
        f: F,
    ) -> Self::Output<impl StateFnMut<N, bool>> {
        self.filter(ODEStateFnMut(f))
    }
    /// push a new [StateFn::ODE2], constructed from a given closure
    fn filter_ode2<F: FnMut(f64, [f64; N]) -> bool>(
        self,
        f: F,
    ) -> Self::Output<impl StateFnMut<N, bool>> {
        self.filter(ODE2StateFnMut(f))
    }

    /// Push a new filtering function, which returns `true` every `n`th invocation, including the
    /// first invocation.
    /// The effect of that filtration, is that only every 'n'th event is remained.
    fn every(self, n: usize) -> Self::Output<impl StateFnMut<N, bool>> {
        let mut counter = n - 1;
        self.filter_constant(move || {
            counter += 1;
            counter -= n * (counter >= n) as usize;
            return counter == 0;
        })
    }

    /// Push a new filtering function, which returns `true` every `n`th invocation, starting with
    /// `(offset % n)`th invocation, such that with `offset = 0` it is equivalent to [Filter::every].
    /// The effect of that filtration, is that only every 'n'th event is remained, starting from
    /// `offset`th event (zero-based).
    fn every_offset(self, n: usize, offset: usize) -> Self::Output<impl StateFnMut<N, bool>> {
        let offset = offset % n;
        let mut counter = n - 1 - offset;
        self.filter_constant(move || {
            counter += 1;
            counter -= n * (counter >= n) as usize;
            return counter == 0;
        })
    }

    /// Push a new filtering function, which remembers the last time it returned `true` and returns
    /// `true` only if the state time is advanced at least by `delta` (inclusive) since the last
    /// returning `true`. The effect is that after filtration events are guaranteed to be separated
    /// in time by at least delta.
    fn separated_by(self, delta: f64) -> Self::Output<impl StateFnMut<N, bool>> {
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

    /// Push a new filtering function, that filters all events, the state time of which is not
    /// contained in a given f64 range.
    fn in_range(
        self,
        interval: impl std::ops::RangeBounds<f64>,
    ) -> Self::Output<impl StateFnMut<N, bool>> {
        self.filter_time(move |t| interval.contains(&t))
    }

    /// Push a filtering function, that returns `true` on the first invocation and then always
    /// returns `false`. Equivalent in behaviour to [Filter::take(1)].
    fn once(self) -> Self::Output<impl StateFnMut<N, bool>> {
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

    /// Push a filtering function, that returns `true` on the first `n` invocations and then always
    /// returns `false`. Equivalent in behaviour to [Filter::times(0..n)].
    fn take(self, n: usize) -> Self::Output<impl StateFnMut<N, bool>> {
        let mut counter = 0;
        self.filter_constant(move || {
            counter += 1;
            counter <= n
        })
    }

    /// Push a filtering function, that returns `true` if the number of the invocation of that
    /// function is in `range`.
    fn times(
        self,
        range: impl std::ops::RangeBounds<usize>,
    ) -> Self::Output<impl StateFnMut<N, bool>> {
        let mut counter = 0;
        self.filter_constant(move || {
            let ret = range.contains(&counter);
            counter += 1;
            ret
        })
    }
}
