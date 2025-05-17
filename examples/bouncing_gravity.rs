use std::cell::Cell;

use diffurch::{Loc, Solver, StateFn, equation, event, event_mut, rk};

fn main() {
    let k = 0.9;
    let g = Cell::new(9.8);
    let eq = equation!(|[_x, dx]| [dx, -g.get()]).with_delay(f64::INFINITY);

    let ic = [1., -0.01];
    let range = 0. ..8.58;

    let mut points = Vec::new();
    let mut points_continuous = Vec::new();

    Solver::rk(&rk::RK98)
        .stepsize(0.5)
        .on_step(
            event!(|t, [x, _dx]| (t, x)).to_vec(&mut points).to_std(), // .separated_by(0.01)
                                                                       // .subdivide(21)
        )
        .on_step(
            event!(|t, [x, _dx]| (t, x))
                .to_vec(&mut points_continuous)
                // .to_std()
                .separated_by(0.01)
                .subdivide(21),
        )
        .on_loc(Loc::zero(StateFn::ode(|[_x, dx]| dx)), event!())
        .on_loc(
            Loc::zero(StateFn::ode(|[x, _dx]| x)),
            event_mut!(|t, [_x, dx]| {
                *dx *= k;
                g.set(g.get() * -1.);
                *t
            })
            .to_std(),
        )
        .run(eq, ic, range);

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
