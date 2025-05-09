fn main() {
    todo!()
}
// #![feature(file_buffered)]
// use core::f64::consts::PI;
// use std::ops::Range;
//
// use diffurch::{Equation, Event, Solver, rk, util::with_derivative::Differentiable};
//
// const A: f64 = -1.;
// const T: f64 = 1.;
//
// fn main() {
//     let epsilon = 0.002;
//     let alpha = 0.005;
//     let beta = 0.005;
//
//     let equation = Equation::dde(|t, [x], [x_]| {
//         [-x + (1. + epsilon) * x_.d(t - T) + A * x_.d(t - T).powi(3)]
//     });
//     let ic = Differentiable(
//         |t: f64| [alpha * (beta * t).sin()],
//         |t: f64| [alpha * beta * (beta * t).cos()],
//     );
//
//     let range = 0. ..1000.;
//
//     let stepsize = 1. / (beta * 10.).round();
//     // for now, delay should be an integer mutiple of a stepsize for NDDEs, due to the initial
//     // discontinuities
//
//
//     Solver::new()
//         .stepsize(stepsize)
//         .rk(&rk::RK98)
//         // .on_period(100., Event::ode_mut(|t, [x, eps]| {*eps *= 0.9; amplitude}))
//         .run(equation, ic, range);
//
//     // let mut plot = pgfplots::axis::plot::Plot2D::new();
//     // plot.coordinates = (0..t.len()).map(|i| (t[i], x[i]).into()).collect();
//     // pgfplots::Picture::from(plot)
//     //     .show_pdf(pgfplots::Engine::PdfLatex)
//     //     .unwrap();
//
// }
//
//     //
//     // use std::io::Write;
//     // let mut file = std::fs::File::create_buffered(format!("integrals; eps={epsilon}.dat")).unwrap();
//     // writeln!(
//     //     &mut file,
//     //     "i beta alpha x_int x_int_error dx_int dx_int_error"
//     // )
//     // .unwrap();
//     // file.flush().unwrap();
//     //
//     //     writeln!(
//     //         &mut file,
//     //         "{i} {beta} {alpha} {x_int} {x_int_error} {dx_int} {dx_int_error}"
//     //     )
//     //     .unwrap();
//     //     file.flush().unwrap();
//     //
