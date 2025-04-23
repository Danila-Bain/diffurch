use crate::rk::*;

pub struct Solver<const S: usize = 26, StepEvents = ()> {
    rk: &'static RungeKuttaTable<'static, S>,
    stepsize: f64,
    step_events: StepEvents,
}

impl Solver {
    fn new() -> Self {
        Self {
            rk: &RK98,
            stepsize: 0.05,
            step_events: (),
        }
    }
}

impl<const S: usize, StepEvents> Solver<S, StepEvents> {

    fn with_rk<const S_NEW: usize>(
        self,
        rk: &'static RungeKuttaTable<'static, S_NEW>,
    ) -> Solver<S_NEW, StepEvents> {
        Solver::<S_NEW, StepEvents> {
            rk,
            stepsize: self.stepsize,
            step_events: self.step_events,
        }
    }

    fn with_stepsize(self, stepsize: f64) -> Self {
        Self {
            rk: self.rk,
            stepsize,
            step_events: self.step_events,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solver() {
        let solver = Solver::new();
        let solver = Solver::new().with_rk(&RK98).with_stepsize(0.2);
        let solver = Solver::new().with_rk(&DP544).with_stepsize(0.1);
    }
}
