#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use diffurch::*;

fn main() {
    let k: f64 = 1.;
    let tau: f64 = 0.5;

    // let a = -k * (k * tau).tan();
    // let b = 1. / (k * tau).cos();

    let equation = equation!(|t, [_x], [x_]| [
        -k * x_(t - tau) * (k * tau).sin() + x_.d(t - tau) * (k * tau).cos()
    ]);

    let ic = (|t: f64| [(k * t).sin()], |t: f64| [k * (k * t).cos()]);
    let sol = |t: f64| (k * t).sin();
    let range = 0. ..110.;

    let mut t = Vec::new();
    let mut x = Vec::new();

    Solver::new()
        .rk(&rk::RK98)
        .stepsize(0.1)
        .on_step(
            event!(|t, [x]| [t, x])
                .to_vecs([&mut t, &mut x])
                .separated_by(0.05),
        )
        .on_step(event!(|t, [x]| [t, x, x - sol(t)]).to_std())
        .run(equation, ic, range);

    let mut plot = pgfplots::axis::plot::Plot2D::new();
    plot.coordinates = (0..t.len()).map(|i| (t[i], x[i]).into()).collect();
    pgfplots::Picture::from(plot)
        .show_pdf(pgfplots::Engine::PdfLatex)
        .unwrap();
}
