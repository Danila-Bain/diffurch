use crate::equation::*;
use crate::event::*;
use crate::rk::*;
use crate::state::*;
use crate::util::tuple_tower::*;

pub struct Solver<const S: usize = 26, StepEvents = TupleTower<()>> {
    rk: &'static RungeKuttaTable<'static, S>,
    stepsize: f64,
    step_events: StepEvents,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            rk: &RK98,
            stepsize: 0.05,
            step_events: TupleTower(()),
        }
    }
}

impl<const S: usize, StepEvents> Solver<S, TupleTower<StepEvents>> {
    pub fn rk<const S_NEW: usize>(
        self,
        rk: &'static RungeKuttaTable<'static, S_NEW>,
    ) -> Solver<S_NEW, TupleTower<StepEvents>> {
        Solver::<S_NEW, TupleTower<StepEvents>> {
            rk,
            stepsize: self.stepsize,
            step_events: self.step_events,
        }
    }

    pub fn stepsize(self, stepsize: f64) -> Self {
        Self { stepsize, ..self }
    }

    pub fn on_step<E>(self, event: E) -> Solver<S, TupleTower<(E, TupleTower<StepEvents>)>> {
        Solver {
            rk: self.rk,
            stepsize: self.stepsize,
            step_events: self.step_events.append(event),
        }
    }
}

impl<const S: usize, StepEvents> Solver<S, StepEvents> {
    pub fn run<
        const N: usize,
        IC,
        RHS,
        EquationEvents,
        EquationArgs,
        // StepEventsArgs,
        // StepEventsRet,
    >(
        self,
        equation: Equation<N, RHS, EquationEvents>,
        initial_function: IC,
        interval: std::ops::Range<f64>,
    ) where
        IC: Fn(f64) -> [f64; N] + 'static,
        // StepEvents: for<'a> ToStateTupleTower<'a, State<N,S,IC>, StepEventsArgs, StepEventsRet>,
        RHS: Fn<EquationArgs, Output = [f64; N]>,
        EquationArgs: std::marker::Tuple,
        // EquationArgs: for<'a> FromState<&'a State<N, S, IC>>,
        RHS: ToStateFunction<State<N, S, IC>, EquationArgs, [f64; N]>,
    {
        /* initializations */
        let mut state = State::new(interval.start, initial_function, self.rk);
        state.t_step = self.stepsize;
        state.t_span = f64::NAN;

        // ToStateFunction::<&State<N, S, IC>, EquationArgs, [f64; N]>::to_state_function(equation.rhs);
        let mut rhs = equation.rhs.to_state_function();
        // let step_events = self.step_events.to_state_tuple_tower();

        let _equation_events = equation.events;

        /* start event */
        /* step event */

        while state.t < interval.end {
            state.make_step(&mut rhs);
            state.push_current();

            // step_events(&state);
            // self.step_events.call_event_tower((&state,));
            //
            //
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
