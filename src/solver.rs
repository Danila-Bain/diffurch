use crate::equation::*;
use crate::event::*;
use crate::rk::*;
use crate::state::*;

pub struct Solver<const S: usize = 26, StepEvents = ()> {
    rk: &'static RungeKuttaTable<'static, S>,
    stepsize: f64,
    step_events: StepEvents,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            rk: &RK98,
            stepsize: 0.05,
            step_events: (),
        }
    }
}

impl<const S: usize, StepEvents> Solver<S, StepEvents> {
    pub fn rk<const S_NEW: usize>(
        self,
        rk: &'static RungeKuttaTable<'static, S_NEW>,
    ) -> Solver<S_NEW, StepEvents> {
        Solver::<S_NEW, StepEvents> {
            rk,
            stepsize: self.stepsize,
            step_events: self.step_events,
        }
    }

    pub fn stepsize(self, stepsize: f64) -> Self {
        Self { stepsize, ..self }
    }

    pub fn on_step<E>(self, event: E) -> Solver<S, (E, StepEvents)> {
        Solver::<S, (E, StepEvents)> {
            rk: self.rk,
            stepsize: self.stepsize,
            step_events: (event, self.step_events),
        }
    }
}

impl<const S: usize, StepEvents> Solver<S, StepEvents> {
    pub fn run<const N: usize, IC, RHS, EquationEvents,  EquationArgs>(
        &mut self,
        equation: Equation<N, RHS, EquationEvents>,
        initial_function: IC,
        interval: std::ops::Range<f64>,
    ) where
        IC: Fn(f64) -> [f64; N],
        StepEvents: for<'a> CallEventTower<(&'a State<N, S, IC>,)>,
        RHS: Fn<EquationArgs, Output = [f64; N]>,
        EquationArgs: std::marker::Tuple + for<'a> FromState<&'a State<N, S, IC>>,
    {
        /* initializations */
        let mut state = State::new(interval.start, initial_function, self.rk);
        state.t_step = self.stepsize;
        state.t_span = f64::NAN;

        // let rhs = to_state_function(equation.rhs);
        let _equation_events = equation.events;

        /* start event */
        /* step event */

        while state.t < interval.end {
            state.make_step(&equation.rhs);
            state.push_current();

            self.step_events.call_event_tower((&state,));
            // call_event_tower(self.step_events, (&state,));

            /* step event */

            // state.t_step = std::min(state.t_step, final_time - state.t_curr);
            //
            // if (reject_step) {
            //     events.reject_events(state);
            //     state.remake_step(&|s| self.f(s))
            //     continue;
            // }
        }


        /* stop event */
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solver() {
        let _solver = Solver::new();
        let _solver = Solver::new().rk(&RK98).stepsize(0.2);
        let _solver = Solver::new().rk(&DP544).stepsize(0.1);

        let _solver = Solver {
            rk: &HEUN3,
            stepsize: 0.05,
            step_events: (),
        };
        let _solver = Solver {
            rk: &RKTP64,
            stepsize: 0.05,
            step_events: (),
        };

        // let solver = Solver { rk: &RKTP64, step_events: (), ..};
    }
}
