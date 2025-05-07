use core::f64::consts::PI;

use diffurch::{Equation, Event, Solver, rk, util::with_derivative::Differentiable};

const A: f64 = -1.;
const T: f64 = 1.;

fn solution(epsilon: f64, alpha: f64, beta: f64) -> (f64, f64, f64) {
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

    let mut ret = (0., 0., 0.);
    //
    //

    let stepsize = 1./(beta * 10.).round();
    // for now, delay should be an integer mutiple of a stepsize for NDDEs, due to the initial
    // discontinuities

    println!("{}", stepsize);

    Solver::new()
        .stepsize(stepsize)
        .rk(&rk::RK98)
        // .on_step(Event::dde(|t, [x, x_int, dx_int], [_, x_int_, dx_int_]| (t, x, x_int - x_int_(t - 1.), dx_int - dx_int_(t - 1.))).to_std().separated_by(5.1))
        .on_stop(
            Event::dde(|t, [_, x_int, dx_int], [x_, x_int_, dx_int_]| {
                (
                    x_int - x_int_(t - 1.),
                    dx_int - dx_int_(t - 1.),
                    x_(t - 1.).hypot(x_.d(t - 1.) / beta),
                )
            })
            .to_var(&mut ret),
        )
        .on_step(
            Event::ode2(|t, [x, _, _]| [t, x])
                .to_vecs([&mut t, &mut x])
                // .subdivide(5)
                .in_range((range.end - 4.*T * (2.*PI / beta))..range.end),
        )
        .run(equation, ic, range);

    let mut plot = pgfplots::axis::plot::Plot2D::new();
    plot.coordinates = (0..t.len()).map(|i| (t[i], x[i]).into()).collect();
    pgfplots::Picture::from(plot)
        .show_pdf(pgfplots::Engine::PdfLatex)
        .unwrap();
    //
    // std::thread::sleep(std::time::Duration::from_secs(1))
    ret
}

fn main() {
    let mut alpha = 0.001;
    for i in (4..=100).step_by(2) {
        let (x_int, dx_int, alpha_end) = solution(0.02, alpha, i as f64 * PI);
        alpha = alpha_end;
        println!("{i} pi:    alpha = {alpha}, x_int = {x_int}, dx_int = {dx_int}");
    }
}
