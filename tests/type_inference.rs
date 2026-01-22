use diffurch::*;
use nalgebra::*;

#[test]
fn no_arguments() {
    Solver::new()
        .interval(0. ..0.)
        .rk(rk::ButcherTableu::euler())
        .stepsize(0.25)
        .initial([0., 0., 0.])
        .equation(|&StateRef { .. }| vector![1., -1., 0.])
        .run();
}

#[test]
fn t_argument_arythmetic() {
    Solver::new()
        .interval(0. ..0.)
        .rk(rk::ButcherTableu::euler())
        .stepsize(0.25)
        .initial([0., 0., 0.])
        .equation(|&StateRef { t, .. }| vector![t, t * 2., (t + 1.) / t])
        .run();
}
