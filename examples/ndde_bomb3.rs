#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use core::f64::consts::PI;
use diffurch::*;
use std::cell::Cell;

fn main() {
    const T: f64 = 1.;
    const A: f64 = -1.;

    let alpha = 0.15;
    let beta = 2. * PI / T;

    let epsilon = Cell::new(0.5);
    let k = 0.95;

    let eq = Equation::dde(|t, [x], [x_]| {
        let dx_ = x_.d(t - T);
        let dx = -x + (1. + epsilon.get()) * dx_ + A * dx_.powi(3);
        [dx]
    })
    .with_delay(2. * T);
    let ic = (
        |t: f64| [alpha * (beta * t).sin()],
        |t: f64| [alpha * beta * (beta * t).cos()],
    );

    let mut epsilons = Vec::new();
    let mut amps = Vec::new();
    // let mut counts = Vec::new();

    let stepsize = 1. / (beta * 10.).round();
    println!("stepsize: {stepsize}");

    Solver::rk(&rk::RK98)
        .stepsize(stepsize)
        // .on_step(
        //     Event::ode2_state()
        //         .to_std()
        //         .to_vecs([&mut t, &mut x])
        //         .separated_by(0.),
        // )
        .on_step(
            Event::dde(|t, [_], [x_]| {
                let mut amp = 0f64;
                let n = 200;
                for i in 0..n {
                    let t = t - T * (i as f64) / (n as f64);
                    amp = amp.max(x_(t).abs());
                }
                println!("t={t}");
                let eps = epsilon.get();
                epsilon.set(eps*k);
                // epsilon.set(eps - amp.powi(2)*2.);
                [eps, amp]
            })
            .separated_by(10000.).times(1..)
            .to_std()
            .to_table("out.dat", " ", Some("eps amp"))
            .to_vecs([&mut epsilons, &mut amps]),
        )
        .on_step(Event::stop_integration().filter_by_constant(|| epsilon.get() <= 0.001))
        .run(eq, ic, 0. .. 1_000_000.);

    let mut axis = pgfplots::axis::Axis::new();

    let points = epsilons.iter().zip(amps.iter()).map(|(eps, amp)| (*eps, *amp).into()).collect();
    for (color, coords) in [("blue", points)] {
        let mut plot = pgfplots::axis::plot::Plot2D::new();
        plot.coordinates = coords;
        plot.add_key(pgfplots::axis::plot::PlotKey::Custom(color.to_string()));
        axis.plots.push(plot);
    }

    pgfplots::Picture::from(axis)
        .show_pdf(pgfplots::Engine::PdfLatex)
        .unwrap();
}
