fn main() {
    let k = f32::ln(2.);
    let interval = 0. ..16.;
    let true_solution = |t: f32| (k * t).exp();

    let mut points = vec![];

    diffurch::Solver::new::<f32, f32>()
        // .rk(diffurch::RK::rktp64()) // the default
        .interval(interval.clone())
        .initial(1f32)
        .equation(|s| k * s.p)
        .stepsize(1.)
        .on_step(|s| points.push((s.t, *s.p)))
        .on_step(|s| {
            let (t, x) = (s.t, s.p);
            let err = x - true_solution(t);
            let rel_err = err / x;
            print!("t={t:<2.0}, ");
            print!("x={x:<8.2}, ");
            print!("abs error={err:<5.0e}, ");
            print!("rel error={rel_err:<5.0e}");
            println!();
        })
        .run();

    // we used f32 and not f64 only because it is what textplots uses.
    use textplots::*;
    Chart::new(100, 50, interval.start, interval.end)
        .lineplot(&Shape::Lines(&points))
        .display();
}
