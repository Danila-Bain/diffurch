//! Defines [Solver].

use crate::rk::{RK98, RungeKuttaTable};
use crate::*;
use hlist2::ops::{Append, Extend};
use hlist2::{HList, Nil};

/// Implements the integration of differential equation, containing the implementation specific (not
/// equation specific) data, including particular Runge-Kutta scheme, stepsize, and events.
pub struct Solver<
    'a,
    const N: usize,
    const S: usize,
    EventsOnStep: HList = Nil,
    EventsOnStart: HList = Nil,
    EventsOnStop: HList = Nil,
    EventsOnLoc: HList = Nil,
> where
    [(); S * (S - 1) / 2]:,
{
    /// Runge-Kutta scheme used during integration. See [crate::rk].
    ///
    /// Set in constructor [Solver::rk].
    pub rk: &'a RungeKuttaTable<S>,
    /// Stepsize used during integration. In the future may be replaced with more generic stepsize
    /// controller.
    ///
    /// Set in setter [Solver::stepsize].
    pub stepsize: f64,
    /// Events, that trigger each completed (not rejected) step.
    ///
    /// See [Solver::on_step].
    pub step_events: EventsOnStep,
    /// Events, that trigger before the start of integration.
    ///
    /// See [Solver::on_start].
    pub start_events: EventsOnStart,
    /// Events, that trigger after the stop of integration.
    ///
    /// See [Solver::on_stop].
    pub stop_events: EventsOnStop,
    /// Events, which trigger on located event during integration, like when the solution crosses
    /// some surface in phase space.
    ///
    /// See [Solver::on_stop].
    pub loc_events: EventsOnLoc,
}

impl<'a, const N: usize> Solver<'a, N, 26> {
    /// Constructor which defaults Runge-Kutta scheme to [crate::rk::RK98],
    /// and stepsize to 0.05.
    pub fn new() -> Self {
        Solver::<N, 26> {
            rk: &RK98,
            stepsize: 0.05,
            step_events: Nil,
            start_events: Nil,
            stop_events: Nil,
            loc_events: Nil,
        }
    }
}

impl<
    'a,
    const N: usize,
    const S: usize,
    EventsOnStep: HList,
    EventsOnStart: HList,
    EventsOnStop: HList,
    EventsOnLoc: HList,
> Solver<'a, N, S, EventsOnStep, EventsOnStart, EventsOnStop, EventsOnLoc>
where
    [(); S * (S - 1) / 2]:,
{
    /// Self-consuming setter for [Self::rk] field
    pub fn rk<const S_: usize>(
        self,
        rk: &'a RungeKuttaTable<S_>,
    ) -> Solver<'a, N, S_, EventsOnStep, EventsOnStart, EventsOnStop, EventsOnLoc>
    where
        [(); S_ * (S_ - 1) / 2]:,
    {
        Solver {
            rk,
            stepsize: 0.05,
            step_events: self.step_events,
            start_events: self.start_events,
            stop_events: self.stop_events,
            loc_events: self.loc_events,
        }
    }

    /// [Solver::stepsize] setter. Returns self.
    pub fn stepsize(self, stepsize: f64) -> Self {
        Self { stepsize, ..self }
    }

    /// Add event to a list of step events.
    /// Events in that list trigger once before the first step, and then after each completed step.
    /// The step may be not completed if it were rejected by a step size controller (currently
    /// unimplemented), or located event (see [Solver::on_loc]).
    ///
    pub fn on_step<E: EventCall<N>>(
        self,
        event: E,
    ) -> Solver<
        'a,
        N,
        S,
        <EventsOnStep as Append>::Output<E>,
        EventsOnStart,
        EventsOnStop,
        EventsOnLoc,
    >
    where
        EventsOnStep: Append,
    {
        let Solver {
            rk,
            stepsize,
            step_events,
            start_events,
            stop_events,
            loc_events,
        } = self;

        Solver {
            rk,
            stepsize,
            step_events: step_events.append(event),
            start_events,
            stop_events,
            loc_events,
        }
    }

    /// Add event to a list of start events.
    /// Events in that list trigger before the start of integration
    /// and before the first trigger of step events (see [Solver::on_step]).
    pub fn on_start<E: EventCall<N>>(
        self,
        event: E,
    ) -> Solver<
        'a,
        N,
        S,
        EventsOnStep,
        <EventsOnStart as Append>::Output<E>,
        EventsOnStop,
        EventsOnLoc,
    >
    where
        EventsOnStart: Append,
    {
        let Solver {
            rk,
            stepsize,
            step_events,
            start_events,
            stop_events,
            loc_events,
        } = self;

        Solver {
            rk,
            stepsize,
            step_events,
            start_events: start_events.append(event),
            stop_events,
            loc_events,
        }
    }
    /// Add event to a list of stop events.
    /// Events in that list trigger after the last step in integration has been made.
    pub fn on_stop<E: EventCall<N>>(
        self,
        event: E,
    ) -> Solver<
        'a,
        N,
        S,
        EventsOnStep,
        EventsOnStart,
        <EventsOnStop as Append>::Output<E>,
        EventsOnLoc,
    >
    where
        EventsOnStop: Append,
    {
        let Solver {
            rk,
            stepsize,
            step_events,
            start_events,
            stop_events,
            loc_events,
        } = self;

        Solver {
            rk,
            stepsize,
            step_events,
            start_events,
            stop_events: stop_events.append(event),
            loc_events,
        }
    }

    /// Add event to a list of loc events.
    /// Events in that list trigger when event is located on a step using [Loc]. If two or more
    /// events are detected on a step, only the earliest one is triggered. In current
    /// implementation, solver always steps on the located event. Which can be used to implement
    /// numerical integration for discontinuous differential equations correctly.
    pub fn on_loc<L: Locate<N>, E: EventCall<N>>(
        self,
        event_locator: L,
        event: E,
    ) -> Solver<
        'a,
        N,
        S,
        EventsOnStep,
        EventsOnStart,
        EventsOnStop,
        <EventsOnLoc as Append>::Output<(L, E)>,
    >
    where
        EventsOnLoc: Append,
    {
        let Solver {
            rk,
            stepsize,
            step_events,
            start_events,
            stop_events,
            loc_events,
        } = self;

        Solver {
            rk,
            stepsize,
            step_events,
            start_events,
            stop_events,
            loc_events: loc_events.append((event_locator, event)),
        }
    }

    /// Run solver.
    pub fn run<RHS: StateFnMut<N, Output = [f64; N]>, Propagations: HList, EventsEquation>(
        mut self,
        eq: Equation<N, RHS, Propagations, EventsEquation>,
        ic: impl InitialCondition<N>,
        interval: impl std::ops::RangeBounds<f64>,
    ) where
        EventsOnStep: EventHList<N>,
        EventsOnStart: EventHList<N>,
        EventsOnStop: EventHList<N>,
        EventsOnLoc: LocEventHList<N> + Extend,
        EventsEquation: LocEventHList<N>,
        <EventsOnLoc as Extend>::Output<EventsEquation>: LocEventHList<N> + Extend,
        Propagations: LocEventHList<N>,
        <<EventsOnLoc as Extend>::Output<EventsEquation> as Extend>::Output::<Propagations> : LocEventHList<N>,
    {
        use std::ops::Bound::*;
        let t_init = match interval.start_bound() {
            Unbounded => 0.,
            Included(&value) | Excluded(&value) => value,
        };
        let t_end = match interval.end_bound() {
            Unbounded => f64::INFINITY,
            Included(&value) | Excluded(&value) => value,
        };

        let mut rhs = eq.rhs;
        let mut state = RKState::new(t_init, ic, eq.max_delay, &self.rk);
        let mut stepsize = self.stepsize;

        let mut loc_events = self.loc_events.extend(eq.events).extend(eq.propagations);

        // MAKE discontinuities FIELD IN STATE, SO PROPAGATION EVENTS CAN ACCESS IT
        // let mut loc_events = self.loc_events.extend(eq.events).extend(PropagatedEach(eq.delays));

        self.start_events.call_each(&mut state);
        self.step_events.call_each(&mut state);

        while state.t() < t_end {
            state.make_step(&mut rhs, stepsize);

            if let Some((t, event)) = loc_events.locate_first(&mut state) && t > state.t_prev() {
                println!("event located at {t}, state.disco: {:?}", state.disco());
                state.undo_step();
                state.make_step(&mut rhs, t - state.t);
                state.push_current();
                self.step_events.call_each(&mut state);
                event.call(&mut state); 
                if state.t_prev() == state.t() {
                    self.step_events.call_each(&mut state);
                    state.disco_mut().push_back((t, 0))
                }
            } else {
                state.push_current();
                self.step_events.call_each(&mut state);
            }

            stepsize = stepsize.min(t_end - state.t);
        }

        self.stop_events.call_each(&mut state);
    }
}
