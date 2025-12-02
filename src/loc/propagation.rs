use num::Float;

use crate::{
    initial_condition::InitialCondition,
    loc::{Loc, detect::Detect},
    state::{EvalStateFn, State},
    util::partition_point_linear,
};

pub struct Propagation;

pub struct Propagator<const N: usize, T: Float, Delayed: EvalStateFn<N, T, T>> {
    pub delayed: Delayed,
    pub smoothing_order: usize,
    pub t_disco: T,
    pub t_order: usize,
    pub t_index: usize,
}

impl<const N: usize, T: Float, Delayed: EvalStateFn<N, T, T>> Propagator<N, T, Delayed> {
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


impl<const N: usize, T: Float, Delayed: EvalStateFn<N, T, T>, L> Detect<N, T>
    for Loc<Propagator<N, T, Delayed>, Propagation, L>
{
    fn detect<const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &State<N, S, S2, T, IC>,
    ) -> bool {
        let propagator = &mut self.function;

        let prev = propagator.delayed.eval_prev(state);
        let curr = propagator.delayed.eval_curr(state);

        // detect if any element in state.disco_seq lies between prev and curr
        let partition_prev =
            partition_point_linear(&state.history.disco_deque, propagator.t_index, |&(t, _order)| {
                t <= prev
            });
        let partition_curr =
            partition_point_linear(&state.history.disco_deque, partition_prev, |&(t, _order)| t <= curr);

        if partition_prev != partition_curr {
            propagator.t_index = partition_prev.min(partition_curr);
            let (t_disco, t_order) = 
                *state.history.disco_deque.get(propagator.t_index).unwrap();
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
impl<const N: usize, T: Float, Delayed: EvalStateFn<N, T, T>> EvalStateFn<N, T, T> for
Propagator<N, T, Delayed> {
    fn eval_curr<'s, const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &'s State<N, S, S2, T, IC>,
    ) -> T {
        self.delayed.eval_curr(state) - self.t_disco
    }

    fn eval_prev<'s, const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &'s State<N, S, S2, T, IC>,
    ) -> T {
        self.delayed.eval_prev(state) - self.t_disco
    }

    fn eval_at<'s, const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &'s State<N, S, S2, T, IC>,
        t: T,
    ) -> T {
        self.delayed.eval_at(state, t) - self.t_disco
    }
}
