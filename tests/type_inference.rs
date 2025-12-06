use diffurch::*;

#[test]
fn no_arguments() {
    Solver::new()
        .equation(StateFn::new(|&StateRef { .. }| [1., -1., 0.]))
        .initial([0., 0., 0.])
        .interval(0. ..0.)
        .stepsize(0.25)
        .rk(rk::euler())
        .run();
}

#[test]
fn t_argument_arythmetic() {
    Solver::new()
        .equation(StateFn::new(|&StateRef { t, .. }| [t, t * 2., (t + 1.)/t]))
        .initial([0., 0., 0.])
        .interval(0. ..0.)
        .stepsize(0.25)
        .rk(rk::euler())
        .run();
}
