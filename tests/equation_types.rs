#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use core::f64;

use diffurch::*;
use StateRef as S;

#[test]
fn constant() {
    let solution = |t: f64| [t, -t, 0.];

    Solver::new()
        .equation(StateFn::new(|&S { .. }| [1., -1., 0.]))
        .initial([0., 0., 0.])
        .interval(0. ..10.)
        .stepsize(0.25)
        .rk(rk::euler())
        .on_step(StateFn::new(|&S { t, x, .. }| {
            assert_eq!(x, solution(t))
        }))
        .run();
}

#[test]
fn time() {
    let solution = |t: f64| [0., t, t * t / 2.];

    let mut tt = vec![];
    let mut xx = vec![];

    Solver::new()
        .equation(StateFn::new(|&S { t, .. }| [0., 1., t]))
        .initial([0., 0., 0.])
        .interval(0. ..10.)
        .rk(rk::midpoint())
        .stepsize(0.5)
        .on_step(StateFn::new(|&S { t, x, .. }| {
            tt.push(t);
            xx.push(x)
        }))
        .run();

    for (t, x) in tt.into_iter().zip(xx.into_iter()) {
        assert_eq!(x, solution(t));
    }
}

#[test]
fn ode_exponent() {
    let solution = |t: f64| (-t).exp();

    Solver::new()
        .equation(StateFn::new(|&S { x: [x], .. }| [-1. * x]))
        .initial([1.])
        .interval(0. ..10.)
        .rk(rk::rktp64())
        .stepsize(0.05)
        .on_step(StateFn::new(|&S { t, x: [x], .. }| {
            assert!((x - solution(t)).abs() < 1e-14)
        }))
        .run();
}

#[test]
fn ode_harmonic() {
    let solution = |t: f64| t.sin();

    Solver::new()
        .equation(StateFn::new(|&S { x: [x, dx], .. }| [dx, -1. * x]))
        .interval(0. ..10.)
        .initial([0., 1.])
        .rk(rk::rktp64())
        .stepsize(0.04)
        .on_step(StateFn::new(|&S { t, x: [x, _], .. }| {
            // println!("{} =?= {}", x, solution(t));
            assert!((x - solution(t)).abs() < 1e-13)
        }))
        .run();
}

#[test]
fn odet_lin() {
    let sol = |t: f64| t.powi(-2);

    Solver::new()
        .equation(StateFn::new(|&S { t, x: [x], .. }| [- 2. * x / t]))
        .initial(|t: f64| [sol(t)])
        .interval(1. ..10.)
        .rk(rk::rktp64())
        .stepsize(0.02)
        .on_step(StateFn::new(|&S { t, x: [x], .. }| {
            // println!("{}", ((x - sol(t))/sol(t)).abs());
            assert!((x - sol(t)).abs() < 1e-13)
        }))
        .run();
}

// #[test]
// fn dde_sin() {
//     let k = 1.;
//     let tau: f64 = 1.;
//
//     let a = k / (k * tau).tan();
//     let b = -k / (k * tau).sin();
//
//     let sol = |t: f64| (k * t).sin();
//
//     Solver::new()
//         .equation(state_fn!(|t, [x], [x_]| [a * x + b * x_(t - tau)]))
//         .delay(tau)
//         .initial(|t: f64| [(k * t).sin()])
//         .interval(..10.)
//         .rk(&rk::RK98)
//         .stepsize(0.33)
//         .on_step(event!(|t, [x]| assert!((x - sol(t)).abs() < 1e-11)))
//         .run();
// }
//
// #[test]
// fn ndde_sin() {
//     let k: f64 = 1.;
//     let tau: f64 = 1.;
//
//     let a = -k * (k * tau).sin();
//     let b = (k * tau).cos();
//
//     let sol = |t: f64| (k * t).sin();
//
//     Solver::new()
//         .equation(state_fn!(
//             |t, [_x], [x]| [a * x(t - tau) + b * x.d(t - tau)]
//         ))
//         .initial((|t: f64| [(k * t).sin()], |t: f64| [k * (k * t).cos()]))
//         .interval(..10.)
//         .rk(&rk::RK98)
//         .stepsize(0.25)
//         .on_step(event!(|t, [x]| assert!((x - sol(t)).abs() < 1e-10)))
//         .run();
// }
//
// #[test]
// fn bouncing_ball() {
//     let sol = |t: f64| {
//         let t = t % 2.;
//         t * (2. - t)
//     };
//
//     let when_hit = Loc::new(state_fn!(|[x, _]| x)).sign().bisection();
//     let bounce = event_mut!(|[x, dx]| {
//         *x = 0.;
//         *dx = dx.abs();
//     });
//
//     Solver::new()
//         .equation(state_fn!(|[_x, dx]| [dx, -2.]))
//         .max_delay(f64::INFINITY)
//         .initial([0., 2.])
//         .interval(0. ..10.)
//         .rk(&rk::RK98)
//         .stepsize(0.05)
//         .on(when_hit, bounce)
//         .on_step(event!(|t, [x, _]| assert!((x - sol(t)).abs() < 1e-13)))
//         .run();
// }
