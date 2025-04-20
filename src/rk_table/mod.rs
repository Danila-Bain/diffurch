pub trait RungeKuttaTable {
    const S: usize;

    const ORDER: usize;
    const ORDER_EMBEDDED: usize;
    const ORDER_INTERPOLANT: usize;

    const A: [&[f64]; Self::S] where [(); Self::S]:;
    const B: [f64; Self::S] where [(); Self::S]:;
    const B2: [f64; Self::S] where [(); Self::S]:;
    const C: [f64; Self::S] where [(); Self::S]:;

    const BI: [fn(f64) -> f64; Self::S] where [(); Self::S]:;

    #[cfg(test)]
    fn assert_a_has_correct_sizes()
    where
        [(); Self::S]:,
    {
        for i in 0..Self::S {
            assert_eq!(Self::A[i].len(), i);
        }
    }

    #[cfg(test)]
    fn interpolation_continuity_error() -> f64
    where
        [(); Self::S]:,
    {
        let mut max = 0f64;
        for i in 0..Self::S {
            max = max.max((Self::B[i] - Self::BI[i](1.)).abs());
        }
        max
    }

    #[cfg(test)]
    fn c_is_sum_of_a_error() -> f64
    where
        [(); Self::S]:,
    {
        let mut max = 0f64;
        for i in 0..Self::S {
            let sum = Self::A[i].iter().sum::<f64>();
            let diff = (Self::C[i] - sum).abs();
            max = f64::max(max, diff);
        }
        max
    }

    #[cfg(test)]
    fn order_conditions_error() -> f64
    where
        [(); Self::S]:,
        [(); Self::ORDER]:,
    {
        // On derivation and formulas for order conditions, see
        // E. Hairer, G. Wanner, and S. P. Nørsett, Solving ordinary differential equations I, vol. 8. in Springer Series in Computational Mathematics, vol. 8. Berlin, Heidelberg: Springer Berlin Heidelberg, 1993. doi: 10.1007/978-3-540-78862-1.
        let mut max_error = 0.;

        // order is q in the text
        for order in 1..=Self::ORDER {
            let advance_tree = |tree: &mut [usize]| -> bool {
                let mut i_of_increased = order - 1;

                // while cannot increase in this index
                while i_of_increased > 0 && tree[i_of_increased] + 1 >= i_of_increased {
                    i_of_increased -= 1;
                }

                if i_of_increased > 0 {
                    tree[i_of_increased] += 1;
                    for i in (i_of_increased + 1)..order {
                        tree[i] = tree[i_of_increased];
                    }
                    return true;
                } else {
                    return false;
                }
            };

            let fix_indexes = |tree: &[usize], indexes: &mut [usize]| {
                for i in (1..order).rev() {
                    indexes[tree[i]] = usize::max(indexes[tree[i]], indexes[i] + 1);
                }
            };

            let advance_indexes = |tree: &[usize], indexes: &mut [usize]| -> bool {
                let mut i_of_increased = order - 1;
                while i_of_increased > 0
                    && indexes[i_of_increased] + 1 >= indexes[tree[i_of_increased]]
                {
                    i_of_increased -= 1;
                }

                indexes[i_of_increased] += 1;
                for i in (i_of_increased + 1)..order {
                    indexes[i] = 0;
                }
                for i in ((i_of_increased + 1)..order).rev() {
                    indexes[tree[i]] = usize::max(indexes[tree[i]], indexes[i] + 1);
                }
                if i_of_increased > 0 {
                    return true;
                } else {
                    return false;
                }
            };

            let gamma = |tree: &[usize]| -> f64 {
                let mut rho = vec![0f64; order];
                for i in (1..order).rev() {
                    rho[i] += 1.;
                    rho[tree[i]] += rho[i];
                }
                rho[0] += 1.;

                rho.iter().product()
            };

            let mut tree = [0usize; Self::ORDER];

            // index[0] in 0..order, index[j] in 0..index[tree[j]]
            let mut indexes = [0usize; Self::ORDER];

            loop
            /* for all trees */
            {
                fix_indexes(&tree, &mut indexes);

                let mut b_phi_sum = 0.;
                let mut b_phi_sum_embedded = 0.;

                println!("{:?}", &tree[..order]);
                for j in indexes[0]..Self::S {
                    // j = 0 => sums are empty
                    indexes.fill(0);
                    indexes[0] = j;
                    fix_indexes(&tree, &mut indexes);

                    let mut phi_j = 0.;

                    loop
                    /* for all index packs starting with j */
                    {
                        // println!("\t{:?}", indexes);
                        phi_j += (1..order)
                            .fold(1., |acc, i| acc * Self::A[indexes[tree[i]]][indexes[i]]);

                        if !advance_indexes(&tree, &mut indexes) {
                            indexes.fill(0);
                            break;
                        }
                    }
                    b_phi_sum += phi_j * Self::B[j];
                    b_phi_sum_embedded += phi_j * Self::B2[j];
                }

                let error = f64::abs(b_phi_sum - 1. / gamma(&tree));
                let error_embedded = f64::abs(b_phi_sum_embedded - 1. / gamma(&tree));
                max_error = f64::max(max_error, error);
                if order <= Self::ORDER_EMBEDDED {
                    max_error = f64::max(max_error, error_embedded);
                }

                println!("\t{:?}", error);
                println!("\t{:?}", error_embedded);

                if !advance_tree(&mut tree) {
                    break;
                }
            }
        }

        max_error
    }
}

pub mod rk1;
pub mod rk2;
pub mod rk3;
pub mod rk4;
pub mod rk5;
pub mod rk6;
pub mod rk98;

macro_rules! fn_rk_tests {
    ($name:ident, $RK:path, $tolerance:expr) => {
        #[test]
        fn $name() {
            <$RK>::assert_a_has_correct_sizes();
            assert!(<$RK>::interpolation_continuity_error() < $tolerance);
            assert!(<$RK>::c_is_sum_of_a_error() < $tolerance);
            assert!(<$RK>::order_conditions_error() < $tolerance);
        }
    };
}

#[cfg(test)]
mod runge_kutta_tests {
    use super::*;

    fn_rk_tests!(euler, rk1::Euler, 1e-15);

    fn_rk_tests!(rk2_heun, rk2::Heun, 1e-15);
    fn_rk_tests!(rk2_ralston, rk2::Ralston, 1e-15);
    fn_rk_tests!(rk2_midpoint, rk2::Midpoint, 1e-15);

    fn_rk_tests!(rk3_ssp, rk3::SSP, 1e-15);
    fn_rk_tests!(rk3_heun, rk3::Heun, 1e-15);
    fn_rk_tests!(rk3_wray, rk3::Wray, 1e-15);
    fn_rk_tests!(rk3_kutta, rk3::Kutta, 1e-15);
    fn_rk_tests!(rk3_ralston, rk3::Ralston, 1e-15);

    fn_rk_tests!(rk4_classic, rk4::Classic, 1e-15);
    fn_rk_tests!(rk4_classic_dense, rk4::ClassicDense, 1e-15);

    fn_rk_tests!(dormand_prince, rk5::DormandPrince, 1e-11);

    fn_rk_tests!(rktp64, rk6::RKTP64, 1e-15);

    #[test]
    #[ignore]
    fn rk98() {
        <rk98::RK98>::assert_a_has_correct_sizes();
        assert!(<rk98::RK98>::interpolation_continuity_error() < 1e-15);
        assert!(<rk98::RK98>::c_is_sum_of_a_error() < 1e-15);
        assert!(<rk98::RK98>::order_conditions_error() < 1e-15);
    }
}
