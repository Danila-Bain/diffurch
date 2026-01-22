use nalgebra::RealField;

use crate::traits::RealVectorSpace;

pub trait StepsizeController<T, Y> {
    fn init(&mut self);
    fn get(&self) -> T;
    fn set(&mut self, new_stepsize: T);
    fn update(&mut self, error: &Y) -> StepStatus;
}

#[derive(Clone, Copy, PartialEq)]
pub enum StepStatus {
    Rejected,
    Accepted,
}

impl<T: Copy, Y> StepsizeController<T, Y> for T {
    fn init(&mut self) {}

    fn get(&self) -> T {
        *self
    }

    fn set(&mut self, new_stepsize: T) {
        *self = new_stepsize;
    }

    fn update(&mut self, _: &Y) -> StepStatus {
        StepStatus::Accepted
    }
}

pub struct AutomaticStepsize<T, Y> {
    pub stepsize: T,
    pub stepsize_range: std::ops::Range<T>,
    pub atol: Y,
    pub rtol: Y,
    pub order: u32,
    pub fac: T,
    pub fac_range: std::ops::Range<T>,
    pub initial_stepsize: Option<T>,
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
            .zip(&self.atol)
            .zip(&self.rtol)
            .map(|((&err, &atol), &rtol)| err.abs() / (atol + err.abs() * rtol))
            .reduce(T::max)
            .unwrap_or(T::zero());

        let factor =
            self.fac * (T::one() / err).powf(T::one() / T::from_u32(self.order + 1).unwrap());
        let factor = factor.clamp(self.fac_range.start, self.fac_range.end);
        self.stepsize *= factor;
        self.stepsize = self
            .stepsize
            .clamp(self.stepsize_range.start, self.stepsize_range.end);

        match err >= T::one() {
            true => StepStatus::Rejected,
            false => StepStatus::Accepted,
        }
    }
}
