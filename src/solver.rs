use crate::Event;
use crate::InitialCondition;
use crate::State;
use crate::equation::Equation;
use crate::rk::{RK98, RungeKuttaTable};
//
pub struct Solver<'a, const N: usize, const S: usize> {
    rk: &'a RungeKuttaTable<'a, S>,
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

    pub fn rk(rk: &'a RungeKuttaTable<'a, S>) -> Solver<'a, N, S> {
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
                subdivision,
            } = event;

            if let Some(n) = subdivision {
                for i in 1..=n {
                    let t = state.t_prev + (state.t - state.t_prev) * (i as f64) / (n as f64);
                    if filter.iter_mut().all(|f| f.eval_at(state, t)) {
                        let output = callback.eval_at(state, t);
                        stream.iter_mut().for_each(|stream| stream(output));
                    }
                }
            } else {
                if filter.iter_mut().all(|f| f.eval(state)) {
                    let output = callback.eval(state);
                    stream.iter_mut().for_each(|stream| stream(output));
                }
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

    pub fn run(
        mut self,
        eq: Equation<'a, N>,
        ic: impl Into<InitialCondition<'a, N>>,
        interval: impl std::ops::RangeBounds<f64>,
    ) {
        use std::ops::Bound::*;
        let t_init = match interval.start_bound() {
            Unbounded => 0.,
            Included(&value) | Excluded(&value) => value,
        };
        let t_end = match interval.end_bound() {
            Unbounded => f64::INFINITY,
            Included(&value) | Excluded(&value) => value,
        };

        let mut state = State::new(t_init, ic.into(), eq, &self.rk);
        let mut stepsize = self.stepsize;

        self.start_events.iter_mut().for_each(|event| event(&state));
        self.step_events.iter_mut().for_each(|event| event(&state));

        while state.t < t_end {
            state.make_step(stepsize);
            state.push_current();
            self.step_events.iter_mut().for_each(|event| event(&state));
            stepsize = stepsize.min(t_end - state.t);
        }

        self.stop_events.iter_mut().for_each(|event| event(&state));
    }
}
