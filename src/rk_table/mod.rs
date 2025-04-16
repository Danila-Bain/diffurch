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
    fn assert_a_has_correct_sizes() {
        for i in 0..S {
            assert_eq!(Self::A[i].len(), i);
        }
    }

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

    #[cfg(test)]
    fn order_conditions_error() -> f64
    where
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

                println!("{:?}", tree);
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

pub mod dp54;
pub mod euler;
pub mod rk4;
pub mod rk98;
pub mod rktp64;

#[cfg(test)]
mod runge_kutta_tests {
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
    fn runge_kutta_a_has_correct_size() {
        rk4::RK4::assert_a_has_correct_sizes();
        rk4::RK43::assert_a_has_correct_sizes();
        euler::Euler::assert_a_has_correct_sizes();
        euler::HeunEuler::assert_a_has_correct_sizes();
        rk98::RK98::assert_a_has_correct_sizes();
    }

    #[test]
    fn runge_kutta_c_vs_a_consistency() {
        assert!(rk4::RK4::c_is_sum_of_a_error() < 1e-15);
        assert!(rk4::RK43::c_is_sum_of_a_error() < 1e-15);
        assert!(euler::Euler::c_is_sum_of_a_error() < 1e-15);
        assert!(euler::HeunEuler::c_is_sum_of_a_error() < 1e-15);
        assert!(rk98::RK98::c_is_sum_of_a_error() < 1e-11);
    }

    #[test]
    fn runge_kutta_order_conditions() {
        assert!(rk4::RK4::order_conditions_error() < 1e-15);
        assert!(rk4::RK43::order_conditions_error() < 1e-15);
        assert!(euler::Euler::order_conditions_error() == 0.);
        assert!(euler::HeunEuler::order_conditions_error() == 0.);
    }

    #[test]
    #[ignore]
    fn runge_kutta_order_conditions_rk98() {
        // to get to complete 8th order it took 1 hour 55 minutes
        // overall errors are good (around 1e-15),
        // but very ocasionally, it gets to 1e-9, 1e-6, or even 1e-4 (for 7th order)
        assert!(rk98::RK98::order_conditions_error() < 1e-8); // will fail
    }
    // fn test_order_conditions<RK: RungeKuttaTable<S>, const S: usize>() {}
}
