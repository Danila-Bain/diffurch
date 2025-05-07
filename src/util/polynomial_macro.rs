// The main macro which creates an anonymous function that computes the polynomial.

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

#[macro_export]
macro_rules! polynomial {
    ($($coef:expr),*) => {
        $crate::util::with_derivative::Differentiable(
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
