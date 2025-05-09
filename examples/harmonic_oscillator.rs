// fn main() {
//     todo!()
// }
use diffurch::{Equation, Event, Solver, rk, InitialCondition};

fn main() {
    let k = 0.5;

    let eq = Equation::ode(|[x, dx]| [dx, -k * k * x]).with_delay(6.);

    let ic = InitialCondition::Function(Box::new(move |t: f64| [(t * k).sin(), k * (t * k).cos()]));
    let solution =move |t: f64| [(t * k).sin(), k * (t * k).cos()];

    let range = 0. ..50.;

    let mut points = Vec::new();

    // let mut t = Vec::new();
    // let mut x = Vec::new();
    // let mut dx = Vec::new();
    //
    // let mut max_radius_deviation = 0f64;
    // let mut max_true_solution_deviation = 0f64;

    Solver::rk(&rk::RK98)
        .stepsize(1.)
        // .on_step(
        //     Event::ode2(|t, [x, _]| (t, x))
        //         .to_vec(&mut points)
        //         .subdivide(10),
        // )
        // .on_step(Event::new(|| "Hello").separated_by(0.99).to_std())
        .on_step(Event::ode2(|t, [x, _dx]| (t, x)).to_vec(&mut points))
        // .on_step(Event::ode2(|t: f64, [x, _dx]: [f64; 2]| (t, x)).to_std())
        // .on_step(
        //     Event::ode2(|t, [x, dx]| [t, x, dx]).to_vecs([&mut t, &mut x, &mut dx]),
        // )
        .on_step(
            Event::ode2(|t, [x, dx]| {
                let [xx, dxx] = solution(t);
                (t, x, dx, f64::max((x - xx).abs(), (dx - dxx).abs()))
            })
            // .subdivide(10)
            .to_std(),
        )
        // .on_step(Event::new(|| "Step finished").to_std())
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
    let mut plot = pgfplots::axis::plot::Plot2D::new();
    plot.coordinates = points.into_iter().map(|p| p.into()).collect();
    pgfplots::Picture::from(plot)
        .show_pdf(pgfplots::Engine::PdfLatex)
        .unwrap();
//
//     std::thread::sleep(Duration::from_secs(1));
}
