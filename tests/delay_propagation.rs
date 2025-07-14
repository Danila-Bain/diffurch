#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use diffurch::*;

#[test]
fn constant_1() {
    let eq = equation!(|| [0.]).const_neutral_delay(1.);
    let ic = [0.];

    let mut ts = vec![];

    Solver::new()
        .rk(&rk::EULER)
        .stepsize(0.75)
        .on_step(event!(|t| ts.push(t)))
        .run(eq, ic, 0. ..5.);

    assert_eq!(
        ts,
        vec![0.0, 0.75, 1.0, 1.75, 2.0, 2.75, 3.0, 3.75, 4.0, 4.75, 5.0]
    )
}

#[test]
fn constant_2() {
    let eq = equation!(|| [0.])
        .const_neutral_delay(1.)
        .const_neutral_delay(1.25);
    let ic = [0.];

    let mut ts = vec![];

    let stepsize = 7. / 8.;
    Solver::new()
        .rk(&rk::EULER)
        .stepsize(stepsize)
        .on_step(event!(|t| ts.push(t)))
        .run(eq, ic, 0. ..5.);

    assert_eq!(
        ts,
        vec![
            0.0, stepsize, 1.0, 1.25, 2., 2.25, 2.5, 3.0, 3.25, 3.5, 3.75, 4., 4.25, 4.5, 4.75, 5.0
        ]
    )
}

#[test]
fn constant_1_smoothing() {
    let eq = equation!(|| [0.]).const_delay(1.);
    let ic = [0.];

    let mut ts = vec![];

    Solver::new()
        .rk(&rk::CLASSIC4)
        .stepsize(0.75)
        .on_step(event!(|t| ts.push(t)))
        .run(eq, ic, 0. ..6.);

    assert_eq!(
        ts,
        vec![0., 0.75, 1., 1.75, 2., 2.75, 3., 3.75, 4.5, 5.25, 6.]
    )
}


#[test]
fn pantograph() {
    let eq = equation!(|| [0.]).neutral_delay(state_fn!(|t| t/2.));
    let mut ts = vec![];

    let stepsize = 0.123456789101112;

    Solver::new()
        .rk(&rk::CLASSIC4)
        .stepsize(stepsize)
        .on_step(event!(|t| ts.push(t)))
        .run(eq, [0.], 1. ..2f64.powi(10));

    for i in 1..=10 {
        assert!(ts.contains(&2f64.powi(i)))
    }
    // assert_eq!(
    //     ts,
    //     vec![0., 0.75, 1., 1.75, 2., 2.75, 3., 3.75, 4.5, 5.25, 6.]
    // )
}
