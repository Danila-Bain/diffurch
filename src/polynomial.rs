//! Defines [crate::polynomial!] macro and [crate::polynomial::Differentiable] conatiner for pairs
//! of closures.

#[derive(Clone)]
pub struct Differentiable<F, DF>(pub F, pub DF);

#[macro_export]
macro_rules! polynomial_closure {
    ($type:ty) => {
        move |_t: $type| { 0. }
    };
    ($type:ty, $($coef:expr),+ $(,)?) => {
        move |t: $type| {
            [$($coef),+].into_iter().rev()
            .reduce(|acc: $type, c: $type| c + t * acc).unwrap()
        }
    };
}

#[macro_export]
macro_rules! polynomial_derivative_closure {
    ($type:ty) => {
        move |_t: $type| { <$type as num::NumCast>::from(0.).unwrap() }
    };
    ($type:ty, $coef:expr) => {
        move |_t: $type| { <$type as num::NumCast>::from(0.).unwrap() }
    };
    ($type:ty, $($coef:expr),+ $(,)?) => {
        move |t: $type| {
            let coef = [$($coef),+];
            let last = *coef.last().unwrap();
            coef.into_iter().enumerate().skip(1).rev().skip(1)
            .fold(last * <$type as num::NumCast>::from((coef.len() - 1) as f64).unwrap(), |acc: $type, (n, c): (usize, $type)| <$type as num::NumCast>::from(n as f64).unwrap() * c + t * acc)
        }
    };
}

#[macro_export]
macro_rules! polynomial {
    ($type:ty, $($coef:expr),* $(,)?) => {
        $crate::polynomial::Differentiable(
            $crate::polynomial_closure![$type, $($coef),*],
            $crate::polynomial_derivative_closure![$type, $($coef),*]
        )
    };
    // ($t:ident => $($coef:expr),+) => {
    //     [$($coef),+].into_iter().rev()
    //         .reduce(|acc: f64, c: f64| c + $t * acc).unwrap()
    // };
}

#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn empty_polynomial() {

        let p = polynomial![f64, 0.];
        assert_eq!((p.0)(0.), 0.);
        assert_eq!((p.0)(1.), 0.);
        assert_eq!((p.1)(0.), 0.);
        assert_eq!((p.1)(1.), 0.);
    }

    #[test]
    fn constant_polynomail_evauation() {
        let p = polynomial_closure![f64, 42.];
        assert_eq!(p(0.), 42.);
        assert_eq!(p(1.), 42.);
    }

    #[test]
    fn linear_polynomial_evaluation() {
        let p = polynomial_closure![f64, 42., 2.];
        assert_eq!(p(0.), 42.);
        assert_eq!(p(69.), 42. + 2. * 69.);
    }

    #[test]
    fn quadratic_polynomial_evaluation() {
        let p = polynomial_closure![f64, -1., 0., 1.];
        assert_eq!(p(0.), -1.);
        assert_eq!(p(-1.), 0.);
        assert_eq!(p(1.), 0.);
    }

    #[test]
    fn geometric_sum() {
        let geometric_series = polynomial_closure![f64,
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
        let p = polynomial_derivative_closure![f64, 42.];
        assert_eq!(p(0.), 0.);
        assert_eq!(p(1.), 0.);
    }

    #[test]
    fn linear_polynomial_derivative() {
        let p = polynomial_derivative_closure![f64, 42., 2.];
        assert_eq!(p(0.), 2.);
        assert_eq!(p(69.), 2.);
    }

    #[test]
    fn quadratic_polynomial_derivative() {
        let p = polynomial_derivative_closure![f64, -1., 0., 1.];
        assert_eq!(p(0.), 2. * 0.);
        assert_eq!(p(-1.), 2. * (-1.));
        assert_eq!(p(1.), 2. * 1.);
    }

    #[test]
    fn geometric_sum_derivative() {
        let geometric_series = polynomial_derivative_closure![f64, 
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
