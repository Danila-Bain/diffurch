#![feature(file_buffered)]
use core::f64::consts::PI;
use std::ops::Range;

use diffurch::{Equation, Event, Solver, rk, util::with_derivative::Differentiable};

const A: f64 = -1.;
const T: f64 = 1.;

fn solution(epsilon: f64, alpha: f64, beta: f64) -> (Range<f64>, Range<f64>, Range<f64>) {
    let equation = Equation::dde(|t, [x, _x_int, _dx_int], [x_, _, _]| {
        let dx = -x + (1. + epsilon) * x_.d(t - T) + A * x_.d(t - T).powi(3);
        [dx, x.powi(2) * beta.powi(2), dx.powi(2)]
    });
    let ic = Differentiable(
        |t: f64| [alpha * (beta * t).sin(), 0., 0.],
        |t: f64| [alpha * beta * (beta * t).cos(), 0., 0.],
    );

    // | 0.25*pow(alpha,2) / beta * (2*beta*T - sin(2*beta*t) + sin(2*beta*(t-T)))
    // | 0.25*pow(alpha,2) * beta * (2*beta*T + sin(2*beta*t) - sin(2*beta*(t-T)));

    let range = 0. ..1000.;
    //
    let mut t = Vec::new();
    let mut x = Vec::new();

    let mut alpha_end = Range::<f64>::default();
    let mut x_int = Range::<f64>::default();
    let mut dx_int = Range::<f64>::default();

    let stepsize = 1. / (beta * 10.).round();
    // for now, delay should be an integer mutiple of a stepsize for NDDEs, due to the initial
    // discontinuities

    println!("{}", stepsize);

    Solver::new()
        .stepsize(stepsize)
        .rk(&rk::RK98)
        // .on_step(Event::dde(|t, [x, x_int, dx_int], [_, x_int_, dx_int_]| (t, x, x_int - x_int_(t - 1.), dx_int - dx_int_(t - 1.))).to_std().separated_by(5.1))
        .on_step(
            Event::dde(|t, [x, x_int, dx_int], [_x_, x_int_, dx_int_]| {
                [x, x_int - x_int_(t - 1.), dx_int - dx_int_(t - 1.)]
            })
            .to_ranges([&mut alpha_end, &mut x_int, &mut dx_int])
            .in_range((range.end - 4. * T)..range.end),
            // .to_var(&mut ret),
        )
        .on_step(
            Event::ode2(|t, [x, _, _]| [t, x])
                .to_vecs([&mut t, &mut x])
                // .subdivide(5)
                .in_range((range.end - 4. * T * (2. * PI / beta))..range.end), // .in_range(range.start .. range.start + 20.*T * (2.*PI / beta))
        )
        .run(equation, ic, range);

    // let mut plot = pgfplots::axis::plot::Plot2D::new();
    // plot.coordinates = (0..t.len()).map(|i| (t[i], x[i]).into()).collect();
    // pgfplots::Picture::from(plot)
    //     .show_pdf(pgfplots::Engine::PdfLatex)
    //     .unwrap();

    // std::thread::sleep(std::time::Duration::from_secs(1))
    (alpha_end, x_int, dx_int)
}

fn main() {
    let epsilon = 0.02;
    let mut alpha = 0.012;
    let mut alphas = Vec::new();
    let mut x_ints = Vec::new();
    let mut dx_ints = Vec::new();

    use std::io::Write;
    let mut file = std::fs::File::create_buffered(format!("integrals,eps={epsilon}.csv")).unwrap();
    writeln!(
        &mut file,
        "beta, alpha, x_int, x_int_error, dx_int, dx_int_error"
    )
    .unwrap();

    for i in (4..=100).step_by(2) {
        let beta = i as f64 * PI;
        let (alpha_end, x_int, dx_int) = solution(epsilon, alpha, beta);
        println!("{i} pi:    alpha = {alpha_end:?}, x_int = {x_int:?}, dx_int = {dx_int:?}");
        alpha = alpha_end.end;
        let (x_int, x_int_error) = (
            (x_int.start + x_int.end) * 0.5,
            (x_int.end - x_int.start) * 0.5,
        );
        let (dx_int, dx_int_error) = (
            (dx_int.start + dx_int.end) * 0.5,
            (dx_int.end - dx_int.start) * 0.5,
        );

        alphas.push((i as f64, alpha).into());
        x_ints.push((i as f64, x_int, None, Some(x_int_error)).into());
        dx_ints.push((i as f64, dx_int, None, Some(dx_int_error)).into());
        // dx_ints.push((i as f64, dx_int).into());
        //
        writeln!(
            &mut file,
            "{i} pi, {alpha}, {x_int}, {x_int_error}, {dx_int}, {dx_int_error}"
        )
        .unwrap();
    }

    let mut axis = pgfplots::axis::Axis::new();

    for (color, coords) in [("red", alphas), ("blue", x_ints), ("green", dx_ints)] {
        let mut plot = pgfplots::axis::plot::Plot2D::new();
        plot.coordinates = coords;
        plot.add_key(pgfplots::axis::plot::PlotKey::Custom(color.to_string()));
        axis.plots.push(plot);
    }

    pgfplots::Picture::from(axis)
        .show_pdf(pgfplots::Engine::PdfLatex)
        .unwrap();
}
