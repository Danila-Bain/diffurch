// const ROOT: &str = env!("CARGO_MANIFEST_DIR");
// const MODULE: &str = module_path!();

use diffurch::*;
use nalgebra::*;
use derive_more::*;

#[test]
fn lorenz_lyapunov_exponents() {

    let sigma = 10.;
    let rho = 28.;
    let beta = 8. / 3.;


    #[derive(Clone, Copy, Debug, Sub, AddAssign, Add, Mul, Div, Neg, Default)]
    struct State {
        state: Vector3<f64>,
        variation: Matrix3<f64>,
    }
    impl num_traits::identities::Zero for State {
        fn zero() -> Self {
            Self {
                state: num_traits::identities::zero(),
                variation: num_traits::identities::zero(),
            }
        }

        fn is_zero(&self) -> bool {
            self.state.is_zero() && self.variation.is_zero()
        }
    }
    impl traits::RealVectorSpace<f64> for State {}

    // ecommons.cornell.edu/server/api/core/bitstreams/c0790d83-7dd3-44e9-964a-cb878542708d/content
    let reference_lambdas = vector![0.90566, 0.00000, -14.57233];

    for tmax in [100., 1000., 10_000., 100_000.] {
        let mut lambdas = vector![0., 0., 0.];

        Solver::new::<f64, State>()
            .stepsize((0.005))
            .initial(State {
                state: vector![10., 15., 20.],
                variation: Matrix3::identity(),
            })
            .interval(0. ..tmax)
            .equation(|s| {
                let [x, y, z] = s.y.state.into();

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
                
                State{
                    state: f,
                    variation: df * s.y.variation
                }
            })
            .on_loc_mut(
                Periodic {
                    period: 0.5,
                    offset: 0.,
                },
                |s| {
                    let (q, r) = s.y.variation.qr().unpack();
                    s.y.variation.copy_from(&q);
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
