#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![allow(dead_code)]

use std::{f64::consts::PI, fs::File, io::BufReader};

use diffurch::rk::*;
use diffurch::*;
use ndarray::*;
use ndarray_linalg::*;

use serde_json::{Value, json, to_string_pretty};

fn ode1_t_pow_m2() -> (
    Equation<1, impl StateFnMut<1, [f64; 1]>>,
    [f64; 1],
    std::ops::Range<f64>,
    impl Fn(f64) -> [f64; 1],
) {
    let eq = equation!(|t| [-2. * t.powi(-3)]);
    let ic = [1.];
    let interval = 1. ..100.;
    let solution = |t: f64| [t.powi(-2)];

    (eq, ic, interval, solution)
}

fn ode1_t_pow_5() -> (
    Equation<1, impl StateFnMut<1, [f64; 1]>>,
    [f64; 1],
    std::ops::Range<f64>,
    impl Fn(f64) -> [f64; 1],
) {
    let eq = equation!(|t| [5. * t.powi(4)]);
    let ic = [0.];
    let interval = 0. ..100.;
    let solution = |t: f64| [t.powi(5)];

    (eq, ic, interval, solution)
}

fn ode1_t_cos() -> (
    Equation<1, impl StateFnMut<1, [f64; 1]>>,
    [f64; 1],
    std::ops::Range<f64>,
    impl Fn(f64) -> [f64; 1],
) {
    let eq = equation!(|t| [t.cos()]);
    let ic = [0.];
    let interval = 0. ..100.;
    let solution = |t: f64| [t.sin()];

    (eq, ic, interval, solution)
}

fn ode1_lin_1() -> (
    Equation<1, impl StateFnMut<1, [f64; 1]>>,
    [f64; 1],
    std::ops::Range<f64>,
    impl Fn(f64) -> [f64; 1],
) {
    let eq = equation!(|[x]| [x]);
    let ic = [1.];
    let interval = 0. ..20.;
    let solution = |t: f64| [t.exp()];

    (eq, ic, interval, solution)
}

fn ode1_lin_m1() -> (
    Equation<1, impl StateFnMut<1, [f64; 1]>>,
    [f64; 1],
    std::ops::Range<f64>,
    impl Fn(f64) -> [f64; 1],
) {
    let eq = equation!(|[x]| [-x]);
    let ic = [1.];
    let interval = 0. ..20.;
    let solution = |t: f64| [(-1. * t).exp()];

    (eq, ic, interval, solution)
}

fn ode1_lin_m10() -> (
    Equation<1, impl StateFnMut<1, [f64; 1]>>,
    [f64; 1],
    std::ops::Range<f64>,
    impl Fn(f64) -> [f64; 1],
) {
    let eq = equation!(|[x]| [-10. * x]);
    let ic = [1.];
    let interval = 0. ..20.;
    let solution = |t: f64| [(-10. * t).exp()];

    (eq, ic, interval, solution)
}

fn ode1_lin_m100() -> (
    Equation<1, impl StateFnMut<1, [f64; 1]>>,
    [f64; 1],
    std::ops::Range<f64>,
    impl Fn(f64) -> [f64; 1],
) {
    let eq = equation!(|[x]| [-100. * x]);
    let ic = [1.];
    let interval = 0. ..20.;
    let solution = |t: f64| [(-100. * t).exp()];

    (eq, ic, interval, solution)
}

fn ode1_lin_cos() -> (
    Equation<1, impl StateFnMut<1, [f64; 1]>>,
    [f64; 1],
    std::ops::Range<f64>,
    impl Fn(f64) -> [f64; 1],
) {
    let eq = equation!(|t, [x]| [t.cos() * x]);
    let ic = [1.];
    let interval = 0. ..20.;
    let solution = |t: f64| [(t.sin()).exp()];

    (eq, ic, interval, solution)
}

fn ode1_lin_10cos() -> (
    Equation<1, impl StateFnMut<1, [f64; 1]>>,
    [f64; 1],
    std::ops::Range<f64>,
    impl Fn(f64) -> [f64; 1],
) {
    let eq = equation!(|t, [x]| [10. * t.cos() * x]);
    let ic = [1.];
    let interval = 0. ..20.;
    let solution = |t: f64| [(10. * t.sin()).exp()];

    (eq, ic, interval, solution)
}

fn ode1_lin_2t() -> (
    Equation<1, impl StateFnMut<1, [f64; 1]>>,
    [f64; 1],
    std::ops::Range<f64>,
    impl Fn(f64) -> [f64; 1],
) {
    let eq = equation!(|t, [x]| [2. * t * x]);
    let ic = [1.];
    let interval = 0. ..20.;
    let solution = |t: f64| [(t * t).exp()];

    (eq, ic, interval, solution)
}

fn ode2_lin_i() -> (
    Equation<2, impl StateFnMut<2, [f64; 2]>>,
    [f64; 2],
    std::ops::Range<f64>,
    impl Fn(f64) -> [f64; 2],
) {
    let eq = equation!(|[x, y]| [-y, x]);
    let ic = [1., 0.];
    let interval = 0. ..20.;
    let solution = |t: f64| [t.cos(), t.sin()];

    (eq, ic, interval, solution)
}

fn ode2_lin_i_log() -> (
    Equation<2, impl StateFnMut<2, [f64; 2]>>,
    [f64; 2],
    std::ops::Range<f64>,
    impl Fn(f64) -> [f64; 2],
) {
    let eq = equation!(|[x, y]| [-x * y.ln(), y * x.ln()]);
    let ic = [(1.).exp(), 1.];
    let interval = 0. ..20.;
    let solution = |t: f64| [t.cos().exp(), t.sin().exp()];

    (eq, ic, interval, solution)
}

fn ode2_lin_i_cos() -> (
    Equation<2, impl StateFnMut<2, [f64; 2]>>,
    [f64; 2],
    std::ops::Range<f64>,
    impl Fn(f64) -> [f64; 2],
) {
    let eq = equation!(|t, [x, y]| {
        let k = 1. + t.cos();
        [-k * y, k * x]
    });
    let ic = [1., 0.];
    let interval = 0. ..20.;
    let solution = |t: f64| {
        let t = t + t.sin();
        [t.cos(), t.sin()]
    };

    (eq, ic, interval, solution)
}

fn ode2_center_r_m1() -> (
    Equation<2, impl StateFnMut<2, [f64; 2]>>,
    [f64; 2],
    std::ops::Range<f64>,
    impl Fn(f64) -> [f64; 2],
) {
    let eq = equation!(|[x, y]| {
        let r = f64::hypot(x, y);
        [-y / r, x / r]
    });
    let ic = [1., 0.];
    let interval = 0. ..20.;
    let solution = |t: f64| [t.cos(), t.sin()];

    (eq, ic, interval, solution)
}

fn ode2_center_r_2() -> (
    Equation<2, impl StateFnMut<2, [f64; 2]>>,
    [f64; 2],
    std::ops::Range<f64>,
    impl Fn(f64) -> [f64; 2],
) {
    let eq = equation!(|[x, y]| {
        let r2 = x.powi(2) + y.powi(2);
        [-r2 * y, r2 * x]
    });
    let ic = [1., 0.];
    let interval = 0. ..20.;
    let solution = |t: f64| [t.cos(), t.sin()];

    (eq, ic, interval, solution)
}

fn ode2_stable_cycle() -> (
    Equation<2, impl StateFnMut<2, [f64; 2]>>,
    [f64; 2],
    std::ops::Range<f64>,
    impl Fn(f64) -> [f64; 2],
) {
    let eq = equation!(|[x, y]| {
        let a = 1. - (x * x + y * y);
        [a * x - 30. * y, 30. * x + a * y]
    });
    let ic = [0.1, 0.];
    let interval = 0. ..20.;
    let solution = |t: f64| {
        let r = (1. + 99. * (-2. * t).exp()).sqrt();
        [(30. * t).cos() / r, (30. * t).sin() / r]
    };

    (eq, ic, interval, solution)
}
fn ode3_tori() -> (
    Equation<3, impl StateFnMut<3, [f64; 3]>>,
    [f64; 3],
    std::ops::Range<f64>,
    impl Fn(f64) -> [f64; 3],
) {
    let eq = equation!(|[x, y, z]| {
        let r = f64::hypot(x, y);
        [
            4. * PI * x * z / r - y,
            x + 4. * PI * y * z / r,
            4. * PI * (1. - r),
        ]
    });
    let ic = [1., 0., 0.5];
    let interval = 0. ..20.;
    let solution = |t: f64| {
        let r = 1. + (4. * PI * t).sin() / 2.;
        [r * t.cos(), r * t.sin(), (4. * PI * t).cos() / 2.]
    };

    (eq, ic, interval, solution)
}

fn ode2_relay_msign_naive() -> (
    Equation<2, ODEStateFnMut<2, impl FnMut([f64; 2]) -> [f64; 2], [f64; 2]>>,
    [f64; 2],
    std::ops::Range<f64>,
    impl Fn(f64) -> [f64; 2],
) {
    let eq = equation!(|[x, dx]| [dx, -2. * x.signum()]);
    let ic = [0.25, 0.];
    let interval = 0.5 ..20.5;
    let solution = |t: f64| {
        let t = t;
        [
            (t - t.floor()) * (t - t.ceil()) * ((t * 0.5).fract() - 0.5).signum(),
            ((t * 0.5).fract() - 0.5).signum() * (t - t.ceil() + t - t.floor()),
        ]
    };

    (eq, ic, interval, solution)
}

macro_rules! time_error {
    ($equation_name:ident) => {{
        // let (eq, ic, interval, solution) = $equation_name();
        let mut data = vec!();

        data.push(time_error!($equation_name, EULER));

        data.push(time_error!($equation_name, MIDPOINT));
        data.push(time_error!($equation_name, HEUN2));
        data.push(time_error!($equation_name, RALSTON2));

        data.push(time_error!($equation_name, KUTTA3));
        data.push(time_error!($equation_name, HEUN3));
        data.push(time_error!($equation_name, RALSTON3));
        data.push(time_error!($equation_name, WRAY3));
        data.push(time_error!($equation_name, SSP3));

        data.push(time_error!($equation_name, CLASSIC4));
        data.push(time_error!($equation_name, CLASSIC43));

        data.push(time_error!($equation_name, DP544));

        data.push(time_error!($equation_name, RKTP64));

        data.push(time_error!($equation_name, RK98));

        println!("{} done!", stringify!($equation_name));

        // json!({
        //     "equation_name": stringify!($equation_name),
        //     "data": data,
        // })
        json!(data)
    }};


    ($equation_name:ident, $rk:ident) => {{
        let mut time = vec!();
        let mut error = vec!();

        for stepsize in stepsizes() {

            let mut duration = u128::MAX;
            let mut err = 0.;

            for _ in 0..REPEAT {

                let (eq, ic, interval, solution) = $equation_name();
                let start = std::time::Instant::now();

                let mut ts = vec!();
                let mut xs = vec!();

                Solver::new()
                    .rk(&$rk)
                    .stepsize(stepsize)
                    .on_step(
                        Event::new(
                            ODE2StateFnMut(|t, x| {
                                ts.push(t);
                                xs.push(x);
                            }
                            )
                        )
                    )
                    .run(eq, ic, interval);

                duration = duration.min(start.elapsed().as_nanos());

                err =
                    ts.into_iter()
                    .zip(xs.into_iter())
                    .fold(0f64, |acc, (t, x)| {
                        // println!("{:?} ~= {:?}", x, solution(t));
                        let x = arr1(&x);
                        let sol = arr1(&solution(t));
                        let norm = (&x - &sol).norm_max()/(1. + sol.norm_max());
                        // println!("{norm}");
                        // if norm > 0.5 {
                        //     println!("t: {t}, norm: {norm}, sol: {sol}, x: {x}");
                        //     // panic!();
                        // }
                        acc.max(norm)
                    });
            }

            // println!("{err}");
            if err > 1. {
                err = f64::NAN;
            }

            time.push(duration);
            error.push(err);
        }

        println!("\t{} done!", stringify!($rk));

        json!({
            "rk": {
                "name": stringify!($rk),
                "order": $rk.order,
                "order_embedded": $rk.order_embedded,
                "order_interpolant": $rk.order_interpolant,
            },
            "time": time,
            "error": error,
        })
    }};
}

macro_rules! process {
    ($json:ident, $equation_name:ident) => {
        // if $recalculate || !$json.contains_key(stringify!($equation_name)) {
        $json.insert(
            stringify!($equation_name).to_string(),
            time_error!($equation_name),
        );
        // }
    };
}

fn stepsizes() -> ndarray::Logspace<f64> {
    ndarray::logspace(10., -4., 0., 30)
}

const REPEAT: usize = 10;

fn main() {
    let mut my_json = serde_json::Map::new();

    if let Ok(file) = File::open("data_ode.json") {
        let reader = BufReader::new(file);
        let json_value: Value = serde_json::from_reader(reader).unwrap();
        // Try to extract the object as a `Map`
        if let Value::Object(map) = json_value {
            my_json = map;
        } else {
            println!("The JSON root is not an object");
        }
    }

    // process!(my_json, ode1_t_pow_m2);
    // process!(my_json, ode1_t_pow_5);
    // process!(my_json, ode1_t_cos);
    // process!(my_json, ode1_lin_1);
    // process!(my_json, ode1_lin_m1);
    // process!(my_json, ode1_lin_m10);
    // process!(my_json, ode1_lin_m100);
    // process!(my_json, ode1_lin_cos);
    // process!(my_json, ode1_lin_10cos);
    // process!(my_json, ode1_lin_2t);
    // process!(my_json, ode2_lin_i);
    // process!(my_json, ode2_lin_i_log);
    // process!(my_json, ode2_lin_i_cos);
    // process!(my_json, ode2_center_r_m1);
    // process!(my_json, ode2_center_r_2);
    // process!(my_json, ode2_stable_cycle);
    // process!(my_json, ode3_tori);
    // process!(my_json, ode2_relay_msign_naive);

    std::fs::write("data_ode.json", to_string_pretty(&my_json).unwrap()).unwrap();
}
