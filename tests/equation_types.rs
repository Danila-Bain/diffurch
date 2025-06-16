#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use diffurch::*;

#[test]
fn constant() {
    let eq = equation!(|| [1., -1., 0.]);
    let ic = [0., 0., 0.];
    let solution = |t: f64| [t, -t, 0.];

    Solver::new()
        .rk(&rk::RK98)
        .stepsize(0.25)
        .on_step(event!(|t, [x, y, z]| assert_eq!([x, y, z], solution(t))))
        .run(eq, ic, 0. ..10.);
}

#[test]
fn time() {
    let eq = equation!(|t| [0., 1., t]);
    let ic = [0., 0., 0.];
    let solution = |t: f64| [0., t, t * t / 2.];

    Solver::new()
        .rk(&rk::RK98)
        .stepsize(0.5)
        .on_step(event!(|t, [x, y, z]| assert_eq!([x, y, z], solution(t))))
        .run(eq, ic, 0. ..10.);
}

#[test]
fn ode_exponent() {
    let eq = equation!(|[x]| [-x]);
    let ic = [1.];
    let solution = |t: f64| (-t).exp();

    Solver::new()
        .rk(&rk::RK98)
        .stepsize(0.1)
        .on_step(event!(|t, [x]| {
            assert!((x - solution(t)).abs() < 1e-14)
        }))
        .run(eq, ic, 0. ..10.);
}

#[test]
fn ode_harmonic() {
    let eq = equation!(|[x, dx]| [dx, -x]);
    let ic = [0., 1.];
    let solution = |t: f64| t.sin();

    Solver::new()
        .rk(&rk::RK98)
        .stepsize(0.1)
        .on_step(event!(|t, [x, _dx]| {
            assert!((x - solution(t)).abs() < 1e-13)
        }))
        .run(eq, ic, 0. ..10.);
}

#[test]
fn ode2_lin() {
    let eq = equation!(|t, [x]| [2. * x / t]);
    let ic = |t: f64| [t * t];
    let sol = |t: f64| t * t;

    Solver::new()
        .rk(&rk::RK98)
        .stepsize(0.1)
        .on_step(event!(|t, [x]| assert!((x - sol(t)).abs() < 1e-11)))
        .run(eq, ic, 1. ..10.);
}

#[test]
fn dde_sin() {
    let k = 1.;
    let tau: f64 = 1.;

    let a = k / (k * tau).tan();
    let b = -k / (k * tau).sin();

    let eq = equation!(|t, [x], [x_]| [a * x + b * x_(t - tau)]);
    let ic = |t: f64| [(k * t).sin()];
    let sol = |t: f64| (k * t).sin();

    Solver::new()
        .rk(&rk::RK98)
        .stepsize(0.33)
        .on_step(event!(|t, [x]| assert!((x - sol(t)).abs() < 1e-11)))
        .run(eq, ic, ..10.);
}

#[test]
fn ndde_sin() {
    let k: f64 = 1.;
    let tau: f64 = 1.;

    let a = -k * (k * tau).sin();
    let b = (k * tau).cos();

    let eq = equation!(|t, [_x], [x]| [a * x(t - tau) + b * x.d(t - tau)]);
    let ic = (|t: f64| [(k * t).sin()], |t: f64| [k * (k * t).cos()]);
    let sol = |t: f64| (k * t).sin();

    Solver::new()
        .rk(&rk::RK98)
        .stepsize(0.25)
        .on_step(event!(|t, [x]| assert!((x - sol(t)).abs() < 1e-10)))
        .run(eq, ic, ..10.);
}

#[test]
fn bouncing_ball() {
    let eq = equation!(|[_x, dx]| [dx, -2.]).with_delay(f64::INFINITY);

    let ic = [0., 2.];
    let range = 0. ..10.;

    let sol = |t: f64| {
        let t = t % 2.;
        t * (2. - t)
    };

    let when_hit = Loc::neg(state_fn!(|[x, _]| x));
    let bounce = event_mut!(|[x, dx]| {
        *x = 0.;
        *dx = dx.abs();
    });

    Solver::new()
        .rk(&rk::RK98)
        .stepsize(0.05)
        .on_loc(when_hit, bounce)
        .on_step(event!(|t, [x, _]| assert!((x - sol(t)).abs() < 1e-13)))
        .run(eq, ic, range);
}
