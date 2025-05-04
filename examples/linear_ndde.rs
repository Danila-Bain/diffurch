
use diffurch::{rk, Equation, Event, Solver};

fn main() {
    // let theta = 0.5f64;
    let k : f64 = 0.5;
    let tau : f64 = 0.5;

    let a = -k * (k * tau).tan();
    let b = 1. / (k * tau).cos();

    let equation = Equation::dde(|t, [x], [x_]| [a * x + b * x_.d(t - tau)]);
    let ic = move |t: f64| [(k * t).sin()];
    let range = 0. .. 10.;

    let mut t = Vec::new();
    let mut x = Vec::new();

    Solver::new()
        .stepsize(0.05)
        .rk(&rk::RK98)
        .on_step(Event::dde(|t, [x], [x_]| [t, x_.d(t-tau)]).to_vecs([&mut t, &mut x]))
        .on_step(Event::ode2(|t, [x]| [t, x, x - ic(t)[0]]).to_std())
        .run(equation, ic, range);

    println!("a={a}, b={b}");

    let mut plot = pgfplots::axis::plot::Plot2D::new();
    plot.coordinates = (0..t.len()).map(|i| (t[i], x[i]).into()).collect();
    pgfplots::Picture::from(plot)
        .show_pdf(pgfplots::Engine::PdfLatex)
        .unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1))
}

