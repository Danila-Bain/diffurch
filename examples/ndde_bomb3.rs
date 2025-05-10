use std::cell::Cell;

use core::f64::consts::PI;
use diffurch::*;

fn main() {
    let alpha = 0.15;
    let beta = 2. * PI;

    const T: f64 = 1.;
    const A: f64 = 1.;
    let epsilon = Cell::new(0.5);
    let k = 1.;

    let eq = Equation::dde(|t, [x], [x_]| {
        let dx_ = x_.d(t - T);
        let dx = -x + (1. + epsilon.get()) * dx_ + A * dx_.powi(3);
        [dx]
    })
    .with_delay(2.*T);
    let ic = (
        |t: f64| [alpha * (beta * t).sin()],
        |t: f64| [alpha * beta * (beta * t).cos()],
    );

    let stepsize = 1. / (beta * 10.).round();

    Solver::rk(&rk::RK98)
        .stepsize(stepsize)
        .on_step(Event::ode_state().to_std())
        .on_step(
            Event::dde(|t, [x], [x_]| {
                epsilon.set(epsilon.get() * k);
                let mut amp = 0f64;
                let n = 50;
                for i in 0..n {
                    let t = t - T*(i as f64)/(n as f64);
                    amp = amp.max(x_(t).abs());
                }
                [t, epsilon.get(), x, amp]
            })
            .separated_by(100.)
            .to_std(),
        )
        .on_step(Event::stop_integration().filter_by_constant(|| epsilon.get() <= 0.1))
        .run(eq, ic, 0. .. 10000.)
}
