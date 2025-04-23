use crate::state::State;

pub trait StepsizeController {
    fn initial_stepsize (&self) -> f64;
}

pub struct ConstantStepsize(f64);

impl StepsizeController for ConstantStepsize {

    fn initial_stepsize (&self) -> f64 {
        self.0
    }

    // fn set_stepsize<const N: usize>(&self, state: &impl State<N>) {
    // 
    // }
}

struct AdaptiveStepsize{
    atol: f64,
    rtol: f64,
    initial_stepsize: f64,
    safety_factor: f64,
    max_factor: f64,
    max_stepsize: f64,
    min_stepsize: f64,
}

impl AdaptiveStepsize {
    // fn hmm<const N: usize>(&self, state: &impl State<N>) -> bool {
    // let mut error = 0.0f64;
    // for i in 0..N {
    //     let num = (state.x_err[i].abs());
    //     let denom = self.atol + state.x()[i].abs() * self.rtol;
    //     error = error.max(num / denom);
    // }
    //
    // const Q: usize = if RK::ORDER_EMBEDDED < RK::ORDER {
    //     RK::ORDER_EMBEDDED
    // } else {
    //     RK::ORDER
    // };
    //
    // let mut fac = (error * safety_factor).powf(-1.0 / (Q as f64 + 1.0));
    // fac = fac.min(max_factor);
    // fac = fac.max(1.0 / max_factor);
    //
    // state.t_step = (state.t_step * fac).clamp(min_stepsize, max_stepsize);
    //
    // error > 1.0
    // }
}

impl Default for AdaptiveStepsize {
    fn default() -> Self {
        Self {
            atol: 1e-7,
            rtol: 1e-7,
            initial_stepsize: 0.05,
            safety_factor: 4.,
            max_factor: 5.,
            max_stepsize: 10.,
            min_stepsize: 1e-7,
        }
    }
}

impl StepsizeController for AdaptiveStepsize {
    fn initial_stepsize (&self) -> f64 {
        self.initial_stepsize 
    }
}


