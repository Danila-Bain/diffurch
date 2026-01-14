use nalgebra::RealField;

use crate::{initial_condition::InitialCondition, state::EvalMutStateFn, traits::RealVectorSpace};

pub struct StopIntegration();

impl<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
> EvalMutStateFn<T, Y, S, I, IC, ()> for StopIntegration
{
    fn eval_mut(&mut self, state: &mut super::State<T, Y, S, I, IC>) {
        state.make_zero_step();
        state.t_curr = T::max_value().unwrap_or(T::from_f64(f64::INFINITY).unwrap())
    }
}
