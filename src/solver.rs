use crate::equation::Equation;
use crate::rk::{RK98, RungeKuttaTable};
use crate::state::State;
use crate::util::tutle::{Tutle, TutleLevel};
use crate::{ToStateFn, ToStateTutle};

mod new_solver {
    use super::*;
    pub struct Solver<const S: usize = 26> {
        rk: &'static RungeKuttaTable<'static, S>,
        stepsize: f64,
    }
}
pub struct Solver<
    const S: usize = 26,
    StepEvents = Tutle,
    StartEvents = Tutle,
    StopEvents = Tutle,
> {
    rk: &'static RungeKuttaTable<'static, S>,
    stepsize: f64,
    step_events: StepEvents,
    start_events: StartEvents,
    stop_events: StopEvents,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            rk: &RK98,
            stepsize: 0.05,
            step_events: Tutle(()),
            start_events: Tutle(()),
            stop_events: Tutle(()),
        }
    }
}

impl<const S: usize, StepEvents, StartEvents, StopEvents>
    Solver<S, Tutle<StepEvents>, Tutle<StartEvents>, Tutle<StopEvents>>
{
    pub fn rk<const S_NEW: usize>(
        self,
        rk: &'static RungeKuttaTable<'static, S_NEW>,
    ) -> Solver<S_NEW, Tutle<StepEvents>, Tutle<StartEvents>, Tutle<StopEvents>>
    {
        Solver {
            rk,
            stepsize: self.stepsize,
            step_events: self.step_events,
            start_events: self.start_events,
            stop_events: self.stop_events,
        }
    }

    pub fn stepsize(self, stepsize: f64) -> Self {
        Self { stepsize, ..self }
    }

    pub fn on_step<E>(
        self,
        event: E,
    ) -> Solver<
        S,
        Tutle<(E, Tutle<StepEvents>)>,
        Tutle<StartEvents>,
        Tutle<StopEvents>,
    > {
        Solver {
            rk: self.rk,
            stepsize: self.stepsize,
            step_events: self.step_events.append(event),
            start_events: self.start_events,
            stop_events: self.stop_events,
        }
    }

    pub fn on_start<E>(
        self,
        event: E,
    ) -> Solver<
        S,
        Tutle<StepEvents>,
        Tutle<(E, Tutle<StartEvents>)>,
        Tutle<StopEvents>,
    > {
        Solver {
            rk: self.rk,
            stepsize: self.stepsize,
            step_events: self.step_events,
            start_events: self.start_events.append(event),
            stop_events: self.stop_events,
        }
    }

    pub fn on_stop<E>(
        self,
        event: E,
    ) -> Solver<
        S,
        Tutle<StepEvents>,
        Tutle<StartEvents>,
        Tutle<(E, Tutle<StopEvents>)>,
    > {
        Solver {
            rk: self.rk,
            stepsize: self.stepsize,
            step_events: self.step_events,
            start_events: self.start_events,
            stop_events: self.stop_events.append(event),
        }
    }
}

impl<const S: usize, StepEvents, StartEvents, StopEvents>
    Solver<S, StepEvents, StartEvents, StopEvents>
{
    pub fn run<
        const N: usize,
        IC,
        RHS,
        EquationEvents,
        EquationArgs,
        StepEventsArgs,
        StepEventsRet,
        StartEventsArgs,
        StartEventsRet,
        StopEventsArgs,
        StopEventsRet,
    >(
        self,
        equation: Equation<N, RHS, EquationEvents>,
        initial_function: IC,
        interval: std::ops::Range<f64>,
    ) where
        IC: Fn(f64) -> [f64; N],
        StepEvents: TutleLevel,
        StepEvents:
            ToStateTutle<State<N, S, IC>, StepEventsArgs, StepEventsRet, StepEvents::Level>,
        StartEvents: TutleLevel,
        StartEvents:
            ToStateTutle<State<N, S, IC>, StartEventsArgs, StartEventsRet, StartEvents::Level>,
        StopEvents: TutleLevel,
        StopEvents:
            ToStateTutle<State<N, S, IC>, StopEventsArgs, StopEventsRet, StopEvents::Level>,
        RHS: Fn<EquationArgs, Output = [f64; N]>,
        EquationArgs: std::marker::Tuple,
        // EquationArgs: for<'a> FromState<&'a State<N, S, IC>>,
        RHS: ToStateFn<State<N, S, IC>, EquationArgs, [f64; N]>,
    {
        /* initializations */
        let mut state = State::new(interval.start, initial_function, self.rk);
        state.t_step = self.stepsize;
        state.t_span = equation.max_delay;

        // ToStateFn::<&State<N, S, IC>, EquationArgs, [f64; N]>::to_state_function(equation.rhs);
        let mut rhs = equation.rhs.to_state_function();
        let mut step_events = self.step_events.to_state_tutle();
        let mut start_events = self.start_events.to_state_tutle();
        let mut stop_events = self.stop_events.to_state_tutle();

        let _equation_events = equation.events;

        start_events(&state);
        step_events(&state);

        while state.t < interval.end {
            state.make_step(&mut rhs);
            state.push_current();

            step_events(&state);

            state.t_step = state.t_step.min(interval.end - state.t);
        }

        stop_events(&state);
    }
}
