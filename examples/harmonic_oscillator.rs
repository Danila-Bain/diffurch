// use diffurch::state::*;
use diffurch::equation::Equation;
use diffurch::event::Event;
use diffurch::rk;
use diffurch::solver::Solver;

fn main() {
    let k = 1.;

    let eq = Equation::ode(|[x, dx]| [dx, -k * k * x]);

    let ic = |t: f64| [(t * k).sin(), k * (t * k).cos()];
    let range = 0. ..20.;

    // let mut points = Vec::new();
    //
    // let mut t = Vec::new();
    // let mut x = Vec::new();
    // let mut dx = Vec::new();
    //
    // let mut max_radius_deviation = 0f64;
    // let mut max_true_solution_deviation = 0f64;
    //
    // Solver::new()
    //     .rk(&rk::RK98)
    //     .stepsize(0.05)
    //     .on_step(Event::new(|t: f64, [x, _dx]: [f64; 2]| (t, x)).to_vec(&mut points))
    //     .on_step(
    //         Event::new(|t: f64, [x, dx]: [f64; 2]| [t, x, dx]).to_vecs([&mut t, &mut x, &mut dx]),
    //     )
    //     .on_step(Event::new(|[x, dx]: [f64; 2]| (x, dx)).to_std())
    //     .on_step(
    //         Event::new(|t: f64, [x, dx]: [f64; 2]| (t, x, dx)).save(|(t, x, dx)| {
    //             max_radius_deviation = max_radius_deviation.max((x.powi(2) + dx.powi(2)) - 1.);
    //             max_true_solution_deviation = max_true_solution_deviation.max({
    //                 let [true_x, true_dx] = ic(t);
    //                 (x - true_x).powi(2) + (dx - true_dx).powi(2)
    //             })
    //         }),
    //     )
    //     .run(eq, ic, range);
    //
    // println!("Max deviation in radius: {}", max_radius_deviation);
    // println!("Global error: {}", max_true_solution_deviation.sqrt());
    //
    // let mut plot = pgfplots::axis::plot::Plot2D::new();
    // plot.coordinates = points.into_iter().map(|p| p.into()).collect();
    // pgfplots::Picture::from(plot)
    //     .show_pdf(pgfplots::Engine::PdfLatex)
    //     .unwrap();
}
