// use crate::rk_table::*;
// use crate::state::*;
//
// pub trait DifferentialEquation {
//     const N: usize;
//
//     fn f(&self, s: &impl State<{ Self::N }>) -> [f64; Self::N]
//     where
//         [(); Self::N]:;
//
//     fn solve<RK: RungeKuttaTable, F: Fn(f64) -> [f64; Self::N]>(
//         &self,
//         interval: std::ops::Range<f64>,
//         initial_function: F,
//         _rk: RK,
//         stepsize: f64,
//     ) -> std::collections::VecDeque<[f64; Self::N]>
//     where
//         [(); RK::S]:,
//     {
//         /* initializations */
//         let mut state = RKState::<{ Self::N }, RK, F>::new(interval.start, initial_function);
//
//         state.t_step = stepsize;
//         state.t_span = f64::NAN;
//         /* start event */
//         /* step event */
//
//         // interval.end can be NAN, meaning no end
//         while !(state.t() >= interval.end) {
//             state.make_step(&|s| self.f(s));
//             state.push_current();
//
//             /* step event */
//
//             // state.t_step = std::min(state.t_step, final_time - state.t_curr);
//             //
//             // if (reject_step) {
//             //     events.reject_events(state);
//             //     state.remake_step(&|s| self.f(s))
//             //     continue;
//             // }
//         }
//
//         state.x_seq
//     }
// }
//
// #[cfg(test)]
// mod test_solver {
//     use super::*;
//
//     // #[test]
//     // fn harmonic_oscillator() {
//     // }
// }
