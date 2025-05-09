#![feature(test)]
extern crate test;

use diffurch::{Equation, Event, Solver, rk};
use test::Bencher;

const RANGE: std::ops::Range<f64> = 0. ..100.;
const STEPSIZE: f64 = 0.05;

fn get_eq() -> (
    Equation<'static, 2>,
    impl Fn(f64) -> [f64; 2],
    std::ops::Range<f64>,
) {
    let k = 1.;
    let eq = Equation::ode(move |[x, dx]| [dx, -k * k * x]);
    let ic = move |t: f64| [(t * k).sin(), k * (t * k).cos()];
    let range = RANGE;
    (eq, ic, range)
}

fn get_eq_dyn() -> (
    Equation<'static, 2>,
    impl Fn(f64) -> [f64; 2],
    std::ops::Range<f64>,
) {
    let k = 1.;

    let rhs: Box<dyn Fn([f64; 2]) -> [f64; 2]> = Box::new(move |[x, dx]| [dx, -k * k * x]);

    let eq = Equation::ode(rhs);
    let ic = move |t: f64| [(t * k).sin(), k * (t * k).cos()];
    let range = RANGE;

    (eq, ic, range)
}

#[bench]
fn base(b: &mut Bencher) {
    b.iter(|| {
        let (eq, ic, range) = get_eq();
        Solver::rk(&rk::RK98).stepsize(STEPSIZE).run(eq, ic, range);
    })
}

#[bench]
fn base_dyn(b: &mut Bencher) {
    b.iter(|| {
        let (eq, ic, range) = get_eq_dyn();
        Solver::rk(&rk::RK98).stepsize(STEPSIZE).run(eq, ic, range);
    })
}

#[bench]
fn with_delay(b: &mut Bencher) {
    b.iter(|| {
        let (eq, ic, range) = get_eq();
        Solver::rk(&rk::RK98)
            .stepsize(STEPSIZE)
            .run(eq.with_delay(10.), ic, range);
    })
}

#[bench]
fn with_delay_infinite(b: &mut Bencher) {
    b.iter(|| {
        let (eq, ic, range) = get_eq();
        Solver::rk(&rk::RK98)
            .stepsize(STEPSIZE)
            .run(eq.with_delay(f64::MAX), ic, range);
    })
}

#[bench]
fn var_output(b: &mut Bencher) {
    b.iter(|| {
        let (eq, ic, range) = get_eq();
        let mut var = (0., 0.);
        Solver::rk(&rk::RK98)
            .stepsize(STEPSIZE)
            .on_step(Event::ode2(|t, [x, _]| (t, x)).to_var(&mut var))
            .run(eq, ic, range);
        var
    })
}

#[bench]
fn vec_output(b: &mut Bencher) {
    b.iter(|| {
        let (eq, ic, range) = get_eq();
        let mut vec = Vec::new();
        Solver::rk(&rk::RK98)
            .stepsize(STEPSIZE)
            .on_step(Event::ode2(|t, [x, _]| (t, x)).to_vec(&mut vec))
            .run(eq, ic, range);
        vec
    })
}

#[bench]
fn std_output(b: &mut Bencher) {
    b.iter(|| {
        let (eq, ic, range) = get_eq();
        Solver::rk(&rk::RK98)
            .stepsize(STEPSIZE)
            .on_step(Event::ode2(|t, [x, _]| (t, x)).to_std())
            .run(eq, ic, range);
    })
}

#[bench]
fn subdivide_5(b: &mut Bencher) {
    b.iter(|| {
        let (eq, ic, range) = get_eq();
        let mut var = (0., 0.);
        Solver::rk(&rk::RK98)
            .stepsize(STEPSIZE)
            .on_step(
                Event::ode2(|t, [x, _]| (t, x))
                    .to_var(&mut var)
                    .subdivide(5),
            )
            .run(eq, ic, range);
        var
    })
}

#[bench]
fn subdivide_compensate_2(b: &mut Bencher) {
    b.iter(|| {
        let (eq, ic, range) = get_eq();
        let mut var = (0., 0.);
        let n = 2;
        Solver::rk(&rk::RK98)
            .stepsize(STEPSIZE * n as f64)
            .on_step(
                Event::ode2(|t, [x, _]| (t, x))
                    .to_var(&mut var)
                    .subdivide(n),
            )
            .run(eq, ic, range);
        var
    })
}

#[bench]
fn subdivide_compensate_5(b: &mut Bencher) {
    b.iter(|| {
        let (eq, ic, range) = get_eq();
        let mut var = (0., 0.);
        let n = 5;
        Solver::rk(&rk::RK98)
            .stepsize(STEPSIZE * n as f64)
            .on_step(
                Event::ode2(|t, [x, _]| (t, x))
                    .to_var(&mut var)
                    .subdivide(n),
            )
            .run(eq, ic, range);
        var
    })
}

#[bench]
fn subdivide_compensate_10(b: &mut Bencher) {
    b.iter(|| {
        let (eq, ic, range) = get_eq();
        let mut var = (0., 0.);
        let n = 10;
        Solver::rk(&rk::RK98)
            .stepsize(STEPSIZE * n as f64)
            .on_step(
                Event::ode2(|t, [x, _]| (t, x))
                    .to_var(&mut var)
                    .subdivide(n),
            )
            .run(eq, ic, range);
        var
    })
}

#[bench]
fn subdivide_compensate_20(b: &mut Bencher) {
    b.iter(|| {
        let (eq, ic, range) = get_eq();
        let mut var = (0., 0.);
        let n = 20;
        Solver::rk(&rk::RK98)
            .stepsize(STEPSIZE * n as f64)
            .on_step(
                Event::ode2(|t, [x, _]| (t, x))
                    .to_var(&mut var)
                    .subdivide(n),
            )
            .run(eq, ic, range);
        var
    })
}

fn get_eq_delay() -> (
    Equation<'static, 2>,
    impl Fn(f64) -> [f64; 2],
    std::ops::Range<f64>,
) {
    let k = 1.;
    let eq = Equation::dde(move |t, [_x, dx], [x_, _]| [dx, -k * k * x_(t - 1.)]).with_delay(2.);
    let ic = move |t: f64| [(t * k).sin(), k * (t * k).cos()];
    let range = RANGE;
    (eq, ic, range)
}

#[bench]
fn eq_with_delay(b: &mut Bencher) {
    b.iter(|| {
        let (eq, ic, range) = get_eq_delay();
        Solver::rk(&rk::RK98).stepsize(STEPSIZE).run(eq, ic, range);
    })
}
