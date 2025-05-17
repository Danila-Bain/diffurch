use diffurch::*;

fn main() {

    // equation of steady motion on a plain
    let eq = Equation::ode(|[_x, _y, dx, dy]| [dx, dy, 0., 0.]);

    let ic = [0., 0.1, 0.3, 0.4];
    let range = 0. ..; // infinite range, stop integration from event

    let mut points = Vec::new();

    let mut counter = 0;

    Solver::rk(&rk::RK98)
        .stepsize(0.5)
        // .on_step(Event::ode(|[x, y, _dx, _dy]| (x, y)).to_vec(&mut points))
        // .on_step(Event::ode2_state().to_std())
        .on_loc(
            Loc::to_pos(StateFn::ode(|[x, y, _dx, _dy]| {
                x.powi(2) + y.powi(2) - y.powi(3)/3. - 1. // zero set is the boundary
            })),
            event_mut!(|t, [x, y, dx, dy]| {
                // gradient of the boundary function
                let x_normal = 2. * *x; 
                let y_normal = 2. * *y - y.powi(2);

                // reflection formula:
                // (*dx, *dy) changes to a vector of the same
                // magnitude and the same dot product with normal
                let h = x_normal.powi(2) + y_normal.powi(2);
                let k1 = -(x_normal.powi(2) - y_normal.powi(2)) / h;
                let k2 = - 2. * x_normal * y_normal / h;
                (*dx, *dy) = (k1 * *dx + k2 * *dy, k2 * *dx - k1 * *dy);

                counter += 1;
                if counter == 1000 {
                    // stop integration after 1000th bounce
                    *t = f64::INFINITY; 
                }
                (*x, *y)
            })
            .to_vec(&mut points),
        )
        .run(eq, ic, range);

    // plotting with pgfplots
    let mut axis = pgfplots::axis::Axis::new();
    let mut plot = pgfplots::axis::plot::Plot2D::new();
    plot.coordinates = points.into_iter().map(|p| p.into()).collect();
    plot.add_key(pgfplots::axis::plot::PlotKey::Custom("violet".to_string()));
    plot.add_key(pgfplots::axis::plot::PlotKey::Custom("line width=0.1pt".to_string()));
    axis.plots.push(plot);
    pgfplots::Picture::from(axis)
        .show_pdf(pgfplots::Engine::PdfLatex)
        .unwrap();
}
