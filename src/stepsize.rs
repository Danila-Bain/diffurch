use nalgebra::RealField;

use crate::traits::RealVectorSpace;

pub trait StepsizeController<T, Y> {
    fn init(&mut self);
    fn get(&self) -> T;
    fn set(&mut self, new_stepsize: T);
    fn update(&mut self, error: &Y) -> StepStatus;
}

pub enum StepStatus {
    Rejected,
    Accepted,
}

pub struct FixedStepsize<T>(pub T);
impl<T: Copy, Y> StepsizeController<T, Y> for FixedStepsize<T> {
    fn init(&mut self) {}

    fn get(&self) -> T {
        self.0
    }

    fn set(&mut self, new_stepsize: T) {
        self.0 = new_stepsize;
    }

    fn update(&mut self, _: &Y) -> StepStatus {
        StepStatus::Accepted
    }
}

pub struct AutomaticStepsize<T, Y> {
    stepsize: T,
    atol: Y,
    rtol: Y,
    order: u32,
    fac: T,
    fac_range: std::ops::Range<T>,
    initial_stepsize: Option<T>,
}

impl<T, Y> AutomaticStepsize<T, Y> {}

impl<T: RealField + Copy, Y: RealVectorSpace<T>> StepsizeController<T, Y>
    for AutomaticStepsize<T, Y>
where
    for<'a> &'a Y: IntoIterator<Item = &'a T>,
{
    fn init(&mut self) {
        self.stepsize = self.initial_stepsize.unwrap_or(T::from_f64(0.001).unwrap())
    }

    fn get(&self) -> T {
        self.stepsize
    }

    fn set(&mut self, new_stepsize: T) {
        self.stepsize = new_stepsize
    }
    fn update(&mut self, error: &Y) -> StepStatus {
        let err = error
            .into_iter()
            .zip(self.atol.into_iter())
            .zip(self.rtol.into_iter())
            .map(|((&err, &atol), &rtol)| err.abs() / (atol + err.abs() * rtol))
            .reduce(T::max)
            .unwrap_or(T::zero());

        let factor =
            self.fac * (T::one() / err).powf(T::one() / T::from_u32(self.order + 1).unwrap());
        let factor = factor.clamp(self.fac_range.start, self.fac_range.end);
        self.stepsize = self.stepsize * factor;

        match err >= T::one() {
            true => StepStatus::Rejected,
            false => StepStatus::Accepted,
        }
    }

}
