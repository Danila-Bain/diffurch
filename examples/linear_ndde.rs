use diffurch::{Equation, Event, Solver, rk, util::with_derivative::WithDerivative};

fn main() {
    // let theta = 0.5f64;
    let k: f64 = 1.;
    let tau: f64 = 0.5;

    let a = -k * (k * tau).tan();
    let b = 1. / (k * tau).cos();

    let equation = Equation::dde(|t, [x], [x_]| [a * x + b * x_.d(t - tau)]);
    let ic = move |t: f64| [(k * t).sin()];
    let ic = (ic, move |t: f64| [k * (k * t).cos()]);
    let range = 0. ..110.;

    let mut t = Vec::new();
    let mut x = Vec::new();

    Solver::rk(&rk::RK98)
        .stepsize(0.1)
        .on_step(
            Event::ode2(|t, [x]| [t, x])
                .to_vecs([&mut t, &mut x])
                .separated_by(0.05),
        )
        .on_step(
            Event::ode2({
                let sol = ic.0.clone();
                move |t, [x]| [t, x, x - sol(t)[0]]
            })
            .to_std(),
        )
        .run(equation, ic, range);

    println!("a={a}, b={b}");

    let mut plot = pgfplots::axis::plot::Plot2D::new();
    plot.coordinates = (0..t.len()).map(|i| (t[i], x[i]).into()).collect();
    pgfplots::Picture::from(plot)
        .show_pdf(pgfplots::Engine::PdfLatex)
        .unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1))
}
