// use diffurch::state::*;
use diffurch::equation::Equation;
use diffurch::event::Event;
use diffurch::rk;
use diffurch::solver::Solver;

fn main() {
    let k = 1.;

    let eq = Equation::new(|_t: f64, [x, dx]: [f64; 2]| [dx, -k * k * x]);

    let ic = |t: f64| [(t * k).sin(), k * (t * k).cos()];
    let range = 0. ..10.;

    // let mut solver = solver
    // .on_step(Event::new(|t: f64, [x, dx]: [f64; 2]| (t, x, dx)).to_std())
    // ;

    // let solver = solver.on_step(Event::new(|t, [x, dx]| (t, x, dx)).to_std());
    let mut points = Vec::new();

    let mut t = Vec::new();
    let mut x = Vec::new();
    let mut dx = Vec::new();

    Solver::new()
        .rk(&rk::RK98)
        .stepsize(0.05)
        .on_step(Event::new(|t: f64, [x, dx]: [f64; 2]| (t, dx)).to_vec(&mut points))
        .on_step(Event::new(|[x, dx]: [f64; 2]| x * x + dx * dx).to_std())
        .on_step(
            Event::new(|t: f64, [x, dx]: [f64; 2]| [t, x, dx]).to_vecs([&mut t, &mut x, &mut dx]),
        )
        .run(eq, ic, range);

    // println!("{:?}", &points);
    let mut plot = pgfplots::axis::plot::Plot2D::new();
    plot.coordinates = points.into_iter().map(|p| p.into()).collect();
    pgfplots::Picture::from(plot)
        .show_pdf(pgfplots::Engine::PdfLatex)
        .unwrap();
}
