#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use diffurch::*;

fn main() {
    let mut points1 = vec![];
    let mut points2 = vec![];

    let eq =
        equation!(|_, [_, dx], [x, _]| [dx, -2. * x.prev().signum()]).loc(Loc::new(state_fn!(|t, [x, _]| {
            println!("try: {x} at {t}");
            x
        })).sign().bisection());
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
        .stepsize(0.3)
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
