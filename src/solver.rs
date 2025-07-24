//! Defines [Solver].

use crate::collections::hlists::{EventHList, LocEventHList};
use crate::delay::IntoDelay;
use crate::rk::RungeKuttaTable;
use crate::*;
use hlist2::ops::Append;
use hlist2::{HList, Nil};

/// Implements the integration of differential equation, containing the implementation specific (not
/// equation specific) data, including particular Runge-Kutta scheme, stepsize, and events.
pub struct Solver<
    'a,
    const N: usize,
    const S: usize,
    Equation = (),
    Initial = (),
    Interval = (),
    EventsOnStep: HList = Nil,
    EventsOnStart: HList = Nil,
    EventsOnStop: HList = Nil,
    EventsOnLoc: HList = Nil,
> where
    [(); S * (S - 1) / 2]:,
{
    pub equation: Equation,
    pub initial: Initial,
    pub initial_disco: Vec<(f64, usize)>,
    pub interval: Interval,
    pub max_delay: f64,
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

impl<const N: usize> Solver<'static, N, 7> {
    /// Constructor which defaults Runge-Kutta scheme to [crate::rk::RKTP64],
    /// and stepsize to 0.05.
    pub fn new() -> Self {
        Solver::<N, 7> {
            rk: &rk::RKTP64,
            stepsize: 0.05,
            equation: (),
            initial: (),
            initial_disco: vec!(),
            interval: (),
            max_delay: f64::NAN,
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
    Equation,
    Initial,
    Interval,
    EventsOnStep: HList,
    EventsOnStart: HList,
    EventsOnStop: HList,
    EventsOnLoc: HList,
>
    Solver<
        'a,
        N,
        S,
        Equation,
        Initial,
        Interval,
        EventsOnStep,
        EventsOnStart,
        EventsOnStop,
        EventsOnLoc,
    >
where
    [(); S * (S - 1) / 2]:,
{
    pub fn equation<NewEquation>(
        self,
        equation: NewEquation,
    ) -> Solver<
        'a,
        N,
        S,
        NewEquation,
        Initial,
        Interval,
        EventsOnStep,
        EventsOnStart,
        EventsOnStop,
        EventsOnLoc,
    > {
        let Solver {
            equation: _,
            initial,
            initial_disco,
            interval,
            max_delay,
            rk,
            stepsize,
            step_events,
            start_events,
            stop_events,
            loc_events,
        } = self;

        Solver {
            equation,
            initial,
            initial_disco,
            interval,
            max_delay,
            rk,
            stepsize,
            step_events,
            start_events,
            stop_events,
            loc_events,
        }
    }
    pub fn initial<NewInitial>(
        self,
        initial: NewInitial,
    ) -> Solver<
        'a,
        N,
        S,
        Equation,
        NewInitial,
        Interval,
        EventsOnStep,
        EventsOnStart,
        EventsOnStop,
        EventsOnLoc,
    > {
        let Solver {
            equation,
            initial: _,
            initial_disco,
            interval,
            max_delay,
            rk,
            stepsize,
            step_events,
            start_events,
            stop_events,
            loc_events,
        } = self;

        Solver {
            equation,
            initial,
            initial_disco,
            interval,
            max_delay,
            rk,
            stepsize,
            step_events,
            start_events,
            stop_events,
            loc_events,
        }
    }

    /// [Solver::initial_disco] setter. Returns self.
    pub fn initial_disco(self, initial_disco: impl Into<Vec<(f64, usize)>>) -> Self {
        Self { initial_disco: initial_disco.into(), ..self }
    }
    pub fn interval<NewInterval>(
        self,
        interval: NewInterval,
    ) -> Solver<
        'a,
        N,
        S,
        Equation,
        Initial,
        NewInterval,
        EventsOnStep,
        EventsOnStart,
        EventsOnStop,
        EventsOnLoc,
    > {
        let Solver {
            equation,
            initial,
            initial_disco,
            interval: _,
            max_delay,
            rk,
            stepsize,
            step_events,
            start_events,
            stop_events,
            loc_events,
        } = self;

        Solver {
            equation,
            initial,
            initial_disco,
            interval,
            max_delay,
            rk,
            stepsize,
            step_events,
            start_events,
            stop_events,
            loc_events,
        }
    }

    /// Self-consuming setter for [Self::rk] field
    pub fn rk<const S_: usize>(
        self,
        rk: &'a RungeKuttaTable<S_>,
    ) -> Solver<
        'a,
        N,
        S_,
        Equation,
        Initial,
        Interval,
        EventsOnStep,
        EventsOnStart,
        EventsOnStop,
        EventsOnLoc,
    >
    where
        [(); S_ * (S_ - 1) / 2]:,
    {
        let Solver {
            equation,
            initial,
            initial_disco,
            interval,
            max_delay,
            rk: _,
            stepsize,
            step_events,
            start_events,
            stop_events,
            loc_events,
        } = self;

        Solver {
            equation,
            initial,
            initial_disco,
            interval,
            max_delay,
            rk,
            stepsize,
            step_events,
            start_events,
            stop_events,
            loc_events,
        }
    }

    /// [Solver::stepsize] setter. Returns self.
    pub fn stepsize(self, stepsize: f64) -> Self {
        Self { stepsize, ..self }
    }

    /// [Solver::max_delay] setter. Returns self.
    pub fn max_delay(self, max_delay: f64) -> Self {
        Self { max_delay, ..self }
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
        Equation,
        Initial,
        Interval,
        <EventsOnStep as Append>::Output<E>,
        EventsOnStart,
        EventsOnStop,
        EventsOnLoc,
    >
    where
        EventsOnStep: Append,
    {
        let Solver {
            equation,
            initial,
            initial_disco,
            interval,
            max_delay,
            rk,
            stepsize,
            step_events,
            start_events,
            stop_events,
            loc_events,
        } = self;

        Solver {
            equation,
            initial,
            initial_disco,
            interval,
            max_delay,
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
        Equation,
        Initial,
        Interval,
        EventsOnStep,
        <EventsOnStart as Append>::Output<E>,
        EventsOnStop,
        EventsOnLoc,
    >
    where
        EventsOnStart: Append,
    {
        let Solver {
            equation,
            initial,
            initial_disco,
            interval,
            max_delay,
            rk,
            stepsize,
            step_events,
            start_events,
            stop_events,
            loc_events,
        } = self;

        Solver {
            equation,
            initial,
            initial_disco,
            interval,
            max_delay,
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
        Equation,
        Initial,
        Interval,
        EventsOnStep,
        EventsOnStart,
        <EventsOnStop as Append>::Output<E>,
        EventsOnLoc,
    >
    where
        EventsOnStop: Append,
    {
        let Solver {
            equation,
            initial,
            initial_disco,
            interval,
            max_delay,
            rk,
            stepsize,
            step_events,
            start_events,
            stop_events,
            loc_events,
        } = self;

        Solver {
            equation,
            initial,
            initial_disco,
            interval,
            max_delay,
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
    pub fn on<L: Locate<N>, E: EventCall<N>>(
        self,
        event_locator: L,
        event: E,
    ) -> Solver<
        'a,
        N,
        S,
        Equation,
        Initial,
        Interval,
        EventsOnStep,
        EventsOnStart,
        EventsOnStop,
        <EventsOnLoc as Append>::Output<(L, E)>,
    >
    where
        EventsOnLoc: Append,
    {
        let Solver {
            equation,
            initial,
            initial_disco,
            interval,
            max_delay,
            rk,
            stepsize,
            step_events,
            start_events,
            stop_events,
            loc_events,
        } = self;

        Solver {
            equation,
            initial,
            initial_disco,
            interval,
            max_delay,
            rk,
            stepsize,
            step_events,
            start_events,
            stop_events,
            loc_events: loc_events.append((event_locator, event)),
        }
    }

    pub fn delay_with_smoothing_order<D: IntoDelay<N>>(
        self,
        value: D,
        smoothing_order: usize,
    ) -> Solver<
        'a,
        N,
        S,
        Equation,
        Initial,
        Interval,
        EventsOnStep,
        EventsOnStart,
        EventsOnStop,
        <EventsOnLoc as Append>::Output<
            Loc<Propagator<N, <D as IntoDelay<N>>::Output>, Propagation, D::LocationMethod>,
        >,
    >
    where
        EventsOnLoc: Append,
    {
        let new_max_delay = value.max_delay();
        let location_method = value.location();

        let propagator = loc::Loc(
            loc::Propagator::new(value.into_delay(), smoothing_order),
            loc::Propagation,
            location_method,
        );

        let Solver {
            equation,
            initial,
            initial_disco,
            interval,
            mut max_delay,
            rk,
            stepsize,
            step_events,
            start_events,
            stop_events,
            loc_events,
        } = self;

        if !new_max_delay.is_nan() {
            if max_delay.is_nan() {
                max_delay = new_max_delay;
            } else {
                max_delay = max_delay.max(new_max_delay)
            }
        }

        Solver {
            equation,
            initial,
            initial_disco,
            interval,
            max_delay,
            rk,
            stepsize,
            step_events,
            start_events,
            stop_events,
            loc_events: loc_events.append(propagator),
        }
    }

    pub fn delay<D: IntoDelay<N>>(
        self,
        value: D,
    ) -> Solver<
        'a,
        N,
        S,
        Equation,
        Initial,
        Interval,
        EventsOnStep,
        EventsOnStart,
        EventsOnStop,
        <EventsOnLoc as Append>::Output<
            Loc<Propagator<N, <D as IntoDelay<N>>::Output>, Propagation, D::LocationMethod>,
        >,
    >
    where
        EventsOnLoc: Append,
    {
        self.delay_with_smoothing_order(value, 1)
    }

    pub fn neutral_delay<D: IntoDelay<N>>(
        self,
        value: D,
    ) -> Solver<
        'a,
        N,
        S,
        Equation,
        Initial,
        Interval,
        EventsOnStep,
        EventsOnStart,
        EventsOnStop,
        <EventsOnLoc as Append>::Output<
            Loc<Propagator<N, <D as IntoDelay<N>>::Output>, Propagation, D::LocationMethod>,
        >,
    >
    where
        EventsOnLoc: Append,
    {
        self.delay_with_smoothing_order(value, 0)
    }

    /// Run solver.
    pub fn run(mut self)
    where
        Equation: StateFnMut<N, Output = [f64; N]>,
        Initial: InitialCondition<N>,
        Interval: std::ops::RangeBounds<f64>,
        EventsOnStep: EventHList<N>,
        EventsOnStart: EventHList<N>,
        EventsOnStop: EventHList<N>,
        EventsOnLoc: LocEventHList<N>,
    {
        use std::ops::Bound::*;
        let t_init = match self.interval.start_bound() {
            Unbounded => 0.,
            Included(&value) | Excluded(&value) => value,
        };
        let t_end = match self.interval.end_bound() {
            Unbounded => f64::INFINITY,
            Included(&value) | Excluded(&value) => value,
        };

        let mut rhs = self.equation;
        let mut state = RKState::new(
            t_init,
            self.initial_disco,
            self.initial,
            self.max_delay,
            &self.rk,
        );
        let mut stepsize = self.stepsize;

        // let mut loc_events = self.loc_events.extend(eq.propagations).extend(eq.events);
        let mut loc_events = self.loc_events;
        // let mut loc_events = self.loc_events.extend(eq.events);

        // MAKE discontinuities FIELD IN STATE, SO PROPAGATION EVENTS CAN ACCESS IT
        // let mut loc_events = self.loc_events.extend(eq.events).extend(PropagatedEach(eq.delays));

        self.start_events.call_each(&mut state);
        self.step_events.call_each(&mut state);

        while state.t() < t_end {
            state.make_step(&mut rhs, stepsize);

            if let Some((t, event)) = loc_events.locate_first(&mut state)
                && t > state.t_prev()
            {
                state.undo_step();
                state.make_step(&mut rhs, t - state.t);
                state.push_current();
                self.step_events.call_each(&mut state);
                event.call(&mut state);
                if state.t_prev() == state.t() {
                    self.step_events.call_each(&mut state);
                    state.disco_seq_mut().push_back((t, 0))
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
