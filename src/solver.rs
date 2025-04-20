use crate::rk_table::*;
use crate::state::*;

trait DifferentialEquation {
    const N: usize;

    fn f(&self, s: &impl State<{Self::N}>) -> [f64; Self::N] where [(); Self::N]:,;

    fn solve<RK: RungeKuttaTable>(
        &self,
        interval: std::ops::Range<f64>,
        initial_function: &impl Fn(f64) -> [f64; Self::N],
    ) {
    }
}
struct HarmonicOscillator {
    w: f64,
}
impl DifferentialEquation for HarmonicOscillator {
    const N: usize = 2;
    fn f(&self, s: &impl State<{Self::N}>) -> [f64; Self::N] {
        let [x, dx] = s.x();
        [dx, -(self.w).powi(2) * x]
    }
}

#[cfg(test)]
mod test_solver {
    use super::*;

    #[test]
    fn test_solution() {
        let eq = HarmonicOscillator { w: 1. };

        let _res = eq.solve::<rk1::Euler>(0. ..f64::NAN, &|t: f64| {
            [(eq.w * t).sin(), eq.w * (eq.w * t).cos()]
        });

        let range = 0. ..10.;
        let initial_condition = |t: f64| [(eq.w * t).sin(), eq.w * (eq.w * t).cos()];
        let _res = eq.solve::<rk1::Euler>(range, &initial_condition);

    }
}

