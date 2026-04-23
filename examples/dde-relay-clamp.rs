#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use diffurch::{loc::detect::Zero, *};

fn main() {
    /* Parameters */
    let alpha = 5.;
    assert!(alpha >= 0.);
    let sigma = 0.4;
    assert!(sigma >= 0.);
    assert!(sigma <= 2.);
    const T: f64 = 1.;
    assert!(T > 0.);
    let v0 = 1.;
    assert!(v0 >= 0.);

    let mut t = Vec::new();
    let mut x = Vec::new();

    let interval = 0. ..100. * T;

    Solver::new::<f64, nalgebra::Vector2<f64>>()
        .initial([1., -alpha * v0])
        .equation(|s| {
            let (x, dx) = (s.p.x, s.p.y);
            nalgebra::vector![dx, -sigma * dx - x - alpha * s.p(s.t - T).x.clamp(-1., 1.)]
        })
        .interval(interval.clone())
        .stepsize(0.012)
        .with_const_delay(T, 2)
        .on_step(|s| {
            t.push(s.t);
            x.push(s.p.x);
        })
        .on::<Zero>(|s| s.p(s.t - T).x.abs() - 1., |_| {})
        .run();

    let points = t.into_iter().zip(x);

    use plotters::prelude::*;
    let root = BitMapBackend::new("dde-relay-clamp.png", (1200, 200)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .margin(15)
        .set_left_and_bottom_label_area_size(20)
        .build_cartesian_2d(interval, -15.27..15.27)
        .unwrap();
    chart.configure_mesh().draw().unwrap();
    chart
        .draw_series(LineSeries::new(points, BLACK.stroke_width(1)))
        .unwrap();
    root.present().unwrap();
}
