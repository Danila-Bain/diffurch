use crate::traits::RealVectorSpace;
use nalgebra::RealField;

#[derive(Clone, Debug)]
pub struct ButcherTableu<T, const S: usize, const I: usize = S> {
    pub order: usize,
    pub order_embedded: usize,
    pub order_interpolant: usize,
    pub a: [[T; S]; S],
    pub b: [T; S],
    pub b2: [T; S],
    pub c: [T; S],
    pub bi: [[T; I]; S],
}

impl<T: RealField + Copy, const S: usize, const I: usize> ButcherTableu<T, S, I> {
    pub fn dense_output<const D: usize, Y: RealVectorSpace<T>>(
        &self,
        y_prev: &Y,
        t_step: T,
        theta: T,
        k: &[Y; S],
    ) -> Y {
        match D {
            0 => {
                let mut delta = Y::zero();
                for i in 0..S {
                    let mut b_i = T::zero();
                    for j in 0..I {
                        b_i += self.bi[i][j] * theta.powi(j as i32)
                    }
                    delta += k[i] * b_i;
                }
                return *y_prev + delta * t_step;
            }
            1 => {
                let mut delta = Y::zero();
                for i in 0..S {
                    let mut b_i = T::zero();
                    for j in 1..I {
                        b_i +=
                            self.bi[i][j] * theta.powi((j - 1) as i32) * T::from_usize(i).unwrap()
                    }
                    delta += k[i] * b_i;
                }
                return delta;
            }
            _ => {
                unimplemented!()
            }
        }
    }
}

impl<T> ButcherTableu<T, 1, 2>
where
    T: RealField,
{
    /// Euler method (<https://en.wikipedia.org/wiki/Euler_method>), with linear interpolation
    pub fn euler() -> Self {
        ButcherTableu {
            order: 1,
            order_embedded: 0,
            order_interpolant: 1,
            a: [[T::zero()]],
            b: [T::one()],
            b2: [T::zero()],
            c: [T::zero()],
            bi: [[T::zero(), T::one()]],
        }
    }
}

impl<T> ButcherTableu<T, 2, 2>
where
    T: RealField + Copy,
{
    // J.C. Butcher - Numerical Methods for Ordinary Differential Equations, p. 185
    pub fn generic_order_2(c2: f64) -> Self {
        let a = [[0., 0.], [c2, 0.]];
        let b = [1. - 1. / (2. * c2), 1. / (2. * c2)];
        let b2 = [1., 0.];
        let c = [0., c2];
        let bi = [[0., b[0]], [0., b[1]]];

        ButcherTableu {
            order: 2,
            order_embedded: 1,
            order_interpolant: 1,
            a: a.map(|row| row.map(|x| T::from_f64(x).unwrap())),
            b: b.map(|x| T::from_f64(x).unwrap()),
            b2: b2.map(|x| T::from_f64(x).unwrap()),
            c: c.map(|x| T::from_f64(x).unwrap()),
            bi: bi.map(|row| row.map(|x| T::from_f64(x).unwrap())),
        }
    }

    pub fn midpoint() -> Self {
        Self::generic_order_2(0.5)
    }

    pub fn heun2() -> Self {
        Self::generic_order_2(1.)
    }

    pub fn ralston2() -> Self {
        Self::generic_order_2(2. / 3.)
    }
}

impl<T> ButcherTableu<T, 3, 2>
where
    T: RealField + Copy,
{
    // J.C. Butcher - Numerical Methods for Ordinary Differential Equations, p. 186
    pub fn generic_order_3(c2: f64, c3: f64, b3: Option<f64>) -> Self {
        if c2 != 0. && c3 != 0. && c2 != c3 && c2 != 2. / 3. && b3.is_none() {
            // solvable case I
            let a = [
                [0., 0., 0.],
                [c2, 0., 0.],
                [
                    (c3 * (3. * c2 * (1. - c2) - c3)) / (c2 * (2. - 3. * c2)),
                    (c3 * (c3 - c2)) / (c2 * (2. - 3. * c2)),
                    0.,
                ],
            ];
            let b = [
                1. - (1. * c2 + 3. * c3 - 2.) / (6. * c2 * c3),
                (3. * c3 - 2.) / (6. * c2 * (c3 - c2)),
                (2. - 3. * c2) / (6. * c3 * (c3 - c2)),
            ];
            let b2 = [1. - 1. / (2. * c2), 1. / (2. * c2), 0.];
            let c = [0., c2, c3];
            let bi = [[0., b[0]], [0., b[1]], [0., b[2]]];

            ButcherTableu {
                order: 2,
                order_embedded: 1,
                order_interpolant: 1,
                a: a.map(|row| row.map(|x| T::from_f64(x).unwrap())),
                b: b.map(|x| T::from_f64(x).unwrap()),
                b2: b2.map(|x| T::from_f64(x).unwrap()),
                c: c.map(|x| T::from_f64(x).unwrap()),
                bi: bi.map(|row| row.map(|x| T::from_f64(x).unwrap())),
            }
        } else if c2 == 2. / 3.
            && c3 == 2. / 3.
            && let Some(b3) = b3
            && b3 != 0.
        {
            // solvable case II
            let a = [
                [0., 0., 0.],
                [c2, 0., 0.],
                [2. / 3. - 1. / (4. * b3), 1. / (4. * b3), 0.],
            ];
            let b = [1. / 4., 3. / 4. - b3, b3];
            let b2 = [1. - 1. / (2. * c2), 1. / (2. * c2), 0.];
            let c = [0., c2, c3];
            let bi = [[0., b[0]], [0., b[1]], [0., b[2]]];

            ButcherTableu {
                order: 2,
                order_embedded: 1,
                order_interpolant: 1,
                a: a.map(|row| row.map(|x| T::from_f64(x).unwrap())),
                b: b.map(|x| T::from_f64(x).unwrap()),
                b2: b2.map(|x| T::from_f64(x).unwrap()),
                c: c.map(|x| T::from_f64(x).unwrap()),
                bi: bi.map(|row| row.map(|x| T::from_f64(x).unwrap())),
            }
        } else if c2 == 2. / 3.
            && c3 == 0.
            && let Some(b3) = b3
            && b3 != 0.
        {
            // solvable case III
            let a = [
                [0., 0., 0.],
                [c2, 0., 0.],
                [-1. / (4. * b3), 1. / (4. * b3), 0.],
            ];
            let b = [1. / 4. - b3, 3. / 4., b3];
            let b2 = [1. - 1. / (2. * c2), 1. / (2. * c2), 0.];
            let c = [0., c2, c3];
            let bi = [[0., b[0]], [0., b[1]], [0., b[2]]];

            ButcherTableu {
                order: 2,
                order_embedded: 1,
                order_interpolant: 1,
                a: a.map(|row| row.map(|x| T::from_f64(x).unwrap())),
                b: b.map(|x| T::from_f64(x).unwrap()),
                b2: b2.map(|x| T::from_f64(x).unwrap()),
                c: c.map(|x| T::from_f64(x).unwrap()),
                bi: bi.map(|row| row.map(|x| T::from_f64(x).unwrap())),
            }
        } else {
            panic!("Provided arguments are incorrect.")
        }
    }

    pub fn kutta3() -> Self {
        Self::generic_order_3(0.5, 1., None)
    }
    pub fn heun3() -> Self {
        Self::generic_order_3(1. / 3., 2. / 3., None)
    }
    pub fn ralston3() -> Self {
        Self::generic_order_3(1. / 2., 3. / 4., None)
    }
    pub fn wray3() -> Self {
        Self::generic_order_3(8. / 15., 2. / 3., None)
    }
    pub fn ssp3() -> Self {
        Self::generic_order_3(1.0, 0.5, None)
    }
}

impl<T> ButcherTableu<T, 4, 2>
where
    T: RealField + Copy,
{
    // "The" Runge Kutta method
    pub fn rk4() -> Self {
        let a = [
            [0., 0., 0., 0.],
            [0.5, 0., 0., 0.],
            [0., 0.5, 0., 0.],
            [0., 0., 1., 0.],
        ];
        let b = [1. / 6., 1. / 3., 1. / 3., 1. / 6.];
        let b2 = [0., 1., 0., 0.];
        let c = [0., 0.5, 0.5, 1.];
        let bi = [[0., b[0]], [0., b[1]], [0., b[2]], [0., b[3]]];

        ButcherTableu {
            order: 4,
            order_embedded: 2,
            order_interpolant: 1,
            a: a.map(|row| row.map(|x| T::from_f64(x).unwrap())),
            b: b.map(|x| T::from_f64(x).unwrap()),
            b2: b2.map(|x| T::from_f64(x).unwrap()),
            c: c.map(|x| T::from_f64(x).unwrap()),
            bi: bi.map(|row| row.map(|x| T::from_f64(x).unwrap())),
        }
    }

    // 3/8 rule method
    pub fn three_eights() -> Self {
        let a = [
            [0., 0., 0., 0.],
            [1. / 3., 0., 0., 0.],
            [-1. / 3., 1., 0., 0.],
            [1., -1., 1., 0.],
        ];
        let b = [1. / 8., 3. / 8., 3. / 8., 1. / 8.];
        let b2 = [-0.5, 1.5, 0., 0.];
        let c = [0., 1. / 3., 2. / 3., 1.];
        let bi = [[0., b[0]], [0., b[1]], [0., b[2]], [0., b[3]]];

        ButcherTableu {
            order: 4,
            order_embedded: 2,
            order_interpolant: 1,
            a: a.map(|row| row.map(|x| T::from_f64(x).unwrap())),
            b: b.map(|x| T::from_f64(x).unwrap()),
            b2: b2.map(|x| T::from_f64(x).unwrap()),
            c: c.map(|x| T::from_f64(x).unwrap()),
            bi: bi.map(|row| row.map(|x| T::from_f64(x).unwrap())),
        }
    }
}

impl<T> ButcherTableu<T, 5, 4>
where
    T: RealField + Copy,
{
    pub fn rk43() -> Self {
        let a = [
            [0., 0., 0., 0., 0.],
            [0.5, 0., 0., 0., 0.],
            [0., 0.5, 0., 0., 0.],
            [0., 0., 1., 0., 0.],
            [5. / 32., 7. / 32., 13. / 32., -1. / 32., 0.],
        ];
        let b = [1. / 6., 1. / 3., 1. / 3., 1. / 6., 0.];
        let b2 = [-1. / 2., 7. / 3., 7. / 3., 13. / 6., -16. / 3.];
        let c = [0., 0.5, 0.5, 1., 0.75];
        let bi = [
            [0., 1., -1.5, 2. / 3.],
            [0., 0., 1., -2. / 3.],
            [0., 0., 1., -2. / 3.],
            [0., 0., -0.5, 2. / 3.],
            [0., 0., 0., 0.],
        ];

        ButcherTableu {
            order: 4,
            order_embedded: 3,
            order_interpolant: 3,
            a: a.map(|row| row.map(|x| T::from_f64(x).unwrap())),
            b: b.map(|x| T::from_f64(x).unwrap()),
            b2: b2.map(|x| T::from_f64(x).unwrap()),
            c: c.map(|x| T::from_f64(x).unwrap()),
            bi: bi.map(|row| row.map(|x| T::from_f64(x).unwrap())),
        }
    }
}

impl<T: RealField + Copy> ButcherTableu<T, 7, 5> {
    pub fn rktp64() -> Self {
        let mut a = [[0.; 7]; 7];
        a[1][0..1].copy_from_slice(&[0.14814814814814814814814814814814814814814814814815]);
        a[2][0..2].copy_from_slice(&[
            0.05555555555555555555555555555555555555555555555556,
            0.16666666666666666666666666666666666666666666666667,
        ]);
        a[3][0..3].copy_from_slice(&[
            0.19241982507288629737609329446064139941690962099125,
            -0.53134110787172011661807580174927113702623906705539,
            0.76749271137026239067055393586005830903790087463557,
        ]);
        a[4][0..4].copy_from_slice(&[
            0.27138264973958333333333333333333333333333333333333,
            -0.28179931640625000000000000000000000000000000000000,
            0.10191932091346153846153846153846153846153846153846,
            0.59599734575320512820512820512820512820512820512821,
        ]);
        a[5][0..5].copy_from_slice(&[
            -0.12140681348692272679528027730121494345436084170722,
            0.47761410187445690404270285927090660818471469359043,
            0.12192296968479080920271208457739825271063725338342,
            0.00820786686248269381285345905535723094994297948894,
            0.28289264429596155050624264362832208237829668447521,
        ]);
        a[6][0..6].copy_from_slice(&[
            0.32310946589106292966684294024325753569539925965098,
            -0.61039132734003172924378635642517186673717609730301,
            0.45846867541639319976612888047255080412929454343221,
            0.57505740806711566278133278660922926335638856572569,
            -0.57379234522267681781745625845989491691225880929347,
            0.82754812318813675484693800756002918046835253778760,
        ]);
        let b = [
            0.07277777777777777777777777777777777777777777777778,
            0.00000000000000000000000000000000000000000000000000,
            0.28752127070690503526324421846809906511399048712482,
            0.18974846220396832187710941882243328294496258901153,
            0.10581736348682550735167973519824811036097403449285,
            0.26909544328484081804764916719375922411975542905334,
            0.07503968253968253968253968253968253968253968253968,
        ];
        let b2 = [
            0.10322666047518118524035683798997408464864086165861,
            0.00000000000000000000000000000000000000000000000000,
            0.15611542056134071025476537742246069068941506933613,
            0.38634918851063907802720917783777796648011517818308,
            -0.12073095208684351320487107578989528150071079171750,
            0.40000000000000000000000000000000000000000000000000,
            0.07503968253968253968253968253968253968253968253968,
        ];
        let c = [
            0.00000000000000000000000000000000000000000000000000,
            0.14814814814814814814814814814814814814814814814815,
            0.22222222222222222222222222222222222222222222222222,
            0.42857142857142857142857142857142857142857142857143,
            0.68750000000000000000000000000000000000000000000000,
            0.76923076923076923076923076923076923076923076923077,
            1.00000000000000000000000000000000000000000000000000,
        ];
        let bi = [
            [
                0.00000000000000000000000000000000000000000000000000,
                1.00000000000000000000000000000000000000000000000000,
                -3.18888888888888888888888888888888888888888888888890,
                3.62962962962962962962962962962962962962962962962960,
                -1.36796296296296296296296296296296296296296296296300,
            ],
            [0., 0., 0., 0., 0.],
            [
                0.00000000000000000000000000000000000000000000000000,
                0.00000000000000000000000000000000000000000000000000,
                3.13664097096932917828440216499917992455305888141710,
                -4.87567656224372642283090044284074134820403477119900,
                2.02655686198130227980974249630966048876496637690670,
            ],
            [
                0.00000000000000000000000000000000000000000000000000,
                0.00000000000000000000000000000000000000000000000000,
                1.44306660772178013557323902151488358384910109048040,
                -2.68732193732193732193732193732193732193732193732190,
                1.43400379180412550824119233462948702103318343585310,
            ],
            [
                0.00000000000000000000000000000000000000000000000000,
                0.00000000000000000000000000000000000000000000000000,
                -1.84105678504031566306399039286326985760850917824670,
                5.43416252072968490878938640132669983416252072968490,
                -3.48728837220254373837371627326518186619303751694540,
            ],
            [
                0.00000000000000000000000000000000000000000000000000,
                0.00000000000000000000000000000000000000000000000000,
                0.00000000000000000000000000000000000000000000000000,
                0.00000000000000000000000000000000000000000000000000,
                0.26909544328484081804764916719375922411975542905334,
            ],
            [
                0.00000000000000000000000000000000000000000000000000,
                0.00000000000000000000000000000000000000000000000000,
                0.45023809523809523809523809523809523809523809523810,
                -1.50079365079365079365079365079365079365079365079370,
                1.12559523809523809523809523809523809523809523809520,
            ],
        ];

        ButcherTableu {
            order: 6,
            order_embedded: 4,
            order_interpolant: 4,
            a: a.map(|row| row.map(|x| T::from_f64(x).unwrap())),
            b: b.map(|x| T::from_f64(x).unwrap()),
            b2: b2.map(|x| T::from_f64(x).unwrap()),
            c: c.map(|x| T::from_f64(x).unwrap()),
            bi: bi.map(|row| row.map(|x| T::from_f64(x).unwrap())),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     fn interpolation_continuity_error<const S: usize, const S2: usize>(
//         rk: &ExplicitRungeKuttaTable<S, S2, f64>,
//     ) -> f64 {
//         let mut max = 0f64;
//         for i in 0..S {
//             max = max.max((rk.b[i] - (rk.bi[i].0)(1.)).abs());
//             max = max.max((rk.bi[i].0)(0.).abs());
//         }
//         max
//     }
//
//     fn c_is_sum_of_a_error<const S: usize, const S2: usize>(
//         rk: &ExplicitRungeKuttaTable<S, S2, f64>,
//     ) -> f64 {
//         let mut max = 0f64;
//         for i in 0..S {
//             let sum = ((i * i - i) / 2..).take(i).map(|j| rk.a[j]).sum::<f64>();
//             let diff = (rk.c[i] - sum).abs();
//             max = f64::max(max, diff);
//         }
//         max
//     }
//
//     fn order_conditions_error<const S: usize, const S2: usize>(
//         rk: &ExplicitRungeKuttaTable<S, S2, f64>,
//     ) -> f64 {
//         // On derivation and formulas for order conditions, see
//         // E. Hairer, G. Wanner, and S. P. Nørsett, Solving ordinary differential equations I, vol. 8. in Springer Series in Computational Mathematics, vol. 8. Berlin, Heidelberg: Springer Berlin Heidelberg, 1993. doi: 10.1007/978-3-540-78862-1.
//         let mut max_error = 0.;
//
//         // order is q in the mentioned text
//         for order in 1..=rk.order {
//             let advance_tree = |tree: &mut [usize]| -> bool {
//                 let mut i_of_increased = order - 1;
//
//                 // while cannot increase in this index
//                 while i_of_increased > 0 && tree[i_of_increased] + 1 >= i_of_increased {
//                     i_of_increased -= 1;
//                 }
//
//                 if i_of_increased > 0 {
//                     tree[i_of_increased] += 1;
//                     for i in (i_of_increased + 1)..order {
//                         tree[i] = tree[i_of_increased];
//                     }
//                     return true;
//                 } else {
//                     return false;
//                 }
//             };
//
//             let fix_indexes = |tree: &[usize], indexes: &mut [usize]| {
//                 for i in (1..order).rev() {
//                     indexes[tree[i]] = usize::max(indexes[tree[i]], indexes[i] + 1);
//                 }
//             };
//
//             let advance_indexes = |tree: &[usize], indexes: &mut [usize]| -> bool {
//                 let mut i_of_increased = order - 1;
//                 while i_of_increased > 0
//                     && indexes[i_of_increased] + 1 >= indexes[tree[i_of_increased]]
//                 {
//                     i_of_increased -= 1;
//                 }
//
//                 indexes[i_of_increased] += 1;
//                 for i in (i_of_increased + 1)..order {
//                     indexes[i] = 0;
//                 }
//                 for i in ((i_of_increased + 1)..order).rev() {
//                     indexes[tree[i]] = usize::max(indexes[tree[i]], indexes[i] + 1);
//                 }
//                 if i_of_increased > 0 {
//                     return true;
//                 } else {
//                     return false;
//                 }
//             };
//
//             let gamma = |tree: &[usize]| -> f64 {
//                 let mut rho = vec![0f64; order];
//                 for i in (1..order).rev() {
//                     rho[i] += 1.;
//                     rho[tree[i]] += rho[i];
//                 }
//                 rho[0] += 1.;
//
//                 rho.iter().product()
//             };
//
//             let mut tree = vec![0usize; rk.order];
//
//             // index[0] in 0..order, index[j] in 0..index[tree[j]]
//             let mut indexes = vec![0usize; rk.order];
//
//             loop
//             /* for all trees */
//             {
//                 fix_indexes(&tree, &mut indexes);
//
//                 let mut b_phi_sum = 0.;
//                 let mut b_phi_sum_embedded = 0.;
//
//                 println!("{:?}", &tree[..order]);
//                 for j in indexes[0]..S {
//                     // j = 0 => sums are empty
//                     indexes.fill(0);
//                     indexes[0] = j;
//                     fix_indexes(&tree, &mut indexes);
//
//                     let mut phi_j = 0.;
//
//                     loop
//                     /* for all index packs starting with j */
//                     {
//                         // println!("\t{:?}", indexes);
//                         phi_j += (1..order).fold(1., |acc, i| {
//                             acc * rk.a_indexed(indexes[tree[i]], indexes[i])
//                         });
//
//                         if !advance_indexes(&tree, &mut indexes) {
//                             indexes.fill(0);
//                             break;
//                         }
//                     }
//                     b_phi_sum += phi_j * rk.b[j];
//                     b_phi_sum_embedded += phi_j * rk.b2[j];
//                 }
//
//                 let error = f64::abs(b_phi_sum - 1. / gamma(&tree));
//                 let error_embedded = f64::abs(b_phi_sum_embedded - 1. / gamma(&tree));
//                 max_error = f64::max(max_error, error);
//                 if order <= rk.order_embedded {
//                     max_error = f64::max(max_error, error_embedded);
//                 }
//
//                 println!("\t{:?}", error);
//                 println!("\t{:?}", error_embedded);
//
//                 if !advance_tree(&mut tree) {
//                     break;
//                 }
//             }
//         }
//
//         max_error
//     }
//
//     macro_rules! test_rk {
//         ($name:ident, $tolerance:expr) => {
//             #[test]
//             fn $name() {
//                 let rk = super::$name::<f64>();
//                 // assert_a_has_correct_sizes(&$RK);
//                 let interpolatiion_continuinuity_error_val = interpolation_continuity_error(&rk);
//                 let c_is_sum_of_a_error_val = c_is_sum_of_a_error(&rk);
//                 let order_conditions_error_val = order_conditions_error(&rk);
//                 println!("tolerance: {}", $tolerance);
//                 println!(
//                     "interplation_error: {}",
//                     interpolatiion_continuinuity_error_val
//                 );
//                 println!("c_sum_error: {}", c_is_sum_of_a_error_val);
//                 println!("order_coditions_error: {}", order_conditions_error_val);
//                 assert!(interpolatiion_continuinuity_error_val < $tolerance);
//                 assert!(c_is_sum_of_a_error_val < $tolerance);
//                 assert!(order_conditions_error_val < $tolerance);
//             }
//         };
//     }
//
//     test_rk!(euler, 1e-15);
//
//     test_rk!(heun2, 1e-15);
//     test_rk!(ralston2, 1e-15);
//     test_rk!(midpoint, 1e-15);
//
//     test_rk!(ssp3, 1e-15);
//     test_rk!(heun3, 1e-15);
//     test_rk!(wray3, 1e-15);
//     test_rk!(kutta3, 1e-15);
//     test_rk!(ralston3, 1e-15);
//
//     test_rk!(classic4, 1e-15);
//     test_rk!(classic43, 1e-15);
//
//     // fails
//     // test_rk!(rk547fm, 1e-11);
//
//     test_rk!(rktp64, 1e-15);
//
//     // #[test]
//     // fn rk98_exept_order_conditions() {
//     //     // assert_a_has_correct_sizes(&RK98);
//     //     assert!(interpolation_continuity_error(&RK98) < 1e-9);
//     //     assert!(c_is_sum_of_a_error(&RK98) < 1e-9);
//     //     assert!(order_conditions_error(&RK98, [(); 5]) < 1e-7);
//     // }
//     //
//     // #[test]
//     // #[ignore] // because takes too long (1.5 hours for 7th order or smth)
//     // fn rk98_order_conditions() {
//     //     assert!(order_conditions_error(&RK98, [(); RK98.order]) < 1e-15);
//     // }
// }
