use nalgebra::RealField;
use num::Float;

pub trait IntegrationInterval<T: RealField> {
    fn start_bound(&self) -> T;
    fn end_bound(&self) -> T;
}

impl<T: RealField + Float, R: std::ops::RangeBounds<T>> IntegrationInterval<T> for R {
    fn start_bound(&self) -> T {
        use std::ops::Bound::*;

        match self.start_bound() {
            Unbounded => T::zero(),
            Included(&value) | Excluded(&value) => value,
        }
    }

    fn end_bound(&self) -> T {
        use std::ops::Bound::*;
        match self.end_bound() {
            Unbounded => T::infinity(),
            Included(&value) | Excluded(&value) => value,
        }
    }
}
