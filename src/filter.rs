//! Defines [crate::Filter] trait for filtering callbacks in [crate::Event] and [crate::Loc].

use crate::{state::*, state_fn};

/// Trait manages adding filtering functions to a vector field of a class, such as
/// [crate::Event] and [crate::Loc].
pub trait Filter<const N: usize>: Sized {
    /// the type of self after application of filter method
    type Output<T>;

    /// push a new [StateFn] which returns `bool` to `self` and return `self` for chained syntax.
    fn filter<S: StateFnMut<N, Output = bool>>(self, f: S) -> Self::Output<S>;

    /// Push a new filtering function, which returns `true` every `n`th invocation, including the
    /// first invocation.
    /// The effect of that filtration, is that only every 'n'th event is remained.
    fn every(self, n: usize) -> Self::Output<impl StateFnMut<N, Output = bool>> {
        let mut counter = n - 1;
        self.filter(state_fn!(move || {
            counter += 1;
            counter -= n * (counter >= n) as usize;
            return counter == 0;
        }))
    }

    /// Push a new filtering function, which returns `true` every `n`th invocation, starting with
    /// `(offset % n)`th invocation, such that with `offset = 0` it is equivalent to [Filter::every].
    /// The effect of that filtration, is that only every 'n'th event is remained, starting from
    /// `offset`th event (zero-based).
    fn every_offset(
        self,
        n: usize,
        offset: usize,
    ) -> Self::Output<impl StateFnMut<N, Output = bool>> {
        let offset = offset % n;
        let mut counter = n - 1 - offset;
        self.filter(state_fn!(move || {
            counter += 1;
            counter -= n * (counter >= n) as usize;
            return counter == 0;
        }))
    }

    /// Push a new filtering function, which remembers the last time it returned `true` and returns
    /// `true` only if the state time is advanced at least by `delta` (inclusive) since the last
    /// returning `true`. The effect is that after filtration events are guaranteed to be separated
    /// in time by at least delta.
    fn separated_by(self, delta: f64) -> Self::Output<impl StateFnMut<N, Output = bool>> {
        let mut last_trigger = f64::NEG_INFINITY;
        self.filter(state_fn!(move |t| {
            if t >= last_trigger + delta {
                last_trigger = t;
                true
            } else {
                false
            }
        }))
    }

    /// Push a new filtering function, that filters all events, the state time of which is not
    /// contained in a given f64 range.
    fn in_range(
        self,
        interval: impl std::ops::RangeBounds<f64>,
    ) -> Self::Output<impl StateFnMut<N, Output = bool>> {
        self.filter(state_fn!(move |t| interval.contains(&t)))
    }

    /// Push a filtering function, that returns `true` on the first invocation and then always
    /// returns `false`. Equivalent in behaviour to [Filter::take(1)].
    fn once(self) -> Self::Output<impl StateFnMut<N, Output = bool>> {
        let mut flag = true;
        self.filter(state_fn!(move || {
            if flag {
                flag = false;
                true
            } else {
                false
            }
        }))
    }

    /// Push a filtering function, that returns `true` on the first `n` invocations and then always
    /// returns `false`. Equivalent in behaviour to [Filter::times(0..n)].
    fn take(self, n: usize) -> Self::Output<impl StateFnMut<N, Output = bool>> {
        let mut counter = 0;
        self.filter(state_fn!(move || {
            counter += 1;
            counter <= n
        }))
    }

    /// Push a filtering function, that returns `true` if the number of the invocation of that
    /// function is in `range`.
    fn times(
        self,
        range: impl std::ops::RangeBounds<usize>,
    ) -> Self::Output<impl StateFnMut<N, Output = bool>> {
        let mut counter = 0;
        self.filter(state_fn!(move || {
            let ret = range.contains(&counter);
            counter += 1;
            ret
        }))
    }
}
