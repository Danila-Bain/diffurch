#[derive(Debug, Clone)]
pub struct Polynomial<const DEG: usize>([f64; DEG + 1])
where
    [(); DEG + 1]:;

impl<const DEG: usize> std::ops::Index<usize> for Polynomial<DEG> where [(); DEG + 1]:, {
    type Output = f64;
    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl<const DEG: usize> Polynomial<DEG>
where
    [(); DEG + 1]:,
{
    pub fn from(coefficients: [f64; DEG + 1]) -> Self {
        Self(coefficients)
    }

    pub fn eval(&self, t: f64) -> f64 {
        let mut res = self[DEG];
        for i in (0..DEG).rev() {
            res *= t;
            res += self[i];
        }
        res
    }

    pub fn derivative(&self) -> Self {
        let mut coefficients = [0f64; DEG + 1];
        for i in 0..DEG {
            coefficients[i] = self[i + 1] * (i + 1) as f64;
        }
        Self(coefficients)
    }
}

impl<const DEG: usize> std::ops::Add for Polynomial<DEG>
where
    [(); DEG + 1]:,
{
    type Output = Self;

    // why consuming?
    fn add(self, other: Self) -> Self::Output {
        Self(core::array::from_fn::<_, { DEG + 1 }, _>(|i| {
            self[i] + other[i]
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_polynomail_evauation() {
        let p = Polynomial::<0>::from([42.]);
        assert_eq!(p.eval(0.), 42.);
        assert_eq!(p.eval(1.), 42.);
    }

    #[test]
    fn linear_polynomial_evaluation() {
        let p = Polynomial::<1>::from([42., 2.]);
        assert_eq!(p.eval(0.), 42.);
        assert_eq!(p.eval(69.), 42. + 2. * 69.);
    }

    #[test]
    fn quadratic_polynomial_evaluation() {
        let p = Polynomial::<2>::from([-1., 0., 1.]);
        assert_eq!(p.eval(0.), -1.);
        assert_eq!(p.eval(-1.), 0.);
        assert_eq!(p.eval(1.), 0.);
    }

    #[test]
    fn add_polynomials() {
        let p1 = Polynomial::<2>::from([1., 2., 3.]);
        let p2 = Polynomial::<2>::from([-1., -2., -3.]);
        let p = p1 + p2;
        assert_eq!(p.eval(-1.), 0.);
        assert_eq!(p.eval(0.), 0.);
        assert_eq!(p.eval(1.), 0.);
    }
}
