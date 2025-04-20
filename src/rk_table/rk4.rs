use crate::polynomial;
use crate::polynomial_body;

pub struct Classic;
impl crate::rk_table::RungeKuttaTable for Classic {
    const S: usize = 4;
    const ORDER: usize = 4;
    const ORDER_EMBEDDED: usize = 2;
    const ORDER_INTERPOLANT: usize = 3;
    const A: [&[f64]; Self::S] = [&[], &[0.5], &[0., 0.5], &[0.0, 0.0, 1.0]];
    const B: [f64; Self::S] = [1. / 6., 1. / 3., 1. / 3., 1. / 6.];
    const B2: [f64; Self::S] = [0., 1., 0., 0.];
    const C: [f64; Self::S] = [0., 0.5, 0.5, 1.];
    const BI: [fn(f64) -> f64; Self::S] = [
        polynomial![0., 1., -1.5, 2. / 3.],
        polynomial![0., 0., 1., -2. / 3.],
        polynomial![0., 0., 1., -2. / 3.],
        polynomial![0., 0., -0.5, 2. / 3.],
    ];
}

pub struct ClassicDense;
impl crate::rk_table::RungeKuttaTable for ClassicDense {
    const S: usize = 5;
    const ORDER: usize = 4;
    const ORDER_EMBEDDED: usize = 3;
    const ORDER_INTERPOLANT: usize = 3;
    const A: [&[f64]; Self::S] = [
        &[],
        &[0.5],
        &[0., 0.5],
        &[0.0, 0.0, 1.0],
        &[5. / 32., 7. / 32., 13. / 32., -1. / 32.],
    ];
    const B: [f64; Self::S] = [1. / 6., 1. / 3., 1. / 3., 1. / 6., 0.];
    const B2: [f64; Self::S] = [-1. / 2., 7. / 3., 7. / 3., 13. / 6., -16. / 3.];
    const C: [f64; Self::S] = [0., 0.5, 0.5, 1., 0.75];
    const BI: [fn(f64) -> f64; 5] = [
        polynomial![0., 1., -1.5, 2. / 3.],
        polynomial![0., 0., 1., -2. / 3.],
        polynomial![0., 0., 1., -2. / 3.],
        polynomial![0., 0., -0.5, 2. / 3.],
        polynomial![0.],
    ];
}
