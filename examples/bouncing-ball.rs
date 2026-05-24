use derive_state::State;
use diffurch::*;

fn main() {
    let k = 0.90;
    let g = 9.8;

    let mut points = Vec::new();
    // let mut points_continuous = Vec::new();

    #[derive(State)]
    struct State {
        x: f64,
        dx: f64,
    }

    let interval = 0. ..8.58;

    type Loc = Locator<f64, State>;

    Solver::new::<f64, State>()
        .initial(State { x: 1., dx: -0.01 })
        .equation(|s| State { x: s.p.dx, dx: -g })
        .interval(interval.clone())
        .stepsize(0.005)
        .on_step(|s| points.push((s.t, s.p.x)))
        .on(Loc::zero(|s| s.p.dx), |_| {})
        .on_mut(Loc::below_zero(|s| s.p.x), |s| {
            s.p.x = 0.;
            s.p.dx = k * s.p.dx.abs();
            dbg!(*s.t);
        })
        .run();

    use plotters::prelude::*;
    let root = SVGBackend::new("bouncing-ball.svg", (1200, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(interval, 0. ..1.)
        .unwrap();
    chart.configure_mesh().draw().unwrap();
    chart
        .draw_series(LineSeries::new(points, BLACK.stroke_width(1)))
        .unwrap();
    root.present().unwrap();
}
