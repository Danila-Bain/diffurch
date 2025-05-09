use crate::Event;
use crate::State;
use crate::StateFn;
use crate::equation::Equation;
use crate::rk::{RK98, RungeKuttaTable};
//
pub struct Solver<'a, const N: usize, const S: usize> {
    rk: &'a RungeKuttaTable<'static, S>,
    stepsize: f64,
    // step_events: Vec<StateFn<'a, N, ()>>,
    step_events: Vec<Box<dyn 'a + for<'s> FnMut(&'s State<N, S>)>>,
    start_events: Vec<Box<dyn 'a + for<'s> FnMut(&'s State<N, S>)>>,
    stop_events: Vec<Box<dyn 'a + for<'s> FnMut(&'s State<N, S>)>>,
}

impl<'a, const N: usize, const S: usize> Solver<'a, N, S> {
    pub fn new() -> Solver<'a, N, 26> {
        Solver::<N, 26> {
            rk: &RK98,
            stepsize: 0.05,
            step_events: Vec::new(),
            start_events: Vec::new(),
            stop_events: Vec::new(),
        }
    }

    pub fn rk(rk: &'a RungeKuttaTable<'static, S>) -> Solver<'a, N, S> {
        Solver {
            rk,
            stepsize: 0.05,
            step_events: Vec::new(),
            start_events: Vec::new(),
            stop_events: Vec::new(),
        }
    }

    pub fn stepsize(self, stepsize: f64) -> Self {
        Self { stepsize, ..self }
    }

    fn event_to_state_function<'c, Output: 'c + Copy>(
        mut event: Event<'c, N, Output>,
    ) -> Box<dyn 'c + for<'b> FnMut(&'b State<N, S>)> {
        Box::new(move |state: &State<N, S>| {
            let Event {
                ref mut callback,
                ref mut stream,
                ref mut filter,
                subdivision: _,
            } = event;

            if filter.iter_mut().all(|f| f.eval(state)) {
                let output = callback.eval(state);
                stream.iter_mut().for_each(|stream| stream(output));
            }
        })
    }

    pub fn on_step<Output: Copy + 'a>(mut self, event: Event<'a, N, Output>) -> Self {
        self.step_events.push(Self::event_to_state_function(event));
        self
    }
    pub fn on_start<Output: Copy + 'a>(mut self, event: Event<'a, N, Output>) -> Self {
        self.start_events.push(Self::event_to_state_function(event));
        self
    }
    pub fn on_stop<Output: Copy + 'a>(mut self, event: Event<'a, N, Output>) -> Self {
        self.stop_events.push(Self::event_to_state_function(event));
        self
    }

//     pub fn run(
//         self,
//         equation: Equation<'a, N>,
//         initial_function: f64,
//         interval: impl std::ops::RangeBounds<f64>,
//     ) {
//
//         /* initializations */
//
//         let t_init = match interval.start_bound() {
//             std::ops::Bound::Unbounded => 0.,
//             std::ops::Bound::Included(value) => *value,
//             std::ops::Bound::Excluded(value) => *value,
//         };
//
//         let t_end = match interval.end_bound() {
//             std::ops::Bound::Unbounded => f64::MAX,
//             std::ops::Bound::Included(value) => *value,
//             std::ops::Bound::Excluded(value) => *value,
//         };
//
//         let mut state = State::new(t_init, initial_function, self.rk);
//         state.t_step = self.stepsize;
//         state.t_span = equation.max_delay;
//
//         // ToStateFn::<&State<N, S, IC>, EquationA, [f64; N]>::to_state_function(equation.rhs);
//         let mut rhs = equation.rhs.to_state_function();
//         let mut step_events = self.step_events.to_state_tutle();
//         let mut start_events = self.start_events.to_state_tutle();
//         let mut stop_events = self.stop_events.to_state_tutle();
//
//         let _equation_events = equation.events;
//
//         start_events(&state);
//         step_events(&state);
//
//         while state.t < t_end {
//             state.make_step(&mut rhs);
//             state.push_current();
//             step_events(&state);
//             state.t_step = state.t_step.min(t_end - state.t);
//         }
//
//         stop_events(&state);
//     }
// }
}

// impl<const S: usize, StepE, StartE, StopE, RootE> Solver<S, StepE, StartE, StopE, RootE> {
//     pub fn run<
//         const N: usize,
//         Interval,
//         IC,
//         RHS,
//         EquationE,
//         EquationA,
//         StepEA,
//         StepER,
//         StartEA,
//         StartER,
//         StopEA,
//         StopER,
//     >(
//         self,
//         equation: Equation<N, RHS, EquationE>,
//         initial_function: IC,
//         interval: Interval,
//     ) where
//         Interval: std::ops::RangeBounds<f64>,
//         IC: Fn(f64) -> [f64; N],
//         StepE: TutleLevel,
//         StepE: ToStateTutle<State<N, S, IC>, StepEA, StepER, StepE::Level>,
//         StartE: TutleLevel,
//         StartE: ToStateTutle<State<N, S, IC>, StartEA, StartER, StartE::Level>,
//         StopE: TutleLevel,
//         StopE: ToStateTutle<State<N, S, IC>, StopEA, StopER, StopE::Level>,
//         RHS: Fn<EquationA, Output = [f64; N]>,
//         EquationA: std::marker::Tuple,
//         // EquationA: for<'a> FromState<&'a State<N, S, IC>>,
//         RHS: ToStateFn<State<N, S, IC>, EquationA, [f64; N]>,
//     {
//         /* initializations */
//
//         let t_init = match interval.start_bound() {
//             std::ops::Bound::Unbounded => 0.,
//             std::ops::Bound::Included(value) => *value,
//             std::ops::Bound::Excluded(value) => *value,
//         };
//
//         let t_end = match interval.end_bound() {
//             std::ops::Bound::Unbounded => f64::MAX,
//             std::ops::Bound::Included(value) => *value,
//             std::ops::Bound::Excluded(value) => *value,
//         };
//
//         let mut state = State::new(t_init, initial_function, self.rk);
//         state.t_step = self.stepsize;
//         state.t_span = equation.max_delay;
//
//         // ToStateFn::<&State<N, S, IC>, EquationA, [f64; N]>::to_state_function(equation.rhs);
//         let mut rhs = equation.rhs.to_state_function();
//         let mut step_events = self.step_events.to_state_tutle();
//         let mut start_events = self.start_events.to_state_tutle();
//         let mut stop_events = self.stop_events.to_state_tutle();
//
//         let _equation_events = equation.events;
//
//         start_events(&state);
//         step_events(&state);
//
//         while state.t < t_end {
//             state.make_step(&mut rhs);
//             state.push_current();
//             step_events(&state);
//             state.t_step = state.t_step.min(t_end - state.t);
//         }
//
//         stop_events(&state);
//     }
// }
