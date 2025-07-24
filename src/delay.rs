use crate::{location::{Bisection, Lerp}, StateFnMut};

pub trait IntoDelay<const N: usize> {
    type Output: StateFnMut<N, Output = f64>;

    type LocationMethod;

    fn into_delay(self) -> Self::Output;

    fn max_delay(&self) -> f64;

    fn location(&self) -> Self::LocationMethod;
}

impl<const N: usize> IntoDelay<N> for f64 {
    type Output = impl StateFnMut<N, Output = f64>;

    type LocationMethod = Lerp;

    fn into_delay(self) -> Self::Output {
        crate::state_fn!(move |t| t - self)
    }

    fn max_delay(&self) -> f64 {
        *self
    }

    fn location(&self) -> Self::LocationMethod {
        Lerp
    }
}


impl<const N: usize, F: StateFnMut<N, Output = f64>> IntoDelay<N> for F {
    type Output = F;

    type LocationMethod = Bisection;

    fn into_delay(self) -> Self::Output {
        self
    }

    fn max_delay(&self) -> f64 {
        f64::NAN
    }

    fn location(&self) -> Self::LocationMethod {
        Bisection
    }
}
