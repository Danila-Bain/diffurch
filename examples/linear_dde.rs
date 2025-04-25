use diffurch::equation::Equation;
use diffurch::event::Event;
use diffurch::rk;
use diffurch::solver::Solver;

fn main() {
    let theta = 0.5f64;
    let k = 1f64;
    let tau = 1f64;

    let a = k / (k * tau).tan();
    let b = -k / (k * tau).sin();

    let equation = Equation::dde(|t, [x], [x_]| [a * x + b * x_(t - tau)]);
    let ic = |t: f64| [(k * t).sin()];
    let range = 0. .. 20.;

    // let mut t = Vec::new();
    // let mut x = Vec::new();
    //
    // Solver::new()
    //     .stepsize(0.01)
    //     .rk(&rk::DP544)
    //     .on_step(Event::new(|t: f64, [x]: [f64; 1]| [t, x]).to_vecs([&mut t, &mut x]))
    //     .run(equation, ic, range);

    
    //
    //
    // let mut plot = pgfplots::axis::plot::Plot2D::new();
    // plot.coordinates = (0..t.len()).map(|i| (t[i], x[i]).into()).collect();
    // pgfplots::Picture::from(plot)
    //     .show_pdf(pgfplots::Engine::PdfLatex)
    //     .unwrap();
}
