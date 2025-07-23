#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use diffurch::{Filter, Loc, Solver, event, event_mut, state_fn};

fn main() {
    let k = 0.90;
    let g = 9.8;

    let mut points = Vec::new();
    let mut points_continuous = Vec::new();

    Solver::new()
        .equation(state_fn!(|[_x, dx]| [dx, -g]))
        .max_delay(f64::INFINITY)
        .initial([1., -0.01])
        .interval(0. ..8.58)
        .on_step(event!(|t, [x, _dx]| (t, x)).to_vec(&mut points).to_std())
        .on_step(
            event!(|t, [x, _dx]| (t, x))
                .to_vec(&mut points_continuous)
                .separated_by(0.01)
                .subdivide(21),
        )
        .on(
            Loc::new(state_fn!(|[x, _dx]| x)).while_neg().bisection(),
            event_mut!(|t, [x, dx]| {
                *x = 0.;
                *dx = k * dx.abs();
                *t
            })
            .to_std(),
        )
        .run();

    let mut axis = pgfplots::axis::Axis::new();
    for (color, coords) in [("red", points), ("blue", points_continuous)] {
        let mut plot = pgfplots::axis::plot::Plot2D::new();
        plot.coordinates = coords.into_iter().map(|p| p.into()).collect();
        plot.add_key(pgfplots::axis::plot::PlotKey::Custom(color.to_string()));
        axis.plots.push(plot);
    }
    pgfplots::Picture::from(axis)
        .show_pdf(pgfplots::Engine::PdfLatex)
        .unwrap();
}
