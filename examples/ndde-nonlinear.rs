#![allow(incomplete_features)]
#![feature(generic_const_exprs)]


use core::f64::consts::PI;

use diffurch::*;

const A: f64 = -1.;
const T: f64 = 1.;

fn solution(epsilon: f64, alpha: f64, beta: f64) {
    let equation =
        equation!(|t, [x], [x_]| [-x + (1. + epsilon) * x_.d(t - T) + A * x_.d(t - T).powi(3)]);
    let ic = (
        |t: f64| [alpha * (beta * t).sin()],
        |t: f64| [alpha * beta * (beta * t).cos()],
    );
    let range = 0. ..30.;
    //
    let mut t = Vec::new();
    let mut x = Vec::new();
    //
    Solver::new()
        .stepsize(0.05)
        .on_step(event!(|t, [x]| (t, x)).to_std().separated_by(0.99))
        .on_step(
            event!(|t, [x]| [t, x])
                .to_vecs([&mut t, &mut x])
                .subdivide(5), // .in_range((range.end - T)..range.end),
        )
        .run(equation, ic, range);

    let mut plot = pgfplots::axis::plot::Plot2D::new();
    plot.coordinates = (0..t.len()).map(|i| (t[i], x[i]).into()).collect();
    pgfplots::Picture::from(plot)
        .show_pdf(pgfplots::Engine::PdfLatex)
        .unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1))
}

fn main() {
    solution(0.2, 0.01, 2. * PI);
}
