pub trait RungeKuttaTable<const S: usize> {
    const S: usize = S;

    const ORDER: usize;
    const ORDER_EMBEDDED: usize;
    const ORDER_INTERPOLANT: usize;

    const A: [&[f64]; S];
    const B: [f64; S];
    const B2: [f64; S];
    const C: [f64; S];

    const BI: [fn(f64) -> f64; S];

    #[cfg(test)]
    fn interpolation_continuity_error() -> f64 {
        let mut max = 0f64;
        for i in 0..S {
            max = max.max((Self::B[i] - Self::BI[i](1.)).abs());
        }
        max
    }

    #[cfg(test)]
    fn c_is_sum_of_a_error() -> f64 {
        let mut max = 0f64;
        for i in 0..S {
            let sum = Self::A[i].iter().sum::<f64>();
            let diff = (Self::C[i] - sum).abs();
            max = f64::max(max, diff);
        }
        max
    }
}

pub mod dp54;
pub mod euler;
pub mod rk4;
pub mod rk98;
pub mod rktp64;

#[cfg(test)]
mod interpolation_tests {
    use super::*;

    #[test]
    fn runge_kutta_interpolation_continuity() {
        assert!(rk4::RK4::interpolation_continuity_error() < 1e-15);
        assert!(rk4::RK43::interpolation_continuity_error() < 1e-15);
        assert!(euler::Euler::interpolation_continuity_error() < 1e-15);
        assert!(euler::HeunEuler::interpolation_continuity_error() < 1e-15);
        assert!(rk98::RK98::interpolation_continuity_error() < 1e-11);
    }


    #[test]
    fn runge_kutta_c_vs_a_consistency() {
        assert!(rk4::RK4::c_is_sum_of_a_error() < 1e-15);
        assert!(rk4::RK43::c_is_sum_of_a_error() < 1e-15);
        assert!(euler::Euler::c_is_sum_of_a_error() < 1e-15);
        assert!(euler::HeunEuler::c_is_sum_of_a_error() < 1e-15);
        assert!(rk98::RK98::c_is_sum_of_a_error() < 1e-11);
    }

    // fn test_order_conditions<RK: RungeKuttaTable<S>, const S: usize>() {}
}
