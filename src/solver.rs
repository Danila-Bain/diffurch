use crate::equation::Equation;
use crate::rk::{RK98, RungeKuttaTable};
use crate::state::State;
use crate::util::tutle::{Tutle, TutleLevel};
use crate::{ToStateFn, ToStateTutle};

pub struct Solver<const S: usize = 26, StepE = Tutle, StartE = Tutle, StopE = Tutle, RootE = Tutle> {
    rk: &'static RungeKuttaTable<'static, S>,
    stepsize: f64,
    step_events: StepE,
    start_events: StartE,
    stop_events: StopE,
    root_events: RootE,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            rk: &RK98,
            stepsize: 0.05,
            step_events: Tutle(()),
            start_events: Tutle(()),
            stop_events: Tutle(()),
            root_events: Tutle(()),
        }
    }
}

impl<const S: usize, StepE, StartE, StopE, RootE> Solver<S, Tutle<StepE>, Tutle<StartE>, Tutle<StopE>, Tutle<RootE>> {
    pub fn rk<const S_NEW: usize>(
        self,
        rk: &'static RungeKuttaTable<'static, S_NEW>,
    ) -> Solver<S_NEW, Tutle<StepE>, Tutle<StartE>, Tutle<StopE>, Tutle<RootE>> {
        Solver {
            rk,
            stepsize: self.stepsize,
            step_events: self.step_events,
            start_events: self.start_events,
            stop_events: self.stop_events,
            root_events: self.root_events,
        }
    }

    pub fn stepsize(self, stepsize: f64) -> Self {
        Self { stepsize, ..self }
    }

    pub fn on_step<E>(
        self,
        event: E,
    ) -> Solver<S, Tutle<(E, Tutle<StepE>)>, Tutle<StartE>, Tutle<StopE>, Tutle<RootE>> {
        Solver {
            rk: self.rk,
            stepsize: self.stepsize,
            step_events: self.step_events.append(event),
            start_events: self.start_events,
            stop_events: self.stop_events,
            root_events: self.root_events,
        }
    }

    pub fn on_start<E>(
        self,
        event: E,
    ) -> Solver<S, Tutle<StepE>, Tutle<(E, Tutle<StartE>)>, Tutle<StopE>, Tutle<RootE>> {
        Solver {
            rk: self.rk,
            stepsize: self.stepsize,
            step_events: self.step_events,
            start_events: self.start_events.append(event),
            stop_events: self.stop_events,
            root_events: self.root_events,
        }
    }

    pub fn on_stop<E>(
        self,
        event: E,
    ) -> Solver<S, Tutle<StepE>, Tutle<StartE>, Tutle<(E, Tutle<StopE>)>, Tutle<RootE>> {
        Solver {
            rk: self.rk,
            stepsize: self.stepsize,
            step_events: self.step_events,
            start_events: self.start_events,
            stop_events: self.stop_events.append(event),
            root_events: self.root_events,
        }
    }


    pub fn on_root<R,E>(
        self,
        root: R,
        event: E,
    ) -> Solver<S, Tutle<StepE>, Tutle<StartE>, Tutle<StopE>, Tutle<((R,E), Tutle<RootE>)>> {
        Solver {
            rk: self.rk,
            stepsize: self.stepsize,
            step_events: self.step_events,
            start_events: self.start_events,
            stop_events: self.stop_events,
            root_events: self.root_events.append((root, event)),
        }
    }
}

impl<const S: usize, StepE, StartE, StopE, RootE> Solver<S, StepE, StartE, StopE, RootE> {
    pub fn run<
        const N: usize,
        Interval,
        IC,
        RHS,
        EquationE,
        EquationA,
        StepEA,
        StepER,
        StartEA,
        StartER,
        StopEA,
        StopER,
    >(
        self,
        equation: Equation<N, RHS, EquationE>,
        initial_function: IC,
        interval: Interval,
    ) where
        Interval: std::ops::RangeBounds<f64>,
        IC: Fn(f64) -> [f64; N],
        StepE: TutleLevel,
        StepE: ToStateTutle<State<N, S, IC>, StepEA, StepER, StepE::Level>,
        StartE: TutleLevel,
        StartE: ToStateTutle<State<N, S, IC>, StartEA, StartER, StartE::Level>,
        StopE: TutleLevel,
        StopE: ToStateTutle<State<N, S, IC>, StopEA, StopER, StopE::Level>,
        RHS: Fn<EquationA, Output = [f64; N]>,
        EquationA: std::marker::Tuple,
        // EquationA: for<'a> FromState<&'a State<N, S, IC>>,
        RHS: ToStateFn<State<N, S, IC>, EquationA, [f64; N]>,
    {
        /* initializations */

        let t_init = match interval.start_bound() {
            std::ops::Bound::Unbounded => 0.,
            std::ops::Bound::Included(value) => *value,
            std::ops::Bound::Excluded(value) => *value,
        };

        let t_end = match interval.end_bound() {
            std::ops::Bound::Unbounded => f64::MAX,
            std::ops::Bound::Included(value) => *value,
            std::ops::Bound::Excluded(value) => *value,
        };

        let mut state = State::new(t_init, initial_function, self.rk);
        state.t_step = self.stepsize;
        state.t_span = equation.max_delay;

        // ToStateFn::<&State<N, S, IC>, EquationA, [f64; N]>::to_state_function(equation.rhs);
        let mut rhs = equation.rhs.to_state_function();
        let mut step_events = self.step_events.to_state_tutle();
        let mut start_events = self.start_events.to_state_tutle();
        let mut stop_events = self.stop_events.to_state_tutle();

        let _equation_events = equation.events;

        start_events(&state);
        step_events(&state);

        while state.t < t_end {
            state.make_step(&mut rhs);
            state.push_current();

            step_events(&state);

            state.t_step = state.t_step.min(t_end - state.t);
        }

        stop_events(&state);
    }
}
