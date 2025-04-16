use crate::polynomial;
use crate::polynomial_body;

pub struct Euler;
impl crate::rk_table::RungeKuttaTable<1> for Euler {
    const ORDER: usize = 1;
    const ORDER_EMBEDDED: usize = 0;
    const ORDER_INTERPOLANT: usize = 1;
    const A: [&[f64]; 1] = [&[]];
    const B: [f64; 1] = [1.];
    const B2: [f64; 1] = [0.];
    const C: [f64; 1] = [0.];
    const BI: [fn(f64) -> f64; 1] = [polynomial![0., 1.]];
}
