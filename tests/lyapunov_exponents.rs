// #![feature(split_array)]

// const ROOT: &str = env!("CARGO_MANIFEST_DIR");
// const MODULE: &str = module_path!();

use diffurch::{StateFn, StateRef, *};
// use nalgebra::{ArrayStorage, Matrix3};

#[test]
fn linear_1_const() {
    fn the_lyapunov_exponent(eigenvalue: f64, tmax: f64) -> f64 {
        let mut lambda = 0.;

        Solver::new::<f64, f64>()
            .initial(1.)
            .interval(0. ..tmax)
            .equation(|state| state.y * eigenvalue)
            .on_loc_mut(
                Periodic {
                    period: 0.3,
                    offset: 0.,
                },
                |s| {
                    let norm = s.y.abs();
                    *s.y /= norm;
                    lambda += norm.ln();
                },
            )
            .run();
        return lambda / tmax;
    }
    assert!(0. == the_lyapunov_exponent(0., 100.));
    for tmax in [10., 100., 1000., 10_000., 100_000., 200_000.] {
        for true_lambda in [-5., -4., -3., -2., -1., 1., 2., 3., 4., 5.] {
            let computed_lambda = the_lyapunov_exponent(true_lambda, tmax);
            dbg!(
                tmax,
                true_lambda,
                computed_lambda,
                true_lambda - computed_lambda
            );
            assert!((true_lambda - computed_lambda).abs() < 2. / tmax);
        }
    }
}

#[test]
fn linear_1_sin() {
    fn the_lyapunov_exponent(eigenvalue: f64, tmax: f64) -> f64 {
        let mut lambda = 0.;

        Solver::new::<f64, f64>()
            .initial(1.)
            .interval(0. ..tmax)
            .equation(|s| (1. + f64::sin(s.t)) * eigenvalue * s.y)
            .on_loc_mut(
                Periodic {
                    period: 1.3,
                    offset: 0.,
                },
                |s| {
                    let norm = s.y.abs();
                    *s.y /= norm;
                    lambda += norm.ln();
                },
            )
            .run();
        return lambda / tmax;
    }
    assert!(0. == the_lyapunov_exponent(0., 100.));
    for tmax in [10., 100., 1000., 10_000., 100_000., 200_000.] {
        for true_lambda in [-5., -4., -3., -2., -1., 1., 2., 3., 4., 5.] {
            let computed_lambda = the_lyapunov_exponent(true_lambda, tmax);
            dbg!(
                tmax,
                true_lambda,
                computed_lambda,
                true_lambda - computed_lambda
            );
            assert!((true_lambda - computed_lambda).abs() < 10. / tmax);
        }
    }
}

#[test]
fn linear_const_3_real() {
    use nalgebra::*;

    fn the_lyapunov_exponents(eigenvalues: [f64; 3], tmax: f64) -> [f64; 3] {
        // random matrix
        let c_matrix = nalgebra::Matrix3::new(
            1., 3., -2., //
            4., 2., -1., //
            -1., 1., -5., //
        );
        let mut j_matrix = Matrix3::from_diagonal(&Vector3::from_row_slice(&eigenvalues));

        if j_matrix[(0, 0)] == j_matrix[(1, 1)] {
            j_matrix[(0, 1)] = 1.;
        }
        if j_matrix[(1, 1)] == j_matrix[(2, 2)] {
            j_matrix[(1, 2)] = 1.;
        }

        let a_matrix = c_matrix * j_matrix * c_matrix.try_inverse().unwrap();

        dbg!(j_matrix, a_matrix);

        let mut lambdas = Vector3::from_row_slice(&[0., 0., 0.]);

        Solver::new::<f64, Matrix3<f64>>()
            .initial(matrix![
                1., 0., 0.; //
                0., 1., 0.; //
                0., 0., 1.; //
            ])
            .interval(0. ..tmax)
            .equation(|s| a_matrix * s.y)
            .on_loc_mut(
                Periodic {
                    period: 1.,
                    offset: 0.,
                },
                |s| {
                    let (q, r) = s.y.qr().unpack();
                    *s.y = q;
                    lambdas += r.diagonal().map(|r| r.ln());
                },
            )
            .run();

        lambdas /= tmax;

        return lambdas.as_slice().try_into().unwrap();
    }
    for tmax in [10., 100., 1000., 10_000., 100_000., 200_000.] {
        for true_lambdas in [
            [2., 0., -2.],
            [3., 1., -2.],
            [5., 3., -2.],
            [3., 2., 1.],
            [2., 2., 1.],
            [4., -1., -1.],
            [3., 2., 2.],
            [0., 0., 0.],
        ] {
            let computed_lambdas = the_lyapunov_exponents(true_lambdas, tmax);

            let norm = (Vector3::from_row_slice(&true_lambdas)
                - Vector3::from_row_slice(&computed_lambdas))
            .amax();

            dbg!(tmax, true_lambdas, computed_lambdas, norm);

            let tolerance =
                if true_lambdas[0] == true_lambdas[1] || true_lambdas[1] == true_lambdas[2] {
                    40.
                } else {
                    2.
                };
            assert!(norm < tolerance / tmax);
        }
    }
}

#[test]
fn linear_const_3_complex() {
    use nalgebra::*;

    fn the_lyapunov_exponents(l1: f64, (re, im): (f64, f64), tmax: f64) -> [f64; 3] {
        // random matrix
        let c_matrix = matrix![
            1., -3., -2.;
            4., -2., -1.;
            -1., 1., -5.;
        ];
        let j_matrix = matrix!(
            l1, 0., 0.;
            0., re, -im;
            0., im, re;
        );

        let a_matrix = c_matrix * j_matrix * c_matrix.try_inverse().unwrap();

        dbg!(j_matrix, a_matrix);

        let mut lambdas = Vector3::from_row_slice(&[0., 0., 0.]);

        Solver::new::<f64, Matrix3<f64>>()
            .initial(matrix![
                1., 0., 0.; //
                0., 1., 0.; //
                0., 0., 1.; //
            ])
            .interval(0. ..tmax)
            .equation(|s| a_matrix * s.y)
            .on_loc_mut(
                Periodic {
                    period: 1.,
                    offset: 0.,
                },
                |s| {
                    let (q, r) = s.y.qr().unpack();
                    *s.y = q;
                    lambdas += r.diagonal().map(|r| r.abs().ln())
                },
            )
            .run();

        lambdas /= tmax;

        return lambdas.as_slice().try_into().unwrap();
    }
    for tmax in [10., 100., 1000., 10_000., 100_000., 200_000.] {
        for (l1, (re, im)) in [
            (4., (-3., 0.)),
            (-1., (5., -5.)),
            (0., (-2., 3.)),
            (2., (1., -4.)),
            (-5., (0., 2.)),
            (3., (4., -1.)),
            (-4., (-5., 5.)),
            (1., (3., -2.)),
            (5., (2., 4.)),
            (-2., (-1., 1.)),
        ] {
            let true_lambdas = if l1 > re { [l1, re, re] } else { [re, re, l1] };

            let computed_lambdas = the_lyapunov_exponents(l1, (re, im), tmax);

            let norm = (Vector3::from_row_slice(&true_lambdas)
                - Vector3::from_row_slice(&computed_lambdas))
            .amax();

            dbg!(tmax, computed_lambdas, norm);

            assert!(norm < 2. / tmax);
        }
    }
}

#[test]
fn lorenz_lyapunov_exponents() {
    use nalgebra::*;

    let sigma = 10.;
    let rho = 28.;
    let beta = 8. / 3.;

    // ecommons.cornell.edu/server/api/core/bitstreams/c0790d83-7dd3-44e9-964a-cb878542708d/content
    let reference_lambdas = vector![0.90566, 0.00000, -14.57233];

    for tmax in [100., 1000., 10_000., 100_000.] {
        let mut lambdas = vector![0., 0., 0.];

        Solver::new::<f64, Matrix3x4<f64>>()
            .stepsize(0.005)
            .initial(matrix![
                10., 1., 0., 0.;
                15., 0., 1., 0.;
                20., 0., 0., 1.;
            ])
            .interval(0. ..tmax)
            .equation(|s| {
                let mut diff = Matrix3x4::<f64>::zeros();
                let [x, y, z] = s.y.column(0).into();
                let var = s.y.fixed_columns::<3>(1);
                diff.set_column(
                    0,
                    &vector![
                        sigma * (y - x),   //
                        x * (rho - z) - y, //
                        x * y - beta * z,  //
                    ],
                );
                let df = matrix![
                    -sigma, sigma, 0.;
                    rho - z, -1., -x;
                    y, x, -beta;
                ];
                diff.fixed_columns_mut::<3>(1).copy_from(&(df * var));
                diff
            })
            .on_loc_mut(
                Periodic {
                    period: 0.5,
                    offset: 0.,
                },
                |s| {
                    let mut var = s.y.fixed_columns_mut::<3>(1);
                    let (q, r) = var.clone_owned().qr().unpack();
                    var.copy_from(&q);
                    lambdas += r.diagonal().map(|r| r.abs().ln())
                },
            )
            .run();

        lambdas /= tmax;

        let error = (lambdas - reference_lambdas).amax();
        dbg!(
            tmax,
            lambdas,
            reference_lambdas,
            lambdas - reference_lambdas,
            error
        );
        // 0.00007 is expected error of reference values
        assert!(error < 0.00007 + 50. / tmax);
    }
}
