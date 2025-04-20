use crate::rk_table::*;
use crate::state::*;

trait DifferentialEquation<const N: usize> {
    fn f(&self, s: &impl State<N>) -> [f64; N];

    fn solution<RK: RungeKuttaTable<S>, const S: usize>(
        &self,
        interval: std::ops::Range<f64>,
        initial_function: &impl Fn(f64) -> [f64; N],
    ) {
    }
}
struct HarmonicOscillator {
    w: f64,
}
impl DifferentialEquation<2> for HarmonicOscillator {
    fn f(&self, s: &impl State<2>) -> [f64; 2] {
        let [x, dx] = s.x();
        [dx, -(self.w).powi(2) * x]
    }
}

#[cfg(test)]
mod test_solver {
    use super::*;

    #[test]
    fn test_solution() {
        let eq = HarmonicOscillator { w: 1. };

        let res = eq.solution::<rk1::Euler, 1>(0.0..10.0, &|t: f64| {
            [(eq.w * t).sin(), eq.w * (eq.w * t).cos()]
        });


        // doesn't compile
        // let res = eq.solution::<rk1::Euler>(0.0..10.0, &|t: f64| {
        //     [(eq.w * t).sin(), eq.w * (eq.w * t).cos()]
        // });
    }
}

// pub trait Solve<const N: usize> {
// fn rhs<State, RK, const S: usize>(self: &Self, s: State) -> [f64; N];
// fn ic(t: f64) -> [f64; N];
// fn solve();
// }

// struct Oscillator {
//     w: f64,
// }

// impl Oscillator {
//     fn rhs<STATE>(&self, s: STATE) -> [f64; 2] {
//         [s.x[1], - w * w * s.x[0]]
//     }
//     fn ic(&self, t: f64) -> [f64; 2] {
//         [(self.w * t).sin(), self.w * (self.w * t).cos()]
//     }
// }

// impl Solver<2, rk1::Euler, 1> for Oscillator {
//
//     fn rhs(&self, s: State<2,rk1::Euler,1>) -> [f64; 2] {
//         return [s.x[1], -self.k*self.k*s.x[0]];
//     }
// }
