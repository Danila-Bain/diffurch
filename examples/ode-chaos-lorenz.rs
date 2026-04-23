#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

fn main() {
    // system parameters
    let sigma = 10.;
    let rho = 28.;
    let beta = 8. / 3.;

    // output variables
    let mut t = Vec::new();
    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut z = Vec::new();

    // run solver
    diffurch::Solver::new::<f64, nalgebra::Vector3<f64>>()
        .initial([1., 2., 20.])
        .equation(|s| {
            let [x, y, z] = s.p.as_slice() else { panic!() };
            nalgebra::vector![sigma * (y - x), x * (rho - z) - y, x * y - beta * z]
        })
        .interval(0. ..2000.)
        .stepsize(1. / 250.)
        .on_step(|s| {
            if s.t >= 50. {
                t.push(s.t);
                x.push(s.p.x);
                y.push(s.p.y);
                z.push(s.p.z);
            }
        })
        .run();

    let points = x.iter().copied().zip(z.iter().copied()).collect::<Vec<_>>();

    use plotters::prelude::*;
    let root = BitMapBackend::new("lorenz.png", (1500, 1200)).into_drawing_area();
    // let root = SVGBackend::new("lorenz.svg", (500, 400)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(30)
        .build_cartesian_2d(lims(x.iter().copied()), lims(z.iter().copied()))
        .unwrap();
    chart
        .draw_series(LineSeries::new(points, BLACK.mix(0.1).stroke_width(1)))
        .unwrap();

    root.present().unwrap();
}

fn lims(iter: impl Iterator<Item = f64>) -> std::ops::Range<f64> {
    let (min, max) = iter.fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), el| {
        (min.min(el), max.max(el))
    });

    min..max
}
