#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use diffurch::*;

fn main() {
    let k = 0.5;
    let sol = |t: f64| (t * k).sin();

    let mut points1 = vec![];
    let mut points2 = vec![];

    Solver::new()
        .equation(state_fn!(|[x, dx]| [dx, -k * k * x]))
        .initial(|t: f64| [(t * k).sin(), k * (t * k).cos()])
        .interval(0. .. 50.)
        .rk(&rk::RK98)
        .stepsize(1.)
        .on_step(event!(|t, [x, _dx]| points1.push((t as f32, x as f32))).subdivide(10))
        .on_step(event!(|t, [x, _dx]| points2.push((t, x).into())).subdivide(10))
        .on_step(event!(|t, [x, dx]| dbg!(t, x, dx, (x - sol(t)).abs())))
        .run();

    use textplots::*;
    Chart::new(160, 80, 0., 50.)
        .lineplot(&Shape::Lines(&points1))
        .display();

    let mut plot = pgfplots::axis::plot::Plot2D::new();
    plot.coordinates = points2;
    pgfplots::Picture::from(plot)
        .show_pdf(pgfplots::Engine::PdfLatex)
        .unwrap();
}
