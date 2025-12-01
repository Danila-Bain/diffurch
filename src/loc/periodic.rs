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

impl<T: Float> Periodic<T> {
    pub fn new(period: T) -> Self {
        Periodic {
            period,
            offset: T::zero(),
        }
    }

    pub fn with_offset(self, offset: T) -> Self {
        Self { offset, ..self }
    }
}

impl<const N: usize, T: Float + std::fmt::Debug> Detect<N, T> for Periodic<T> {
    fn detect<const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &State<N, S, S2, T, IC>,
    ) -> bool {
        let prev = ((state.t_prev - self.offset) / (self.period)).floor();
        let curr = ((state.t_curr - self.offset) / (self.period)).floor();
        // dbg!(prev, state.t_prev/self.period, curr, state.t_curr/self.period);
        return prev < curr;
    }
}
impl<const N: usize, T: Float + std::fmt::Debug> Locate<N, T> for Periodic<T> {
    fn locate<const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &State<N, S, S2, T, IC>,
    ) -> Option<T> {
        if self.detect(state) {
            let r =
                ((state.t_curr - self.offset) / self.period).floor() * self.period + self.offset;
            (r > state.t_prev).then_some(r)
        } else {
            None
        }
    }
}
