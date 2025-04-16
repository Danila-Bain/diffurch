use crate::polynomial;
use crate::polynomial_body;

macro_rules! generic_rk_order3 {
    ($TypeName:ident, $alpha:expr, $beta:expr) => {
        pub struct $TypeName;
        impl crate::rk_table::RungeKuttaTable<3> for $TypeName {
            const ORDER: usize = 3;
            const ORDER_EMBEDDED: usize = 2;
            const ORDER_INTERPOLANT: usize = 1; // could be better
            const A: [&[f64]; Self::S] = [
                &[],
                &[$alpha],
                &[
                    ($beta / $alpha) * ($beta - 3. * $alpha * (1. - $alpha)) / (3. * $alpha - 2.),
                    -($beta / $alpha) * ($beta - $alpha) / (3. * $alpha - 2.),
                ],
            ];
            const B: [f64; Self::S] = [
                1. - (3. * $alpha + 3. * $beta - 2.) / (6. * $alpha * $beta),
                (3. * $beta - 2.) / (6. * $alpha * ($beta - $alpha)),
                (2. - 3. * $alpha) / (6. * $beta * ($beta - $alpha)),
            ];
            const B2: [f64; Self::S] = [1. - 0.5 / $alpha, 0.5 / $alpha, 0.];
            const C: [f64; Self::S] = [0., $alpha, $beta];

            // linear interpolation
            const BI: [fn(f64) -> f64; Self::S] = [
                polynomial![
                    0.,
                    1. - (3. * $alpha + 3. * $beta - 2.) / (6. * $alpha * $beta)
                ],
                polynomial![0., (3. * $beta - 2.) / (6. * $alpha * ($beta - $alpha))],
                polynomial![0., (2. - 3. * $alpha) / (6. * $beta * ($beta - $alpha))],
            ];
        }
    };
}

generic_rk_order3!(Kutta, 0.5, 1.);
generic_rk_order3!(Heun, 1./3., 2./3.);
generic_rk_order3!(Ralston, 1./2., 3./4.); // also used in the embedded Bogacki-Shampine
generic_rk_order3!(Wray, 8./15., 2./3.);
generic_rk_order3!(SSP, 1., 1./2.); // strong stability preserving
