#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use diffurch::*;

fn main() {
    let k = 1f64;
    let tau = 1f64;
    let a = k / (k * tau).tan();
    let b = -k / (k * tau).sin();

    let sol = |t: f64| (k * t).sin();

    let mut t = vec![];
    let mut x = vec![];

    Solver::new()
        .equation(state_fn!(|t, [x], [x_]| [a * x + b * x_(t - tau)]))
        .initial(|t: f64| [sol(t)])
        .interval(0. ..10.)
        .stepsize(0.33)
        .on_step(
            event!(|t, [x]| [t, x])
                .to_vecs([&mut t, &mut x])
                .subdivide(5),
        )
        .on_step(Event::ode2_state().to_std())
        .on_step(event!(|t, [x]| [t, x, x - sol(t)]).to_std())
        .run();

    let mut plot = pgfplots::axis::plot::Plot2D::new();
    plot.coordinates = (0..t.len()).map(|i| (t[i], x[i]).into()).collect();
    pgfplots::Picture::from(plot)
        .show_pdf(pgfplots::Engine::PdfLatex)
        .unwrap();
}
