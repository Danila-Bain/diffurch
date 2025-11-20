use num::Float;

use crate::{
    initial_condition::InitialCondition,
    loc::{detect::Detect, locate::Locate},
    state::State,
};

pub struct Periodic<T> {
    pub period: T,
    pub offset: T,
}

impl<const N: usize, T: Float> Detect<N, T> for Periodic<T> {
    fn detect<const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &State<N, S, S2, T, IC>,
    ) -> bool {
        ((state.t_prev - self.offset) / (self.period)).floor()
            < ((state.t_curr - self.offset) / (self.period)).floor()
    }
}
impl<const N: usize, T: Float> Locate<N, T> for Periodic<T> {
    fn locate<const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &State<N, S, S2, T, IC>,
    ) -> Option<T> {
        self.detect(state).then(|| {
            ((state.t_prev - self.offset) / self.period).ceil() * self.period + self.offset
        })
    }
}
