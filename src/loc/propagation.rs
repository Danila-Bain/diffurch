use nalgebra::RealField;
use num::Float;

use crate::{
    initial_condition::InitialCondition,
    loc::{Loc, detect::Detect},
    state::{EvalStateFn, State},
    traits::RealVectorSpace,
    util::partition_point_linear,
};

pub struct Propagation;

pub struct Propagator<T, Delayed> {
    pub delayed: Delayed,
    pub smoothing_order: usize,
    pub t_disco: T,
    pub t_order: usize,
    pub t_index: usize,
}

impl<T: RealField + Float, Delayed> Propagator<T, Delayed> {
    pub fn new(delayed: Delayed, smoothing_order: usize) -> Self {
        Self {
            delayed,
            smoothing_order,
            t_disco: T::nan(),
            t_order: usize::MAX,
            t_index: 0,
        }
    }
}

impl<T: RealField + Copy, Y: RealVectorSpace<T>, Delayed: EvalStateFn<T, Y, T>, L> Detect<T, Y>
    for Loc<Propagator<T, Delayed>, Propagation, L>
{
    fn detect<const S: usize, const I: usize, IC: InitialCondition<T, Y>>(
        &mut self,
        state: &State<T, Y, S, I, IC>,
    ) -> bool {
        let propagator = &mut self.function;

        let prev = propagator.delayed.eval_prev(state);
        let curr = propagator.delayed.eval_curr(state);

        // detect if any element in state.disco_seq lies between prev and curr
        let partition_prev = partition_point_linear(
            &state.history.disco_deque,
            propagator.t_index,
            |&(t, _order)| t <= prev,
        );
        let partition_curr = partition_point_linear(
            &state.history.disco_deque,
            partition_prev,
            |&(t, _order)| t <= curr,
        );

        if partition_prev != partition_curr {
            propagator.t_index = partition_prev.min(partition_curr);
            let (t_disco, t_order) = *state.history.disco_deque.get(propagator.t_index).unwrap();
            propagator.t_disco = t_disco;
            propagator.t_order = t_order + propagator.smoothing_order;
            return true;
        } else {
            return false;
        }
    }
}

// When used by Locate trait, it will lead to location of points, where delayed argument
// is equal to past discontinuity
impl<T: RealField + Copy, Y: RealVectorSpace<T>, Delayed: EvalStateFn<T, Y, T>> EvalStateFn<T, Y, T>
    for Propagator<T, Delayed>
{
    fn eval_curr<'s, const S: usize, const I: usize, IC: InitialCondition<T, Y>>(
        &mut self,
        state: &'s State<T, Y, S, I, IC>,
    ) -> T {
        self.delayed.eval_curr(state) - self.t_disco
    }

    fn eval_prev<'s, const S: usize, const I: usize, IC: InitialCondition<T, Y>>(
        &mut self,
        state: &'s State<T, Y, S, I, IC>,
    ) -> T {
        self.delayed.eval_prev(state) - self.t_disco
    }

    fn eval_at<'s, const S: usize, const I: usize, IC: InitialCondition<T, Y>>(
        &mut self,
        state: &'s State<T, Y, S, I, IC>,
        t: T,
    ) -> T {
        self.delayed.eval_at(state, t) - self.t_disco
    }
}
