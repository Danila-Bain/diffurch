use std::time::Duration;

use diffurch::{rk, Equation, Event, Solver, State};

fn main() {
    let k = 1.;

    let eq = Equation::ode(move |[x, dx]: [f64; 2]| [dx, -k * k * x]);

    let ic = move |t: f64| [(t * k).sin(), k * (t * k).cos()]; // argument can be inffered, if
                                                               // closure is typed in the argument

    let range = 0. ..20.;

    // let mut e = Event::new(|t: f64| (t, t-1.)).to_std();
    //
    // let mut s = State::new(0., ic, &rk::RK98);
    //
    // let mut e = e.to_state_function();
    // e(&s);

    let mut points = Vec::new();

    // let mut t = Vec::new();
    // let mut x = Vec::new();
    // let mut dx = Vec::new();
    //
    // let mut max_radius_deviation = 0f64;
    // let mut max_true_solution_deviation = 0f64;

    Solver::new()
        .rk(&rk::RK98)
        .stepsize(0.05)
        .on_step(Event::ode2(|t, [x, _dx]| (t, x)).in_range(10. .. 11.).to_std())
        .on_step(Event::new(|| "Hello").separated_by(0.99).to_std())
        .on_step(Event::ode2(|t, [x, _dx]| (t, x)).to_vec(&mut points))
        // .on_step(Event::ode2(|t: f64, [x, _dx]: [f64; 2]| (t, x)).to_std())
        // .on_step(
        //     Event::ode2(|t, [x, dx]| [t, x, dx]).to_vecs([&mut t, &mut x, &mut dx]),
        // )
        // .on_step(
        //     Event::ode2(|t, [x, dx]| {
        //         let [xx, dxx] = ic(t);
        //         (x, dx, f64::max((x - xx).abs(), (dx - dxx).abs()))
        //     })
        //     .to_std(),
        // )
        // .on_step(Event::ode2(|t, [x, dx]| (t, x, dx)).to(
        //     |(t, x, dx): (f64, f64, f64)| {
        //         max_radius_deviation = max_radius_deviation.max((x.powi(2) + dx.powi(2)) - 1.);
        //         max_true_solution_deviation = max_true_solution_deviation.max({
        //             let [true_x, true_dx] = ic(t);
        //             (x - true_x).powi(2) + (dx - true_dx).powi(2)
        //         })
        //     },
        // ))
        .run(eq, ic, range);
    //
    // println!("Max deviation in radius: {}", max_radius_deviation);
    // println!("Global error: {}", max_true_solution_deviation.sqrt());
    //
    // let mut plot = pgfplots::axis::plot::Plot2D::new();
    // plot.coordinates = points.into_iter().map(|p| p.into()).collect();
    // pgfplots::Picture::from(plot)
    //     .show_pdf(pgfplots::Engine::PdfLatex)
    //     .unwrap();
    //
    // std::thread::sleep(Duration::from_secs(1));
}
