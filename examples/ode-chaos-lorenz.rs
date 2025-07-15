#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

fn main() {
    // system parameters
    let sigma = 10.;
    let rho = 28.;
    let beta = 8. / 3.;

    // equation and initial conditions
    let eq =
        diffurch::equation!(|[x, y, z]| [sigma * (y - x), x * (rho - z) - y, x * y - beta * z]);
    let ic = [1., 2., 20.];
    let interval = 0. ..50.;

    // output variables
    let mut t = Vec::new();
    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut z = Vec::new();


    // run solver
    diffurch::Solver::new()
        .on_step(
            diffurch::event!(|t, [x, y, z]| [t, x, y, z])
                .subdivide(4) // dense output: save 4 points per step for smoother plot
                .to_vecs([&mut t, &mut x, &mut y, &mut z]) // save values to individual `Vec<f64>`s
                .to_std(), // additionally output values to console
        )
        .run(eq, ic, interval);

    // plot with pgfplots
    let mut axis = pgfplots::axis::Axis::new();
    let mut plot = pgfplots::axis::plot::Plot2D::new();
    plot.coordinates = x.into_iter().zip(z.into_iter()).map(|p| p.into()).collect();
    plot.add_key(pgfplots::axis::plot::PlotKey::Custom("smooth".to_string()));
    axis.plots.push(plot);
    pgfplots::Picture::from(axis)
        .show_pdf(pgfplots::Engine::PdfLatex)
        .unwrap();
}
