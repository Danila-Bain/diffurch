use diffurch::{Locator, Solver};
use nalgebra::{Vector2, vector};

fn main() {
    let mut points = vec![];
    let interval = 0.5..10.5;
    let solution = |t: f64| {
        vector![
            (t - t.floor()) * (t - t.ceil()) * ((t * 0.5).fract() - 0.5).signum(),
            ((t * 0.5).fract() - 0.5).signum() * (t - t.ceil() + t - t.floor()),
        ]
    };

    type Loc = Locator<f64, Vector2<f64>>;
    Solver::new::<f64, Vector2<f64>>()
        .initial([0.25, 0.])
        .equation(|s| vector![s.p.y, -2. * s.p_prev.x.signum()])
        .interval(interval.clone())
        .stepsize(0.09)
        .on(Loc::zero(|s| s.p.x), |_| {})
        .on_step(|s| {
            points.push((s.t, s.p.x));
            dbg!(s.t, (s.p - solution(s.t)).abs());
        })
        .run();

    use plotters::prelude::*;
    let root = BitMapBackend::new("sign-relay-2.png", (600, 200)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .margin(15)
        .set_left_and_bottom_label_area_size(20)
        .build_cartesian_2d(interval, -0.27..0.27)
        .unwrap();
    chart.configure_mesh().draw().unwrap();
    chart
        .draw_series(LineSeries::new(points, BLACK.stroke_width(1)))
        .unwrap();
    root.present().unwrap();
}
