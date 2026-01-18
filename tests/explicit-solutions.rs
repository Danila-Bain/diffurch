#![feature(trait_alias)]
// #![feature(split_array)]

// const ROOT: &str = env!("CARGO_MANIFEST_DIR");
// const MODULE: &str = module_path!();
// use diffurch::{
//     StateFn, StateRef,
//     state::{EvalMutStateFn, EvalStateFn},
//     *,
// };
// use std::f64::consts::PI;

// type Array<const N: usize> = [f64; N];
//
// type DefaultSolver<const N: usize, Equation: EvalMutStateFn<N, f64, [f64; N]>> =
//     Solver<N, 7, 21, f64, Equation, (), std::ops::Range<f64>>;

// trait Equation<const N: usize> = EvalStateFn<N, f64, [f64; N]>;
// trait Solution<const N: usize> = Fn(f64) -> [f64; N];
// type DefaultSolver<Eq, const N: usize> = Solver<f64, N, 7, 21, Eq, (), std::ops::Range<f64>>;
// macro_rules! solver_and_solution {
//     ($N:expr) => { (DefaultSolver<impl Equation<$N>, $N>, impl Fn(f64) -> [f64; $N]) }
// }
//
// macro_rules! unroll {
//     (for $key:pat in [$($val:expr),* $(,)?] $block:block) => {
//         $(
//             let $key = $val;
//             $block
//         )*
//     }
// }
//
// fn ode1_t_pow_m2() -> solver_and_solution! {1} {
//     let eq = StateFn::<1, f64, _, _, _>::new(|&StateRef{ t, .. }| [-2. * t.powi(-3)]);
//     let interval = 1. ..100.;
//     let solution = |t: f64| [t.powi(-2)];
//     let solver = Solver::new().equation(eq).interval(interval);
//     (solver, solution)
// }

// fn ode1_t_pow_5() -> solver_and_solution! {1} {
//     let interval = 0. ..100.;
//     let solution = |t: f64| [t.powi(5)];
//     let solver = Solver::new()
//         .equation(diffurch::StateFn::new(
//             |&diffurch::StateRef::<f64, 1> { t, .. }| [5. * t.powi(4)],
//         ))
//         .interval(interval);
//     (solver, solution)
// }
//
// fn ode1_t_cos() -> solver_and_solution! {1} {
//     let eq = state_fn!(|t| [f64::cos(t)]);
//     let interval = 0. ..100.;
//     let solution = |t: f64| [t.sin()];
//     let solver = Solver::new().equation(eq).interval(interval);
//     (solver, solution)
// }
//
// fn ode1_lin_1() -> solver_and_solution! {1} {
//     let eq = state_fn!(|x: &[x]| [x]);
//     let interval = 0. ..20.;
//     let solution = |t: f64| [t.exp()];
//     let solver = Solver::new().equation(eq).interval(interval);
//     (solver, solution)
// }
//
// fn ode1_lin_m1() -> solver_and_solution! {1} {
//     let eq = state_fn!(|x: [x]| [-x]);
//     let interval = 0. ..20.;
//     let solution = |t: f64| [(-1. * t).exp()];
//
//     let solver = Solver::new().equation(eq).interval(interval);
//
//     (solver, solution)
// }
//
// fn ode1_lin_m10() -> solver_and_solution! {1} {
//     let eq = state_fn!(|x: [x]| [-10. * x]);
//     let interval = 0. ..20.;
//     let solution = |t: f64| [(-10. * t).exp()];
//     let solver = Solver::new().equation(eq).interval(interval);
//     (solver, solution)
// }
//
// fn ode1_lin_m100() -> solver_and_solution! {1} {
//     let eq = state_fn!(|x: [x]| [-100. * x]);
//     let interval = 0. ..20.;
//     let solution = |t: f64| [(-100. * t).exp()];
//     let solver = Solver::new().equation(eq).interval(interval);
//     (solver, solution)
// }
//
// fn ode1_lin_cos() -> solver_and_solution! {1} {
//     let eq = state_fn!(|t, x: [x]| [f64::cos(t) * x]);
//     let interval = 0. ..20.;
//     let solution = |t: f64| [(t.sin()).exp()];
//
//     let solver = Solver::new().equation(eq).interval(interval);
//
//     (solver, solution)
// }
//
// fn ode1_lin_10cos() -> solver_and_solution! {1} {
//     let eq = state_fn!(|t, x: [x]| [10. * f64::cos(t) * x]);
//     let interval = 0. ..20.;
//     let solution = |t: f64| [(10. * t.sin()).exp()];
//
//     let solver = Solver::new().equation(eq).interval(interval);
//
//     (solver, solution)
// }
//
// fn ode1_lin_2t() -> solver_and_solution! {1} {
//     let eq = state_fn!(|t, x: [x]| [2. * t * x]);
//     let interval = 0. ..20.;
//     let solution = |t: f64| [(t * t).exp()];
//
//     let solver = Solver::new().equation(eq).interval(interval);
//
//     (solver, solution)
// }
//
// fn ode2_lin_i() -> solver_and_solution! {2} {
//     let eq = state_fn!(|x: &[x, y]| [0.-y, x]);
//     let interval = 0. ..20.;
//     let solution = |t: f64| [t.cos(), t.sin()];
//
//     let solver = Solver::new().equation(eq).interval(interval);
//
//     (solver, solution)
// }
// //
// fn ode2_lin_i_log() -> solver_and_solution! {2} {
//     let eq = state_fn!(|x: &[x, y]| [0. - x * f64::ln(y), y * f64::ln(x)]);
//     let interval = 0. ..20.;
//     let solution = |t: f64| [t.cos().exp(), t.sin().exp()];
//
//     let solver = Solver::new().equation(eq).interval(interval);
//
//     (solver, solution)
// }
//
// fn ode2_lin_i_cos() -> solver_and_solution!{2} {
//     let eq = state_fn!(|t, x: [x, y]| {
//         let k = 1. + t.cos();
//         [-k * y, k * x]
//     });
//     let interval = 0. ..20.;
//     let solution = |t: f64| {
//         let t = t + t.sin();
//         [t.cos(), t.sin()]
//     };
//
//     let solver = Solver::new().equation(eq).interval(interval);
//
//     (solver, solution)
// }
//
// fn ode2_center_r_m1() -> solver_and_solution!{2} {
//     let eq = state_fn!(|x: [x, y]| {
//         let r = f64::hypot(x, y);
//         [-y / r, x / r]
//     });
//     let interval = 0. ..20.;
//     let solution = |t: f64| [t.cos(), t.sin()];
//
//     let solver = Solver::new().equation(eq).interval(interval);
//
//     (solver, solution)
// }
//
// fn ode2_center_r_2() -> solver_and_solution!{2} {
//     let eq = state_fn!(|x: &[x, y]| {
//         let [x, y]: [f64; 2] = [x, y];
//         let r2 = x.powi(2) + y.powi(2);
//         [-r2 * y, r2 * x]
//     });
//     let interval = 0. ..20.;
//     let solution = |t: f64| [t.cos(), t.sin()];
//     let solver = Solver::new().equation(eq).interval(interval);
//     (solver, solution)
// }
//
// fn ode2_stable_cycle() -> solver_and_solution!{2} {
//     let eq = state_fn!(|x: [x, y]| {
//         let a = 1. - (x * x + y * y);
//         [a * x - 30. * y, 30. * x + a * y]
//     });
//     let interval = 0. ..20.;
//     let solution = |t: f64| {
//         let r = (1. + 99. * (-2. * t).exp()).sqrt();
//         [(30. * t).cos() / r, (30. * t).sin() / r]
//     };
//
//     let solver = Solver::new().equation(eq).interval(interval);
//
//     (solver, solution)
// }
// fn ode3_tori() -> solver_and_solution!{3} {
//     let eq = state_fn!(|x: &[x, y, z]| {
//         let r = f64::hypot(x, y);
//         [
//             4. * PI * x * z / r - y,
//             x + 4. * PI * y * z / r,
//             4. * PI * (1. - r),
//         ]
//     });
//     let interval = 0. ..20.;
//     let solution = |t: f64| {
//         let r = 1. + (4. * PI * t).sin() / 2.;
//         [r * t.cos(), r * t.sin(), (4. * PI * t).cos() / 2.]
//     };
//
//     let solver = Solver::new().equation(eq).interval(interval);
//
//     (solver, solution)
// }
//
// fn ode1_relay_fract_t_no_detection() -> solver_and_solution!{1} {
//     let eq = state_fn!(|t, x: [x]| [-x + t.fract()]);
//     let interval = 0. ..20.;
//     let solution = |t: f64| [t.fract() - 1. + (1. - t.fract()).exp() / (f64::consts::E - 1.)];
//
//     let solver = Solver::new().equation(eq).interval(interval);
//
//     (solver, solution)
// }

// fn ode1_relay_fract_t_periodic_location() -> solver_and_solution!{1} {
//     let eq = state_fn!(|t, x: [x]| [-x + t.fract()]);
//     let interval = 0. ..20.;
//     let solution = |t: f64| [t.fract() - 1. + (1. - t.fract()).exp() / (f64::consts::E - 1.)];
//
//     let solver = Solver::new()
//         .equation(eq)
//         .initial(ic)
//         .interval(interval)
//         .on(
//             Periodic {
//                 period: 1.,
//                 offset: 0.,
//             },
//             event!(), // event!(|t| println!("Periodic at t = {t:.5}")),
//         );
//
//     (solver, solution)
// }
//
// fn ode1_relay_fract_t() -> solver_and_solution!{1} {
//     let eq = state_fn!(|t, x: [x], [xx]| [-x + t - xx.t_prev().floor()]);
//     let interval = 0. ..20.;
//     let solution = |t: f64| [t.fract() - 1. + (1. - t.fract()).exp() / (f64::consts::E - 1.)];
//
//     let solver = Solver::new()
//         .equation(eq)
//         .initial(ic)
//         .interval(interval)
//         .on(
//             Periodic {
//                 period: 1.,
//                 offset: 0.,
//             },
//             event_mut!(), // event!(|t| println!("Periodic at t = {t:.5}")),
//         );
//
//     (solver, solution)
// }
//
// fn ode1_relay_fract_t_stateful() -> solver_and_solution!{1} {
//     let n_ = Rc::new(Cell::new(0.));
//     let n = n_.clone();
//     let eq = state_fn!(move |t, x: [x]| [-x + t - n.get()]);
//     let interval = 0. ..20.;
//     let solution = |t: f64| [t.fract() - 1. + (1. - t.fract()).exp() / (f64::consts::E - 1.)];
//
//     let n = n_.clone();
//     let solver = Solver::new()
//         .equation(eq)
//         .initial(ic)
//         .interval(interval)
//         .on(
//             Periodic {
//                 period: 1.,
//                 offset: 0.,
//             },
//             event!(move || n.set(n.get() + 1.)), // event!(|t| println!("Periodic at t = {t:.5}")),
//         );
//
//     (solver, solution)
// }
//
// fn ode2_relay_msign_no_detection() -> solver_and_solution!{2} {
//     let eq = state_fn!(|x: [x, dx]| [dx, -2. * x.signum()]);
//     let interval = 0.5..20.5;
//     let solution = |t: f64| {
//         let t = t;
//         [
//             (-1i32).pow(t.floor() as u32) as f64 * (t - t.floor()) * (t.ceil() - t),
//             (-1i32).pow(t.floor() as u32) as f64 * (1. + 2. * t.floor() - 2. * t),
//         ]
//     };
//
//     let solver = Solver::new().equation(eq).initial(ic).interval(interval);
//
//     (solver, solution)
// }
//
// fn ode2_relay_msign_no_conservative_switching() -> solver_and_solution!{2} {
//     let eq = state_fn!(|[x, dx]| [dx, -2. * x.signum()]);
//     let interval = 0.5..20.5;
//     let solution = |t: f64| {
//         let t = t;
//         [
//             (-1i32).pow(t.floor() as u32) as f64 * (t - t.floor()) * (t.ceil() - t),
//             (-1i32).pow(t.floor() as u32) as f64 * (1. + 2. * t.floor() - 2. * t),
//         ]
//     };
//
//     let solver = Solver::new()
//         .equation(eq)
//         .initial(ic)
//         .interval(interval)
//         .on(loc_sign!(|[x, _]| x), event!());
//
//     (solver, solution)
// }
//
// fn ode2_relay_msign() -> solver_and_solution!{2} {
//     let eq = state_fn!(|_, [_, dx], [x, _]| [dx, -2. * x.prev().signum()]);
//     let interval = 0.5..20.5;
//     let solution = |t: f64| {
//         let t = t;
//         [
//             (-1i32).pow(t.floor() as u32) as f64 * (t - t.floor()) * (t.ceil() - t),
//             (-1i32).pow(t.floor() as u32) as f64 * (1. + 2. * t.floor() - 2. * t),
//         ]
//     };
//
//     let solver = Solver::new()
//         .equation(eq)
//         .initial(ic)
//         .interval(interval)
//         .on(loc_sign!(|[x, _]| x), event!());
//
//     (solver, solution)
// }
//
// fn dde1_lin_i() -> solver_and_solution!{1} {
//     let eq = state_fn!(|t, [x], [x_]| [x / 1f64.tan() - x_(t - 1.) / 1f64.sin()]);
//     let interval = 0. ..20.;
//     let solution = ic.clone();
//
//     let solver = Solver::new().equation(eq).initial(ic).interval(interval);
//
//     (solver, solution)
// }
//
// fn dde1_lin_jagged_exp_no_propagation() -> (
//     Solver<
//         'static,
//         1,
//         7,
//         DDEStateFnMut<
//             1,
//             impl FnMut(f64, [f64; 1], [&dyn StateCoordFnTrait; 1]) -> [f64; 1],
//             [f64; 1],
//         >,
//         impl Fn(f64) -> [f64; 1],
//         std::ops::Range<f64>,
//     >,
//     impl Fn(f64) -> [f64; 1],
// ) {
//     let eq = state_fn!(|t, [_], [x]| [x(t - 1.)]);
//     let ic = |t: f64| {
//         [(0..=(t.floor() as i32))
//             .map(|k| (t - k as f64).powi(k) / (k as f64 + 1.).gamma())
//             .sum()]
//     };
//     let interval = 1. ..20.;
//     let solution = ic.clone();
//
//     let solver = Solver::new().equation(eq).initial(ic).interval(interval);
//
//     (solver, solution)
// }
//
// fn dde1_lin_jagged_exp() -> (
//     Solver<
//         'static,
//         1,
//         7,
//         DDEStateFnMut<
//             1,
//             impl FnMut(f64, [f64; 1], [&dyn StateCoordFnTrait; 1]) -> [f64; 1],
//             [f64; 1],
//         >,
//         [f64; 1],
//         std::ops::Range<f64>,
//         Nil,
//         Nil,
//         Nil,
//         hlist2::Cons<
//             Loc<Propagator<1, impl StateFnMut<1, Output = f64>>, Propagation, location::Lerp>,
//             Nil,
//         >,
//     >,
//     impl Fn(f64) -> [f64; 1],
// ) {
//     let eq = state_fn!(|t, [_], [x]| [x(t - 1.)]);
//     let interval = 1. ..20.;
//     let solution = |t: f64| {
//         [(0..=(t.floor() as i32))
//             .map(|k| (t - k as f64).powi(k) / (k as f64 + 1.).gamma())
//             .sum()]
//     };
//
//     let solver = Solver::new()
//         .equation(eq)
//         .initial(ic)
//         .interval(interval)
//         .delay(1.)
//         .initial_disco([(1., 1)]);
//
//     (solver, solution)
// }
//
// fn dde2_lin_jagged_cos_no_propagation() -> (
//     Solver<
//         'static,
//         2,
//         7,
//         DDEStateFnMut<
//             2,
//             impl FnMut(f64, [f64; 2], [&dyn StateCoordFnTrait; 2]) -> [f64; 2],
//             [f64; 2],
//         >,
//         [f64; 2],
//         std::ops::Range<f64>,
//     >,
//     impl Fn(f64) -> [f64; 2],
// ) {
//     let eq = state_fn!(|t, [_, dx], [x, _]| [dx, -x(t - 1.)]);
//     let interval = 1. ..20.;
//     let solution = |t: f64| {
//         [
//             (0..=(t.floor() as i32))
//                 .map(|k| (-1.).powi(k) * (t - k as f64).powi(2 * k) / ((2 * k) as f64 + 1.).gamma())
//                 .sum(),
//             (1..=(t.floor() as i32))
//                 .map(|k| {
//                     (-1.).powi(k) * (t - k as f64).powi(2 * k - 1)
//                         / ((2 * k - 1) as f64 + 1.).gamma()
//                 })
//                 .sum(),
//         ]
//     };
//
//     let solver = Solver::new().equation(eq).initial(ic).interval(interval);
//
//     (solver, solution)
// }
//
// fn dde2_lin_jagged_cos() -> (
//     Solver<
//         'static,
//         2,
//         7,
//         DDEStateFnMut<
//             2,
//             impl FnMut(f64, [f64; 2], [&dyn StateCoordFnTrait; 2]) -> [f64; 2],
//             [f64; 2],
//         >,
//         [f64; 2],
//         std::ops::Range<f64>,
//         Nil,
//         Nil,
//         Nil,
//         hlist2::Cons<
//             Loc<Propagator<2, impl StateFnMut<2, Output = f64>>, Propagation, location::Lerp>,
//             Nil,
//         >,
//     >,
//     impl Fn(f64) -> [f64; 2],
// ) {
//     let eq = state_fn!(|t, [_, dx], [x, _]| [dx, -x(t - 1.)]);
//     let interval = 1. ..20.;
//     let solution = |t: f64| {
//         [
//             (0..=(t.floor() as i32))
//                 .map(|k| (-1.).powi(k) * (t - k as f64).powi(2 * k) / ((2 * k) as f64 + 1.).gamma())
//                 .sum(),
//             (1..=(t.floor() as i32))
//                 .map(|k| {
//                     (-1.).powi(k) * (t - k as f64).powi(2 * k - 1)
//                         / ((2 * k - 1) as f64 + 1.).gamma()
//                 })
//                 .sum(),
//         ]
//     };
//
//     let solver = Solver::new()
//         .equation(eq)
//         .initial(ic)
//         .interval(interval)
//         .delay(1.)
//         .initial_disco([(1., 1)]);
//
//     (solver, solution)
// }
//
// fn ndde1_lin_i_u() -> (
//     Solver<
//         'static,
//         1,
//         7,
//         DDEStateFnMut<
//             1,
//             impl FnMut(f64, [f64; 1], [&dyn StateCoordFnTrait; 1]) -> [f64; 1],
//             [f64; 1],
//         >,
//         (impl Fn(f64) -> [f64; 1], impl Fn(f64) -> [f64; 1]),
//         std::ops::Range<f64>,
//     >,
//     impl Fn(f64) -> [f64; 1],
// ) {
//     let eq = state_fn!(|t, [x], [x_]| [-x * 1f64.tan() + x_.d(t - 1.) / 1f64.cos()]);
//     let ic = (|t: f64| [t.sin()], |t: f64| [t.cos()]);
//     let interval = 0. ..10.;
//     let solution = ic.0.clone();
//     let solver = Solver::new().equation(eq).initial(ic).interval(interval);
//
//     (solver, solution)
// }
//
// fn ndde1_lin_i_s() -> (
//     Solver<
//         'static,
//         1,
//         7,
//         DDEStateFnMut<
//             1,
//             impl FnMut(f64, [f64; 1], [&dyn StateCoordFnTrait; 1]) -> [f64; 1],
//             [f64; 1],
//         >,
//         (impl Fn(f64) -> [f64; 1], impl Fn(f64) -> [f64; 1]),
//         std::ops::Range<f64>,
//     >,
//     impl Fn(f64) -> [f64; 1],
// ) {
//     let eq = state_fn!(|t, [_], [x_]| [-x_(t - 1.) * 1f64.sin() + x_.d(t - 1.) * 1f64.cos()]);
//     let ic = (|t: f64| [t.sin()], |t: f64| [t.cos()]);
//     let interval = 0. ..20.;
//     let solution = ic.0.clone();
//     let solver = Solver::new().equation(eq).initial(ic).interval(interval);
//
//     (solver, solution)
// }
//
// fn ndde1_lin_copy_sin() -> (
//     Solver<
//         'static,
//         1,
//         7,
//         DDEStateFnMut<
//             1,
//             impl FnMut(f64, [f64; 1], [&dyn StateCoordFnTrait; 1]) -> [f64; 1],
//             [f64; 1],
//         >,
//         (impl Fn(f64) -> [f64; 1], impl Fn(f64) -> [f64; 1]),
//         std::ops::Range<f64>,
//     >,
//     impl Fn(f64) -> [f64; 1],
// ) {
//     let eq = state_fn!(|t, [_], [x_]| [x_.d(t - 2. * PI)]);
//     let ic = (|t: f64| [t.sin()], |t: f64| [t.cos()]);
//     let interval = 0. ..20.;
//     let solution = ic.0.clone();
//     let solver = Solver::new().equation(eq).initial(ic).interval(interval);
//
//     (solver, solution)
// }
// fn ndde1_lin_copy_triangle_no_propagation() -> (
//     Solver<
//         'static,
//         1,
//         7,
//         DDEStateFnMut<
//             1,
//             impl FnMut(f64, [f64; 1], [&dyn StateCoordFnTrait; 1]) -> [f64; 1],
//             [f64; 1],
//         >,
//         (impl Fn(f64) -> [f64; 1], impl Fn(f64) -> [f64; 1]),
//         std::ops::Range<f64>,
//     >,
//     impl Fn(f64) -> [f64; 1],
// ) {
//     let eq = state_fn!(|t, [_], [x_]| [x_.d(t - 2. * PI)]);
//     let ic = (|t: f64| [t.sin().asin()], |t: f64| [t.cos().signum()]);
//     let interval = 0. ..20.;
//     let solution = ic.0.clone();
//     let solver = Solver::new().equation(eq).initial(ic).interval(interval);
//
//     (solver, solution)
// }
//
// fn ndde1_lin_copy_triangle() -> (
//     Solver<
//         'static,
//         1,
//         7,
//         DDEStateFnMut<
//             1,
//             impl FnMut(f64, [f64; 1], [&dyn StateCoordFnTrait; 1]) -> [f64; 1],
//             [f64; 1],
//         >,
//         (impl Fn(f64) -> [f64; 1], impl Fn(f64) -> [f64; 1]),
//         std::ops::Range<f64>,
//         Nil,
//         Nil,
//         Nil,
//         hlist2::Cons<
//             Loc<Propagator<1, impl StateFnMut<1, Output = f64>>, Propagation, location::Lerp>,
//             Nil,
//         >,
//     >,
//     impl Fn(f64) -> [f64; 1],
// ) {
//     let eq = state_fn!(|t, [_], [x_]| [x_.d(t - 2. * PI)]);
//     let ic = (|t: f64| [t.sin().asin()], |t: f64| [t.cos().signum()]);
//     let interval = 0. ..20.;
//     let solution = ic.0.clone();
//     let solver = Solver::new()
//         .equation(eq)
//         .initial(ic)
//         .interval(interval)
//         .neutral_delay(2. * PI)
//         .initial_disco([(-3. * PI / 2., 1), (-PI / 2., 1), (0., 1)]);
//
//     (solver, solution)
// }
//
// #[test]
// fn test1() {
//     unroll! {
//         for rk in [
//             RK::euler(),
//             RK::midpoint(),
//             RK::heun2(),
//             RK::ralston2(),
//             RK::kutta3(),
//             RK::ralston3(),
//             RK::wray3(),
//             RK::ssp3(),
//             RK::rk4(),
//             RK::rk43(),
//             RK::rktp64(),
//         ] {
//             unroll! {
//                 for (solver, solution) in [
//                     ode1_t_pow_m2()
//                 ] {
//                     let ic = solution(solver.interval.start);
//                     solver.rk(rk).stepsize(0.1).initial(ic).run();
//                 }
//             }
//         }
//     }
// }
