#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use diffurch::*;

#[test]
fn constant_1() {
    let eq = equation!(|| [0.]).const_delay(1.);
    let ic = [0.];

    Solver::new()
        .stepsize(0.9)
        .on_step(event!(|t| println!("{t}")))
        .run(eq, ic, 0. ..10.);

    panic!("asdf");
}
