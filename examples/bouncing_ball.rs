
use diffurch::{rk, Equation, Event, EventLocator, Solver, StateFn};

fn main() {
    let k = 0.90;
    let g = 9.8;
    let eq = Equation::ode(|[_x, dx]| [dx, -g]).with_delay(f64::INFINITY);

    let ic = [20., 0.];
    let range = 0. ..50.;

    let mut points = Vec::new();

    Solver::rk(&rk::RK98)
        .stepsize(0.05)
        .on_step(Event::ode2(|t, [x, _dx]| (t, x)).to_vec(&mut points).to_std())
        .on(
            EventLocator {
                detection: diffurch::Detection::SignNeg(StateFn::ODE(Box::new(|[x, _]| x))),
                location: diffurch::LocationMethod::Bisection,
            },
            Event::ode2_mut(|t, [x, dx]| {
                *x = 0.;
                *dx = k*dx.abs();
                *t
            }).to_std(),
        )
        .run(eq, ic, range);

    //
    // println!("Max deviation in radius: {}", max_radius_deviation);
    // println!("Global error: {}", max_true_solution_deviation.sqrt());
    //
    let mut plot = pgfplots::axis::plot::Plot2D::new();
    plot.coordinates = points.into_iter().map(|p| p.into()).collect();
    pgfplots::Picture::from(plot)
        .show_pdf(pgfplots::Engine::PdfLatex)
        .unwrap();
    //
    //     std::thread::sleep(Duration::from_secs(1));
}
