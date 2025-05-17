#[derive(Clone)]
pub struct Differentiable<F, DF>(pub F, pub DF);

impl<Ret, F: FnOnce(f64) -> Ret, DF> FnOnce<(f64,)> for Differentiable<F, DF> {
    type Output = Ret;

    extern "rust-call" fn call_once(self, args: (f64,)) -> Self::Output {
        (self.0)(args.0)
    }
}
impl<Ret, F: FnMut(f64) -> Ret, DF> FnMut<(f64,)> for Differentiable<F, DF> {
    extern "rust-call" fn call_mut(&mut self, args: (f64,)) -> Self::Output {
        (self.0)(args.0)
    }
}
impl<Ret, F: Fn(f64) -> Ret, DF> Fn<(f64,)> for Differentiable<F, DF> {
    extern "rust-call" fn call(&self, args: (f64,)) -> Self::Output {
        (self.0)(args.0)
    }
}

impl<F, DF, Ret> Differentiable<F, DF>
where
    DF: Fn<(f64,), Output = Ret>,
{
    pub fn d(&self, t: f64) -> Ret {
        (self.1)(t)
    }
}

pub trait WithDerivative<DF>
where
    Self: Sized,
{
    fn with_derivative(self, derivative: DF) -> Differentiable<Self, DF> {
        Differentiable(self, derivative)
    }
}

impl<Ret, F: Fn(f64) -> Ret, DF: Fn(f64) -> Ret> WithDerivative<DF> for F {}

/// Produce a fn(f64) -> f64 closure that represents a polynomial function with given coefficients.
///
/// # Examples:
/// ```rust 
/// use diffurch::polynomial_closure;
///
/// let p0 = polynomial_closure![1., 0., -1./2., 0., 1./24.];
/// let p1 = |t: f64| 1. - t*t/2. + t.powi(4)/24.;
/// let p2 = |t: f64| 1. + t * (t * ( - 1./2. + t * (t * (1./24.))));
///
/// for i in 0..10 {
///     let t = i as f64;
///     assert!((p0(t) - p1(t)).abs() < 1e-13); // not exact due to rounding errors
///     assert_eq!(p0(t), p2(t));
/// }
/// ```
#[macro_export]
macro_rules! polynomial_closure {
    () => {
        |_t: f64| { 0. }
    };
    ($($coef:expr),+ $(,)?) => {
        |t: f64| {
            [$($coef),+].into_iter().rev()
            .reduce(|acc: f64, c: f64| c + t * acc).unwrap()
        }
    };
}

/// Same as [crate::polynomial_closure], but produces a closure, that corresponds to differentiated
/// polynomial function.
#[macro_export]
macro_rules! polynomial_derivative_closure {
    () => {
        |_t: f64| { 0. }
    };
    ($coef:expr) => {
        |_t: f64| { 0. }
    };
    ($($coef:expr),+ $(,)?) => {
        |t: f64| {
            let coef = [$($coef),+];
            let last = *coef.last().unwrap();
            coef.into_iter().enumerate().skip(1).rev().skip(1)
            .fold(last * (coef.len()-1) as f64, |acc: f64, (n, c): (usize, f64)| n as f64 * c + t * acc)
        }
    };
}

/// Produces [crate::util::with_derivative::Differentiable] that holds a polynomial, produced by
/// [crate::polynomial_closure] and its derivative closure, produced by
/// [crate::polynomial_derivative_closure].
///
/// # Example
/// ```rust
/// use diffurch::polynomial;
///
/// let p0 = polynomial![1., 0., -1./2., 0., 1./24.];
/// let p1 = |t: f64| 1. - t*t/2. + t.powi(4)/24.;
/// let p2 = |t: f64| 1. + t * (t * ( - 1./2. + t * (t * (1./24.))));
///
/// let d1 = |t: f64| -t + t.powi(3)/6.;
/// let d2 = |t: f64| t * (-1. + t*t*(1./6.));
///
/// for i in 0..10 {
///     let t = i as f64;
///     assert!((p0(t) - p1(t)).abs() < 1e-13); // not exact due to rounding errors
///     assert_eq!(p0(t), p2(t));
///
///     assert!((p0.d(t) - d1(t)).abs() < 1e-13); // not exact due to rounding errors
///     assert_eq!(p0.d(t), d2(t));
/// }
/// ```
#[macro_export]
macro_rules! polynomial {
    ($($coef:expr),*) => {
        $crate::polynomial::Differentiable(
            $crate::polynomial_closure![$($coef),*],
            $crate::polynomial_derivative_closure![$($coef),*]
        )
    };
    ($t:ident => $($coef:expr),+) => {
        [$($coef),+].into_iter().rev()
            .reduce(|acc: f64, c: f64| c + $t * acc).unwrap()
    };
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn constant_polynomail_evauation() {
        let p = polynomial_closure![42.];
        assert_eq!(p(0.), 42.);
        assert_eq!(p(1.), 42.);
    }

    #[test]
    fn linear_polynomial_evaluation() {
        let p = polynomial_closure![42., 2.];
        assert_eq!(p(0.), 42.);
        assert_eq!(p(69.), 42. + 2. * 69.);
    }

    #[test]
    fn quadratic_polynomial_evaluation() {
        let p = polynomial_closure![-1., 0., 1.];
        assert_eq!(p(0.), -1.);
        assert_eq!(p(-1.), 0.);
        assert_eq!(p(1.), 0.);
    }

    #[test]
    fn geometric_sum() {
        let geometric_series = polynomial_closure![
            1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1.,
            1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1.,
            1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1.,
            1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1.,
        ];

        assert_eq!(geometric_series(1. / 2.), 1. / (1. - 1. / 2.));
        assert_eq!(geometric_series(1. / 3.), 1.5);
        assert_eq!(geometric_series(1. / 4.), 1. / (1. - 1. / 4.));
        assert_eq!(geometric_series(1. / 5.), 1. / (1. - 1. / 5.));
        assert_eq!(geometric_series(1. / 6.), 1. / (1. - 1. / 6.));
    }

    #[test]
    fn constant_polynomail_derivative() {
        let p = polynomial_derivative_closure![42.];
        assert_eq!(p(0.), 0.);
        assert_eq!(p(1.), 0.);
    }

    #[test]
    fn linear_polynomial_derivative() {
        let p = polynomial_derivative_closure![42., 2.];
        assert_eq!(p(0.), 2.);
        assert_eq!(p(69.), 2.);
    }

    #[test]
    fn quadratic_polynomial_derivative() {
        let p = polynomial_derivative_closure![-1., 0., 1.];
        assert_eq!(p(0.), 2. * 0.);
        assert_eq!(p(-1.), 2. * (-1.));
        assert_eq!(p(1.), 2. * 1.);
    }

    #[test]
    fn geometric_sum_derivative() {
        let geometric_series = polynomial_derivative_closure![
            1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1.,
            1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1.,
            1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1.,
            1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1.,
        ];

        assert_eq!(geometric_series(1. / 2.), (1f64 / (1. - 1. / 2.)).powi(2));
        assert_eq!(geometric_series(1. / 3.), 1.5f64.powi(2));
        assert_eq!(geometric_series(1. / 4.), (1f64 / (1. - 1. / 4.)).powi(2));
        assert_eq!(geometric_series(1. / 5.), (1f64 / (1. - 1. / 5.)).powi(2));
        assert_eq!(geometric_series(1. / 6.), (1f64 / (1. - 1. / 6.)).powi(2));
    }
}
