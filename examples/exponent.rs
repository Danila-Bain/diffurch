// use std::time::Duration;
//
// use diffurch::*;
//
fn main() {
    //     let k = f64::ln(2.);
    //     let eq = Equation::ode(move |[x]| [k * x]).with_delay(100.);
    //     let ic = |t: f64| [(k * t).exp()];
    //
    //     Solver::new()
    //         .rk(&rk::RK98)
    //         .stepsize(0.09)
    //         // .on_step(Event::ode2(|t, [x]| (t, x, (x - ic(t)[0]) / x)).to_std())
    //         .on_step(
    //             Event::dde(|t, [x], [x_]| (t, x, (x - ic(t)[0]) / x, (x_(t - 1.) - ic(t - 1.)[0]) / x))
    //                 .to_std(),
    //         )
    //         .run(eq, ic, 0. ..5.);
    //
    //     std::thread::sleep(Duration::from_millis(100));
}
