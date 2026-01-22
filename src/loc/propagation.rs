use nalgebra::RealField;

use crate::{
    initial_condition::InitialCondition,
    loc::{Loc, detect::Detect},
    state::{EvalMutStateFn, EvalStateFn, State},
    traits::RealVectorSpace,
    util::partition_point_linear,
};

pub struct Propagation;

use impl_tools::autoimpl;
#[autoimpl(Debug ignore self.delayed_argument where T: std::fmt::Debug)]
pub struct Propagator<T, Delayed> {
    pub delayed_argument: Delayed,
    pub smoothing_order: usize,
    pub tracked_disco_t: T,
    pub tracked_disco_order: usize,
    pub tracked_queue_index: usize,
}

impl<T: RealField, Delayed> Propagator<T, Delayed> {
    pub fn new(delayed: Delayed, smoothing_order: usize) -> Self {
        Self {
            delayed_argument: delayed,
            smoothing_order,
            tracked_disco_t: T::min_value().unwrap(),
            tracked_disco_order: usize::MAX,
            tracked_queue_index: 0,
        }
    }
}

impl<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
    Delayed: EvalStateFn<T, Y, S, I, IC, T>,
    L,
> Detect<T, Y, S, I, IC> for Loc<T, Y, S, I, IC, Propagator<T, Delayed>, Propagation, L>
{
    fn detect(&mut self, state: &State<T, Y, S, I, IC>) -> bool {


        let propagator = &mut self.function;

        let prev = propagator.delayed_argument.eval_prev(state);
        let curr = propagator.delayed_argument.eval_curr(state);

        // detect if any element in state.disco_seq lies between prev and curr
        let partition_prev = partition_point_linear(
            &state.history.disco_deque,
            propagator.tracked_queue_index,
            |&(t, _order)| t <= prev,
        );
        let partition_curr = partition_point_linear(
            &state.history.disco_deque,
            partition_prev,
            |&(t, _order)| t <= curr,
        );

        propagator.tracked_queue_index = partition_prev.min(partition_curr);

        if partition_prev != partition_curr {
            let (t_disco, t_order) = *state.history.disco_deque.get(propagator.tracked_queue_index).unwrap();
            (propagator.tracked_disco_t, propagator.tracked_disco_order) = (t_disco, t_order + propagator.smoothing_order);
            true
        } else {
            false
        }
    }
}

// When used by Locate trait, it will lead to location of points, where delayed argument
// is equal to past discontinuity
impl<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
    Delayed: EvalStateFn<T, Y, S, I, IC, T>,
> EvalStateFn<T, Y, S, I, IC, T> for Propagator<T, Delayed>
{
    fn eval_curr(&mut self, state: &State<T, Y, S, I, IC>) -> T {
        self.delayed_argument.eval_curr(state) - self.tracked_disco_t
    }
    fn eval_prev(&mut self, state: &State<T, Y, S, I, IC>) -> T {
        self.delayed_argument.eval_prev(state) - self.tracked_disco_t
    }
    fn eval_at(&mut self, state: &State<T, Y, S, I, IC>, t: T) -> T {
        self.delayed_argument.eval_at(state, t) - self.tracked_disco_t
    }
}


impl<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
    Delayed: EvalStateFn<T, Y, S, I, IC, T>,
    L,
> EvalMutStateFn<T, Y, S, I, IC, ()> for Loc<T, Y, S, I, IC, Propagator<T, Delayed>, Propagation, L>
{
    fn eval_mut(&mut self, state: &mut State<T, Y, S, I, IC>) -> () {
        if self.function.tracked_disco_order < state.rk.order {
            state.history.disco_deque.push_back((state.t_curr, self.function.tracked_disco_order))
        }
    }
}

