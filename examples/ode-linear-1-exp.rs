use diffurch::*;

fn main() {
    let k = f64::ln(2.);
    let eq = equation!(|[x]| [k * x]);
    let ic = [1.];
    let sol = |t: f64| (k * t).exp();

    let range = 0. ..16.;

    let mut points = vec![];

    Solver::rk(&rk::RK98)
        .stepsize(1.)
        .on_step(
            event!(|t, [x]| (t as f32, x as f32))
                .subdivide(10)
                .to_vec(&mut points),
        )
        .on_step(event!(|t, [x]| {
            let err = x - sol(t);
            let rel_err = err / x;
            println!("t={t:<2.0}, x={x:<5.0}, abs error={err:.0e}, rel error={rel_err:.0e}",)
        }))
        .run(eq, ic, range.clone());

    use textplots::*;
    Chart::new(100, 50, range.start as f32, range.end as f32)
        .lineplot(&textplots::Shape::Lines(&points))
        .display();
}
