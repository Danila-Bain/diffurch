use diffurch::{Periodic, Solver, StateRef};
use nalgebra::{Vector3, matrix};

// Solving linear 3-dimensional system, which produces consecutive
// fibonacci numbers at integer values of t.
//
// On the construction of equation, see comments below.

fn main() {
    // magic matrix of the system (see derivation below)
    let a = matrix![
        -0.21520447048200203, 0.43040894096400406, 3.141592653589793;
        0.43040894096400406, 0.21520447048200203, -1.941611038725466;
        -2.2732777998989695, 1.4049629462081452, -0.4812118250596034;
    ];

    Solver::new::<f64, Vector3<f64>>()
        .initial([0., 1., 0.]) // [f_0, f_1, 0.]
        .equation(|&StateRef { p: y, .. }| a * y)
        .interval(0. ..50.)
        .stepsize(0.1)
        .on(Periodic::new(1.), |s| {
            println!("f_{:02} = {:14.2}", s.t, s.p.x)
        })
        .run();
}

// Here we construct a 3-dimensional linear system of the form
//
//      (x', y', z')^T = A (x, y, z)^T,
//
// such that it accepts the solution, such that
//
//      x(n) = nth fibonacci number,
//      y(n) = (n+1)th fibonacci number,
//      z(n) = 0
//
//  for integer nth.
//
//  To do that, we use the closed formula for nth fibonacci number f_n:
//
//      f_n = (phi^n - (-1/phi)^n)/sqrt(5),
//
//  which can be generalized to real argument as
//
//      f(t) = (phi^t - phi^(-t) * cos(pi t) + i phi^(-t) sin(pi t) )/sqrt(5).
//
//  Then, a real linear system that accepts such function in the solution must have
//  eigen values of ln(phi), and ln(1/phi) +- i * pi, hence the dimension 3.
//
//  An example of a real matrix that has such eighen values is
//
//              ln(phi)     0           0
//      A =     0           -ln(phi)    pi
//              0           -pi         -ln(phi).
//
//  From here, I constructed general solution to this equation, and figured a change
//  of variables, that leads to the system that has a desired solution:
//
//      x(t) = Re f(t)   = (phi^t     - phi^(-t)   * cos(pi t))/sqrt(5),
//      y(t) = Re f(t+1) = (phi^(t+1) + phi^(-t-1) * cos(pi t))/sqrt(5),
//      z(t) = Im f(t)   = phi^(-t) * sin(pi t)/sqrt(5).
//

// Code that defines the solution to the constructed equation:
/*
    let sqrt_5 = f64::sqrt(5.);
    let phi = (1. + sqrt_5) / 2.;
    let solution = |t: f64| {
        [
            // t'th Fibonacci number real part
            (phi.powf(t + 0.) - phi.powf(-(t + 0.)) * (PI * t).cos()) / sqrt_5,
            // (t+1)'th Fibonacci number real part
            (phi.powf(t + 1.) + phi.powf(-(t + 1.)) * (PI * t).cos()) / sqrt_5,
            // t'th Fibonacci number imaginary part
            (phi.powf(-t) * (PI * t).sin()) / sqrt_5,
        ]
    };
*/

// Matrix of the equation
/*
    let a = [
        [
            -phi * phi.ln() / (2. + phi),
            2. * phi * phi.ln() / (2. + phi),
            PI,
        ],
        [
            2. * (1. + phi) * phi.ln() / (phi * (2. + phi)),
            phi * phi.ln() / (2. + phi),
            -PI / phi,
        ],
        [
            -(1. + phi) * PI / (2. + phi),
            phi * PI / (2. + phi),
            -phi.ln(),
        ],
    ];
*/
