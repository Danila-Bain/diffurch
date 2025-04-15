// The helper macro performs recursive expansion of the coefficients.
#[macro_export]
macro_rules! polynomial_body {
    // Base case: only one coefficient left.
    ($t:ident, $a:expr) => { $a };
    // Recursive case: take the first coefficient and multiply the next inner expression by t.
    ($t:ident, $a:expr, $($rest:expr),+) => {
        $a + $t * (polynomial_body!($t, $($rest),+))
    };
}

// The main macro which creates an anonymous function that computes the polynomial.
#[macro_export]
macro_rules! polynomial {
    ($($coef:expr),+ $(,)?) => {
        |_t| { polynomial_body!(_t, $($coef),+) }
    };
}


#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn constant_polynomail_evauation() {
        let p = polynomial![42.];
        assert_eq!(p(0.), 42.);
        assert_eq!(p(1.), 42.);
    }

    #[test]
    fn linear_polynomial_evaluation() {
        let p = polynomial![42., 2.];
        assert_eq!(p(0.), 42.);
        assert_eq!(p(69.), 42. + 2. * 69.);
    }

    #[test]
    fn quadratic_polynomial_evaluation() {
        let p = polynomial![-1., 0., 1.];
        assert_eq!(p(0.), -1.);
        assert_eq!(p(-1.), 0.);
        assert_eq!(p(1.), 0.);
    }
}
