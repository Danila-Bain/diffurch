// const ROOT: &str = env!("CARGO_MANIFEST_DIR");
// const MODULE: &str = module_path!();

// use derive_more::*;
use derive_state::State;
use diffurch::*;
use nalgebra::*;

#[test]
fn lorenz_lyapunov_exponents() {
    let sigma = 10.;
    let rho = 28.;
    let beta = 8. / 3.;

    #[derive(State)]
    struct State {
        state: Vector3<f64>,
        variation: Matrix3<f64>,
    }

    // ecommons.cornell.edu/server/api/core/bitstreams/c0790d83-7dd3-44e9-964a-cb878542708d/content
    let reference_lambdas = vector![0.90566, 0.00000, -14.57233];

    for tmax in [100., 1000., 10_000., 100_000.] {
        let mut lambdas = vector![0., 0., 0.];

        Solver::new::<f64, State>()
            .stepsize(0.005)
            .initial(State {
                state: vector![10., 15., 20.],
                variation: Matrix3::identity(),
            })
            .interval(0. ..tmax)
            .equation(|StateRef { p, .. }| {
                let [x, y, z] = p.state.into();

                let f = vector![
                    sigma * (y - x),   //
                    x * (rho - z) - y, //
                    x * y - beta * z,  //
                ];

                let df = matrix![
                    -sigma, sigma, 0.;
                    rho - z, -1., -x;
                    y, x, -beta;
                ];

                State {
                    state: f,
                    variation: df * p.variation,
                }
            })
            .on_mut(
                Periodic {
                    period: 0.5,
                    offset: 0.,
                },
                |s| {
                    let (q, r) = s.p.variation.qr().unpack();
                    s.p.variation.copy_from(&q);
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
