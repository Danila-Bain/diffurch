// fn ode2_relay_msign_naive() -> (
//     Equation<2, ODEStateFnMut<2, impl FnMut([f64; 2]) -> [f64; 2], [f64; 2]>>,
//     [f64; 2],
//     std::ops::Range<f64>,
//     impl Fn(f64) -> [f64; 2],
// ) {
//
//     (eq, ic, interval, solution)
// }
//
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use diffurch::*;

fn main() {
    let mut points1 = vec![];
    let mut points2 = vec![];

    let eq = equation!(|[x, dx]| [dx, -2. * x.signum()]).disco(Loc::sign(state_fn!(|[x, _]| {println!("try: {x}"); x})));
    let ic = [0.25, 0.];
    let interval = 0.5..20.5;
    let solution = |t: f64| {
        let t = t;
        [
            (t - t.floor()) * (t - t.ceil()) * ((t * 0.5).fract() - 0.5).signum(),
            ((t * 0.5).fract() - 0.5).signum() * (t - t.ceil() + t - t.floor()),
        ]
    };

    Solver::new()
        .rk(&rk::RK98)
        .stepsize(0.05)
        .on_step(event!(|t, [x, _dx]| points1.push((t as f32, x as f32))).subdivide(10))
        .on_step(event!(|t, [x, _dx]| points2.push((t, x).into())).subdivide(10))
        .on_step(event!(|t, [x, dx]| {
            println!("step: {t}");
            dbg!(t, x, dx, (x - solution(t)[0]).abs())
        }))
        .run(eq, ic, interval.clone());

    use textplots::*;
    Chart::new(160, 80, interval.start as f32, interval.end as f32)
        .lineplot(&Shape::Lines(&points1))
        .display();

    let mut plot = pgfplots::axis::plot::Plot2D::new();
    plot.coordinates = points2;
    pgfplots::Picture::from(plot)
        .show_pdf(pgfplots::Engine::PdfLatex)
        .unwrap();
}
