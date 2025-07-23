use crate::StateFnMut;

pub trait IntoDelay<const N: usize> {
    type Output: StateFnMut<N, Output = f64>;

    fn into_delay(self) -> Self::Output;

    fn max_delay(&self) -> f64;
}

impl<const N: usize> IntoDelay<N> for f64 {
    type Output = impl StateFnMut<N, Output = f64>;

    fn into_delay(self) -> Self::Output {
        crate::state_fn!(move |t| t - self)
    }

    fn max_delay(&self) -> f64 {
        *self
    }
}


impl<const N: usize, F: StateFnMut<N, Output = f64>> IntoDelay<N> for F {
    type Output = F;

    fn into_delay(self) -> Self::Output {
        self
    }

    fn max_delay(&self) -> f64 {
        f64::NAN
    }
}
