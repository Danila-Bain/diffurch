use nalgebra::RealField;
use num::Float;

use crate::{
    initial_condition::InitialCondition,
    loc::{detect::Detect, locate::Locate},
    state::State,
    traits::RealVectorSpace,
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

impl<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
> Detect<T, Y, S, I, IC> for Periodic<T>
{
    fn detect(&mut self, state: &State<T, Y, S, I, IC>) -> bool {
        let prev = ((state.t_prev - self.offset) / (self.period)).floor();
        let curr = ((state.t_curr - self.offset) / (self.period)).floor();
        // dbg!(prev, state.t_prev/self.period, curr, state.t_curr/self.period);
        prev < curr
    }
}
impl<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
> Locate<T, Y, S, I, IC> for Periodic<T>
{
    fn locate(&mut self, state: &State<T, Y, S, I, IC>) -> Option<T> {
        if self.detect(state) {
            let r =
                ((state.t_curr - self.offset) / self.period).floor() * self.period + self.offset;
            (r > state.t_prev).then_some(r)
        } else {
            None
        }
    }
}
