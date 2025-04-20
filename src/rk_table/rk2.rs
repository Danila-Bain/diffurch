use crate::polynomial;
use crate::polynomial_body;

macro_rules! generic_rk_order2 {
    ($TypeName:ident, $alpha:expr) => {
        pub struct $TypeName;
        impl crate::rk_table::RungeKuttaTable for $TypeName {
            const S: usize = 2;
            const ORDER: usize = 2;
            const ORDER_EMBEDDED: usize = 1;
            const ORDER_INTERPOLANT: usize = 1;
            const A: [&[f64]; 2] = [&[], &[$alpha]];
            const B: [f64; 2] = [1. - 0.5 / $alpha, 0.5 / $alpha];
            const B2: [f64; 2] = [1., 0.];
            const C: [f64; 2] = [0., $alpha];
            const BI: [fn(f64) -> f64; 2] =
                [polynomial![0., Self::B[0]], polynomial![0., Self::B[1]]]; // linear interpolation
        }
    };
}

generic_rk_order2!(Midpoint, 0.5);
generic_rk_order2!(Heun, 1.);
generic_rk_order2!(Ralston, 2. / 3.);
