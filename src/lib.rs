#![feature(generic_const_exprs)]
#![feature(generic_const_items)]

mod util;
mod rk_table;
mod state;
mod solver;
// mod events;
// mod equation;
//
//
//



// struct HarmonicOscillator {
//     w: f64,
// }
// impl Equation for HarmonicOscillator {
//     fn rhs (&self, s: impl State<2>) {
//         let [x, dx] = &s.x;
//         [
//             dx,
//             -self.w.pow(2)*x,
//         ] 
//     }
//
//     fn ic(&self, t: f64) {
//         [(t*w).sin(), (t*w).cos()*w] 
//     }
// }
//

// #[cfg(test)]
// mod my_state_tests {
//     use super::*;
//
//
//
//
// }
