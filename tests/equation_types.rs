#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use diffurch::{initial_condition::InitFn, *};
use nalgebra::*;

#[test]
fn constant() {
    let solution = |t: f64| vector![t, -t, 0.];
    Solver::new::<f64, Vector3<f64>>()
        .rk(RK::euler())
        .stepsize(0.25)
        .initial([0., 0., 0.])
        .interval(0. ..10.)
        .equation(|_| vector![1., -1., 0.])
        .on_step(|state| assert_eq!(*state.p, solution(state.t)))
        .run();
}

#[test]
fn time() {
    let solution = |t: f64| vector![0., t, t * t / 2.];

    let mut tt = vec![];
    let mut xx = vec![];

    Solver::new()
        .interval(0. ..10.)
        .rk(RK::midpoint())
        .stepsize(0.5)
        .initial([0., 0., 0.])
        .equation(|&StateRef { t, .. }| vector![0., 1., t])
        .on_step(|&StateRef { t, p: &x, .. }| {
            tt.push(t);
            xx.push(x)
        })
        .run();

    for (t, x) in tt.into_iter().zip(xx.into_iter()) {
        assert_eq!(x, solution(t));
    }
}

#[test]
fn time2() {
    let solution = |t: f64| vector![0., t, t * t / 2.];

    let mut tt = vec![];
    let mut xx = vec![];

    Solver::new()
        .initial([0., 0., 0.])
        .interval(0. ..10.)
        .rk(RK::midpoint())
        .stepsize(0.5)
        .equation(|state| vector![0., 1., state.t])
        .on_step(|&StateRef { t, p: &x, .. }| {
            tt.push(t);
            xx.push(x)
        })
        .run();

    for (t, x) in tt.into_iter().zip(xx.into_iter()) {
        assert_eq!(x, solution(t));
    }
}

#[test]
fn ode_exponent() {
    let solution = |t: f64| (-t).exp();

    Solver::new::<f64, f64>()
        .initial(1.)
        .interval(0. ..10.)
        .rk(RK::rktp64())
        .stepsize(0.05)
        .equation(|state| -state.p)
        .on_step(|s| assert!((s.p - solution(s.t)).abs() < 1e-14))
        .run();
}

#[test]
fn ode_harmonic() {
    let solution = |t: f64| t.sin();

    Solver::new::<f64, Vector2<f64>>()
        .rk(RK::rktp64())
        .stepsize(0.04)
        .initial([0., 1.])
        .equation(|&StateRef { p: &pos, .. }| {
            let [x, dx] = pos.into();
            vector![dx, -x]
        })
        .interval(0. ..10.)
        .on_step(|state| assert!((state.p.x - solution(state.t)).abs() < 1e-13))
        .run();
}

#[test]
fn odet_lin() {
    let sol = |t: f64| t.powi(-2);

    Solver::new::<f64, f64>()
        .rk(RK::rktp64())
        .stepsize(0.02)
        .interval(1. ..10.)
        .initial(InitFn(sol, ()))
        .equation(|s| -2. * s.p / s.t)
        .on_step(|&StateRef { t, p: y, .. }| {
            // println!("{}", ((x - sol(t))/sol(t)).abs());
            assert!((y - sol(t)).abs() < 1e-13)
        })
        .run();
}

#[test]
fn dde_sin() {
    let k = 1.;
    let tau: f64 = 1.;

    let a = k / (k * tau).tan();
    let b = -k / (k * tau).sin();

    let sol = |t: f64| (k * t).sin();

    Solver::new::<f64, f64>()
        // .delay(tau)
        .max_delay(tau)
        .initial(InitFn(sol, ()))
        .interval(0. ..10.)
        .stepsize(0.02)
        .equation(|s| a * s.p + b * s.p(s.t - tau))
        .on_step(|s| {
            assert!((s.p - sol(s.t)).abs() < 1e-11);
        })
        .run();
}

#[test]
fn ndde_sin() {
    let k: f64 = 1.;
    let tau: f64 = 1.;

    let a = -k * (k * tau).sin();
    let b = (k * tau).cos();

    let sol = |t: f64| (k * t).sin();

    Solver::new::<f64, f64>()
        .initial(InitFn(|t: f64| (k * t).sin(), |t: f64| k * (k * t).cos()))
        .equation(|s| a * s.p(s.t - tau) + b * s.d(s.t - tau))
        .interval(0. ..10.)
        .max_delay(tau)
        .stepsize(0.02)
        .on_step(|s| {
            assert!((s.p - sol(s.t)).abs() < 1e-11);
        })
        .run();
}
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
//          .stepsize((0.05))
//         .on(when_hit, bounce)
//         .on_step(event!(|t, [x, _]| assert!((x - sol(t)).abs() < 1e-13)))
//         .run();
// }
