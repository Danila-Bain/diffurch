use diffurch::*;


fn main() {

    // let g = 9.8;
    let g = 2.;

    let eq = Equation::ode(move |[x, dx]| [dx, -g]);
    let ic = |t: f64| [64., 0.];
    let range = 0. .. 8.;

    // Solver::new()
    //     .stepsize(0.1)
    //     .on_step(Event::ode2(|t, [x,dx]| (t, x)).to_std())
    //     .run(eq, ic, range);
}
