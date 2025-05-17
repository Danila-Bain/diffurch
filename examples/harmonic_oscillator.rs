// fn main() {
//     todo!()
// }
use diffurch::*;

fn main() {
    let k = 0.5;
    let eq = Equation::ode(|[x, dx]| [dx, -k * k * x]);
    let ic = |t: f64| [(t * k).sin(), k * (t * k).cos()];
    let sol = |t: f64| (t * k).sin();
    let range = 0. ..50.;

    let mut points = Vec::new();

    Solver::rk(&rk::RK98)
        .stepsize(1.)
        .on_step(
            event!(|t, [x, _dx]| (t, x))
                .subdivide(10)
                .to_vec(&mut points),
        )
        .on_step(event!(|t, [x, dx]| (t, x, dx, (x - sol(t)).abs())).to_std())
        .run(eq, ic, range);

    let mut plot = pgfplots::axis::plot::Plot2D::new();
    plot.coordinates = points.into_iter().map(|p| p.into()).collect();
    pgfplots::Picture::from(plot)
        .show_pdf(pgfplots::Engine::PdfLatex)
        .unwrap();
}
