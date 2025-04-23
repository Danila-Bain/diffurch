// use diffurch::state::*;
// use diffurch::rk_table::*;
// use diffurch::solver::*;
//
// struct HarmonicOscillator {
//     w: f64,
// }
// impl DifferentialEquation for HarmonicOscillator {
//     const N: usize = 2;
//
//     fn f(&self, s: &impl State<{ Self::N }>) -> [f64; Self::N] {
//         let [x, dx] = s.x();
//         [dx, -(self.w).powi(2) * x]
//     }
// }
//
// fn main() {
//     let eq = HarmonicOscillator { w: 1. };
//
//     let range = 0. ..10.;
//     // let range = 0. .. f64::NAN;
//     let initial_condition = |t: f64| [(eq.w * t).sin(), eq.w * (eq.w * t).cos()];
//     let rk = rk98::RK98;
//     let stepsize = 0.05;
//     let res: Vec<[f64; 2]> = eq
//         .solve(range, &initial_condition, rk, stepsize)
//         .into_iter()
//         .collect();
//
//     let [min, max] = res
//         .iter()
//         .map(|p| p[0] * p[0] + p[1] * p[1])
//         .fold([f64::NAN, f64::NAN], |acc, i| {
//             [acc[0].min(i), acc[1].max(i)]
//         });
//
//     assert!(max - min < 1e-14);
//
//     // let mut plot = pgfplots::axis::plot::Plot2D::new();
//     // plot.coordinates = res.into_iter().map(|p| (p[0], p[1]).into()).collect();
//     // pgfplots::Picture::from(plot)
//     //     .show_pdf(pgfplots::Engine::PdfLatex)
//     //     .unwrap();
// }
//
//
fn main() {

}
