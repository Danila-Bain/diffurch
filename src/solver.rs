use crate::equation::Equation;
use crate::rk::{RK98, RungeKuttaTable};
use crate::state::State;
use crate::util::tuple_tower::{TupleTower, TupleTowerLevel};
use crate::{ToStateFunction, ToStateTupleTower};

pub struct Solver<
    const S: usize = 26,
    StepEvents = TupleTower,
    StartEvents = TupleTower,
    StopEvents = TupleTower,
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
            step_events: TupleTower(()),
            start_events: TupleTower(()),
            stop_events: TupleTower(()),
        }
    }
}

impl<const S: usize, StepEvents, StartEvents, StopEvents> Solver<S, TupleTower<StepEvents>, TupleTower<StartEvents>, TupleTower<StopEvents>> {
    pub fn rk<const S_NEW: usize>(
        self,
        rk: &'static RungeKuttaTable<'static, S_NEW>,
    ) -> Solver<S_NEW, TupleTower<StepEvents>, TupleTower<StartEvents>, TupleTower<StopEvents>> {
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

    pub fn on_step<E>(self, event: E) -> Solver<S, TupleTower<(E, TupleTower<StepEvents>)>, TupleTower<StartEvents>, TupleTower<StopEvents> > {
        Solver {
            rk: self.rk,
            stepsize: self.stepsize,
            step_events: self.step_events.append(event),
            start_events: self.start_events,
            stop_events: self.stop_events,
        }
    }


    pub fn on_start<E>(self, event: E) -> Solver<S, TupleTower<StepEvents>, TupleTower<(E, TupleTower<StartEvents>)>, TupleTower<StopEvents> > {
        Solver {
            rk: self.rk,
            stepsize: self.stepsize,
            step_events: self.step_events,
            start_events: self.start_events.append(event),
            stop_events: self.stop_events,
        }
    }


    pub fn on_stop<E>(self, event: E) -> Solver<S, TupleTower<StepEvents>,  TupleTower<StartEvents>, TupleTower<(E, TupleTower<StopEvents>)>> {
        Solver {
            rk: self.rk,
            stepsize: self.stepsize,
            step_events: self.step_events,
            start_events: self.start_events,
            stop_events: self.stop_events.append(event),
        }
    }
}

impl<const S: usize, StepEvents, StartEvents, StopEvents> Solver<S, StepEvents, StartEvents, StopEvents> {
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
        StepEvents: TupleTowerLevel,
        StepEvents:
            ToStateTupleTower<State<N, S, IC>, StepEventsArgs, StepEventsRet, StepEvents::Level>,
        StartEvents: TupleTowerLevel,
        StartEvents:
            ToStateTupleTower<State<N, S, IC>, StartEventsArgs, StartEventsRet, StartEvents::Level>,
        StopEvents: TupleTowerLevel,
        StopEvents:
            ToStateTupleTower<State<N, S, IC>, StopEventsArgs, StopEventsRet, StopEvents::Level>,
        RHS: Fn<EquationArgs, Output = [f64; N]>,
        EquationArgs: std::marker::Tuple,
        // EquationArgs: for<'a> FromState<&'a State<N, S, IC>>,
        RHS: ToStateFunction<State<N, S, IC>, EquationArgs, [f64; N]>,
    {
        /* initializations */
        let mut state = State::new(interval.start, initial_function, self.rk);
        state.t_step = self.stepsize;
        state.t_span = equation.max_delay;

        // ToStateFunction::<&State<N, S, IC>, EquationArgs, [f64; N]>::to_state_function(equation.rhs);
        let mut rhs = equation.rhs.to_state_function();
        let mut step_events = self.step_events.to_state_tuple_tower();
        let mut start_events = self.start_events.to_state_tuple_tower();
        let mut stop_events = self.stop_events.to_state_tuple_tower();

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

