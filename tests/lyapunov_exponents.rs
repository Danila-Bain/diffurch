// #![feature(split_array)]

// const ROOT: &str = env!("CARGO_MANIFEST_DIR");
// const MODULE: &str = module_path!();

use diffurch::{StateFn, StateRef, *};
// use nalgebra::{ArrayStorage, Matrix3};

#[test]
fn linear_1_const() {
    fn the_lyapunov_exponent(eigenvalue: f64, tmax: f64) -> f64 {
        let equation = StateFn::new(|&StateRef { x: [x], .. }| [eigenvalue * x]);

        let mut lambda = 0.;

        Solver::new()
            .initial([1.])
            .interval(0. ..tmax)
            .equation(equation)
            .on((
                Periodic {
                    period: 0.3,
                    offset: 0.,
                },
                StateFn::new_mut(|&mut StateRefMut::<f64, 1> { x: [x], .. }| {
                    let norm = x.abs();
                    *x /= norm;
                    lambda += norm.ln();
                }),
            ))
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
        let equation =
            StateFn::new(|&StateRef { t, x: [x], .. }| [(1. + f64::sin(t)) * eigenvalue * x]);

        let mut lambda = 0.;

        Solver::new()
            .initial([1.])
            .interval(0. ..tmax)
            .equation(equation)
            .on((
                Periodic {
                    period: 1.3,
                    offset: 0.,
                },
                StateFn::new_mut(|&mut StateRefMut::<f64, 1> { x: [x], .. }| {
                    let norm = x.abs();
                    *x /= norm;
                    lambda += norm.ln();
                }),
            ))
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

        Solver::new()
            .initial([
                1., 0., 0., //
                0., 1., 0., //
                0., 0., 1., //
            ])
            .interval(0. ..tmax)
            .equation(StateFn::new(|&StateRef::<f64, 9> { x, .. }| {
                let v = Matrix3::from_column_slice(x);
                let mut f = [0.; 9];
                f.copy_from_slice((a_matrix * v).as_slice());
                return f;
            }))
            .on((
                Periodic {
                    period: 1.,
                    offset: 0.,
                },
                StateFn::new_mut(|&mut StateRefMut::<f64, 9> { ref mut x, .. }| {
                    let v = Matrix3::from_column_slice(&x[..]);
                    let (q, r) = v.qr().unpack();
                    x.copy_from_slice(q.as_slice());
                    lambdas += r.diagonal().map(|r| r.ln());
                }),
            ))
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

        Solver::new()
            .initial([
                1., 0., 0., //
                0., 1., 0., //
                0., 0., 1., //
            ])
            .interval(0. ..tmax)
            .equation(StateFn::new(|&StateRef::<f64, 9> { x, .. }| {
                let v = Matrix3::from_column_slice(x);
                let mut f = [0.; 9];
                f.copy_from_slice((a_matrix * v).as_slice());
                return f;
            }))
            .on((
                Periodic {
                    period: 1.,
                    offset: 0.,
                },
                StateFn::new_mut(|&mut StateRefMut::<f64, 9> { ref mut x, .. }| {
                    let v = Matrix3::from_column_slice(&x[..]);
                    let (q, r) = v.qr().unpack();
                    x.copy_from_slice(q.as_slice());
                    lambdas += r.diagonal().map(|r| r.ln());
                }),
            ))
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
