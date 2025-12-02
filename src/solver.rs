use hlist2::{HList, Nil};

use num::Float;
use replace::replace_ident;

use crate::{loc::loc_callback::LocCallback, rk::ExplicitRungeKuttaTable};

macro_rules! SolverType {
    () => {Solver<N, S, S2, T, Equation, Initial, Interval, EventsOnStep, EventsOnStart, EventsOnStop, EventsOnLoc> };
    ($arg:ident => $replacement:expr) => {
        replace_ident!($arg, $replacement, Solver<N, S, S2, T, Equation, Initial, Interval, EventsOnStep, EventsOnStart, EventsOnStop, EventsOnLoc>)
    };
    ($arg1:ident => $replacement1:expr, $arg2:ident => $replacement2:expr) => {
        replace_ident!($arg1, $replacement1,
            replace_ident!($arg2, $replacement2,
                Solver<N, S, S2, T, Equation, Initial, Interval, EventsOnStep, EventsOnStart, EventsOnStop, EventsOnLoc>
            )
        )
    };
}

macro_rules! solver_set {
    ($self:expr, $field:ident: $value:expr) => {
        {
            replace_ident!(
                $field, $field,
                #[allow(unused_variables)]
                let Solver { equation, initial, initial_disco, interval, max_delay, rk, stepsize, events_on_step, events_on_start, events_on_stop, events_on_loc, } = $self;
            );
            replace_ident!(
                $field, $field: $value,
                Solver { equation, initial, initial_disco, interval, max_delay, rk, stepsize, events_on_step, events_on_start, events_on_stop, events_on_loc, }
            )
        }
    };
}

pub struct Solver<
    const N: usize = 0,
    const S: usize = 0,
    const S2: usize = 0,
    T = f64,
    Equation = (),
    Initial = (),
    Interval = (),
    EventsOnStep: HList = Nil,
    EventsOnStart: HList = Nil,
    EventsOnStop: HList = Nil,
    EventsOnLoc: HList = Nil,
> {
    pub equation: Equation,
    pub initial: Initial,
    pub initial_disco: Vec<(T, usize)>,
    pub interval: Interval,
    pub rk: crate::rk::ExplicitRungeKuttaTable<S, S2, T>,
    pub stepsize: T,
    pub max_delay: T,
    pub events_on_step: EventsOnStep,
    pub events_on_start: EventsOnStart,
    pub events_on_stop: EventsOnStop,
    pub events_on_loc: EventsOnLoc,
}

impl<const N: usize, T: Float> Solver<N, 0, 0, T> {
    pub fn new() -> Solver<N, 7, 21, T, (), (), (), Nil, Nil, Nil> {
        Solver {
            equation: (),
            initial: (),
            initial_disco: vec![],
            interval: (),
            max_delay: T::zero(),
            rk: crate::rk::rktp64(),
            stepsize: T::from(0.05).unwrap(),
            events_on_step: Nil,
            events_on_start: Nil,
            events_on_stop: Nil,
            events_on_loc: Nil,
        }
    }
}
impl<
    const N: usize,
    const S: usize,
    const S2: usize,
    T: Float,
    Equation,
    Initial,
    Interval,
    EventsOnStep: HList + hlist2::ops::Append,
    EventsOnStart: HList + hlist2::ops::Append,
    EventsOnStop: HList + hlist2::ops::Append,
    EventsOnLoc: HList + hlist2::ops::Append,
> SolverType!()
{
    /// [Solver::initial_disco] setter. Returns self.
    pub fn initial_disco(self, initial_disco: impl Into<Vec<(T, usize)>>) -> Self {
        Self {
            initial_disco: initial_disco.into(),
            ..self
        }
    }

    /// [Solver::stepsize] setter. Returns self.
    pub fn stepsize(self, stepsize: T) -> Self {
        Self { stepsize, ..self }
    }

    /// [Solver::max_delay] setter. Returns self.
    pub fn max_delay(self, max_delay: T) -> Self {
        Self { max_delay, ..self }
    }

    pub fn equation<E>(self, new_equation: E) -> SolverType!(Equation => E::Output)
    where
        E: crate::state::IntoStateFn<N, T, [T; N]>,
    {
        solver_set!(self, equation: new_equation.into())
    }

    pub fn initial<I>(self, new_initial: I) -> SolverType!(Initial => I) {
        solver_set!(self, initial: new_initial)
    }

    pub fn interval<I>(self, new_interval: I) -> SolverType!(Interval => I) {
        solver_set!(self, interval: new_interval)
    }

    pub fn rk<const S_: usize, const S2_: usize>(
        self,
        new_rk: ExplicitRungeKuttaTable<S_, S2_, T>,
    ) -> SolverType!(S => S_, S2 => S2_) {
        solver_set!(self, rk: new_rk)
    }

    pub fn on_step<C>(self, callback: C) -> SolverType!(EventsOnStep => EventsOnStep::Output::<C>) {
        solver_set!(self, events_on_step: events_on_step.append(callback))
    }

    pub fn on_stop<C>(self, callback: C) -> SolverType!(EventsOnStop => EventsOnStop::Output::<C>) {
        solver_set!(self, events_on_stop: events_on_stop.append(callback))
    }

    pub fn on_start<C>(
        self,
        callback: C,
    ) -> SolverType!(EventsOnStart => EventsOnStart::Output::<C>) {
        solver_set!(self, events_on_start: events_on_start.append(callback))
    }

    pub fn on<L, C, V: Into<LocCallback<L, C>>>(
        self,
        loc_callback: V,
    ) -> SolverType!(EventsOnLoc => EventsOnLoc::Output::<LocCallback<L, C>>) {
        solver_set!(self, events_on_loc: events_on_loc.append(loc_callback.into()))
    }

    #[allow(unused)]
    pub fn run(mut self)
    where
        Equation: crate::state::EvalStateFn<N, T, [T; N]>,
        Interval: crate::interval::IntegrationInterval<T>,
        Initial: crate::initial_condition::InitialCondition<N, T>,
        EventsOnStart: crate::state::EvalMutStateFnHList<N, T, ()>,
        EventsOnStep: crate::state::EvalMutStateFnHList<N, T, ()>,
        EventsOnStop: crate::state::EvalMutStateFnHList<N, T, ()>,
        EventsOnLoc: crate::loc::loc_hlist::HListLocateEarliest<N, T>
            + crate::state::EvalMutStateFnHList<N, T, ()>,
    {
        let t_init = self.interval.start_bound();
        let t_end = self.interval.end_bound();

        let mut rhs = self.equation;
        let mut state = crate::state::State::new(t_init, self.initial, &self.rk);

        let mut stepsize = self.stepsize;

        self.events_on_start.eval_mut(&mut state);
        self.events_on_step.eval_mut(&mut state);

        while state.t_curr < t_end {
            state.make_step(&mut rhs, stepsize);

            if let Some((index, time)) = self.events_on_loc.locate_earliest(&state) {
                state.undo_step();
                state.make_step(&mut rhs, time - state.t_curr);
                state.commit_step();
                state.make_zero_step();
                self.events_on_loc.eval_mut_at_index(&mut state, index);
                state.commit_step();
            } else {
                state.commit_step();
                self.events_on_step.eval_mut(&mut state);
            }
            stepsize = stepsize.min(t_end - state.t_curr);
        }
        self.events_on_stop.eval_mut(&mut state);
    }
}
