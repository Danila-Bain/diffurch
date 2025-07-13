#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use diffurch::*;

#[test]
fn constant_1() {
    let eq = equation!(|| [0.]).const_delay(1.);
    let ic = [0.];

    let mut ts = vec![];

    Solver::new()
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
    let eq = equation!(|| [0.]).const_delay(1.).const_delay(1.25);
    let ic = [0.];

    let mut ts = vec![];

    Solver::new()
        .stepsize(0.625)
        .on_step(event!(|t| ts.push(t)))
        .run(eq, ic, 0. ..15.);

    assert_eq!(
        ts,
        vec![0.0, 0.625, 1.0, 1.25, 1.25 + 0.625, 2., 2.25, 2.5, 3.0, 3.5, 3.75, 4., 4.25, 4.5, 4.75, 5.0]
    )
}
