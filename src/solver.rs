use hlist2::{HList, Nil};

use nalgebra::RealField;
use replace::replace_ident;

use crate::{
    Loc,
    initial_condition::InitialCondition,
    loc::loc_callback::LocCallback,
    rk::ButcherTableu,
    stepsize::{FixedStepsize, StepStatus, StepsizeController},
    traits::RealVectorSpace,
};

macro_rules! SolverType {
    () => {Solver<T, Y, S, I, Equation, Initial, Interval, Stepsize, EventsOnStep, EventsOnStart, EventsOnStop, EventsOnLoc> };
    ($arg:ident => $replacement:ty) => {
        replace_ident!($arg, $replacement, Solver<T, Y, S, I, Equation, Initial, Interval, Stepsize, EventsOnStep, EventsOnStart, EventsOnStop, EventsOnLoc>)
    };
    ($arg1:ident => $replacement1:ty, $arg2:ident => $replacement2:ty) => {
        replace_ident!($arg1, $replacement1,
            replace_ident!($arg2, $replacement2,
                Solver<T, Y, S, I, Equation, Initial, Interval, Stepsize, EventsOnStep, EventsOnStart, EventsOnStop, EventsOnLoc>
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
                let Solver { equation, initial, initial_disco, interval, max_delay, rk, stepsize, events_on_step, events_on_start, events_on_stop, events_on_loc, _phantom_y } = $self;
            );
            replace_ident!(
                $field, $field: $value,
                Solver { equation, initial, initial_disco, interval, max_delay, rk, stepsize, events_on_step, events_on_start, events_on_stop, events_on_loc, _phantom_y }
            )
        }
    };
}

pub struct Solver<
    T = f64,
    Y = f64,
    const S: usize = 0,
    const I: usize = 0,
    Equation = (),
    Initial = (),
    Interval = (),
    Stepsize = (),
    EventsOnStep: HList = Nil,
    EventsOnStart: HList = Nil,
    EventsOnStop: HList = Nil,
    EventsOnLoc: HList = Nil,
> {
    pub equation: Equation,
    pub initial: Initial,
    pub initial_disco: Vec<(T, usize)>,
    pub interval: Interval,
    pub rk: crate::rk::ButcherTableu<T, S, I>,
    pub stepsize: Stepsize,
    pub max_delay: T,
    pub events_on_step: EventsOnStep,
    pub events_on_start: EventsOnStart,
    pub events_on_stop: EventsOnStop,
    pub events_on_loc: EventsOnLoc,
    pub _phantom_y: std::marker::PhantomData<Y>,
}

impl Solver {
    pub fn new<T: RealField + Copy, Y: RealVectorSpace<T>>()
    -> Solver<T, Y, 7, 5, (), (), (), FixedStepsize<T>, Nil, Nil, Nil> {
        Solver {
            equation: (),
            initial: (),
            initial_disco: vec![],
            interval: (),
            max_delay: T::zero(),
            rk: crate::rk::ButcherTableu::rktp64(),
            stepsize: FixedStepsize(T::from_f64(0.05).unwrap()),
            events_on_step: Nil,
            events_on_start: Nil,
            events_on_stop: Nil,
            events_on_loc: Nil,
            _phantom_y: Default::default(),
        }
    }
}
impl<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    Equation,
    Initial,
    Interval,
    Stepsize,
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
    pub fn stepsize<NewStepsize>(
        self,
        new_stepsize: NewStepsize,
    ) -> SolverType!(Stepsize => NewStepsize) {
        solver_set!(self, stepsize: new_stepsize)
    }

    /// [Solver::max_delay] setter. Returns self.
    pub fn max_delay(self, max_delay: T) -> Self {
        Self { max_delay, ..self }
    }

    #[allow(unused_parens)]
    pub fn equation<F: FnMut(&crate::StateRef<T, Y, S, I, Initial>) -> Y>(
        self,
        new_equation: F, // crate::state::StateFn<T, Y, Y, F>,
    ) -> SolverType!(Equation => (crate::state::StateFn<T, Y, Y, F>)) {
        solver_set!(self, equation: crate::StateFn::new(new_equation))
    }

    pub fn initial<NewInitial>(
        self,
        new_initial: NewInitial,
    ) -> SolverType!(Initial => NewInitial) {
        solver_set!(self, initial: new_initial)
    }

    pub fn interval<NewInterval>(
        self,
        new_interval: NewInterval,
    ) -> SolverType!(Interval => NewInterval) {
        solver_set!(self, interval: new_interval)
    }

    pub fn rk<const S_: usize, const I_: usize>(
        self,
        new_rk: ButcherTableu<T, S_, I_>,
    ) -> SolverType!(S => S_, I => I_) {
        solver_set!(self, rk: new_rk)
    }

    #[allow(unused_parens)]
    pub fn on_step<C: FnMut(&crate::StateRef<T, Y, S, I, Initial>)>(
        self,
        callback: C,
    ) -> SolverType!(EventsOnStep => EventsOnStep::Output::<(crate::state::StateFn<T, Y, (), C>)>)
    {
        solver_set!(self, events_on_step: events_on_step.append(crate::StateFn::new(callback)))
    }

    #[allow(unused_parens)]
    pub fn on_stop<C: FnMut(&crate::StateRef<T, Y, S, I, Initial>)>(
        self,
        callback: C,
    ) -> SolverType!(EventsOnStop => EventsOnStop::Output::<(crate::state::StateFn<T, Y, (), C>)>)
    {
        solver_set!(self, events_on_stop: events_on_stop.append(crate::StateFn::new(callback)))
    }

    #[allow(unused_parens)]
    pub fn on_start<C: FnMut(&crate::StateRef<T, Y, S, I, Initial>)>(
        self,
        callback: C,
    ) -> SolverType!(EventsOnStart => EventsOnStart::Output::<(crate::state::StateFn<T, Y, (), C>)>)
    {
        solver_set!(self, events_on_start: events_on_start.append(crate::StateFn::new(callback)))
    }

    #[allow(unused_parens)]
    pub fn on_loc<
        LocF: FnMut(&crate::StateRef<T, Y, S, I, Initial>) -> Output,
        Output,
        LocLocate,
        LocDetect,
        CallbackF: FnMut(&crate::StateRef<T, Y, S, I, Initial>),
    >(
        self,
        loc: Loc<T, Y, S, I, Initial, LocF, LocLocate, LocDetect>,
        callback: CallbackF,
    ) -> SolverType!(EventsOnLoc => (EventsOnLoc::Output::<LocCallback<Loc<T, Y, S, I, Initial, LocF, LocLocate, LocDetect>, (crate::state::StateFn<T, Y, (), CallbackF>)>>))
    {
        solver_set!(self, events_on_loc: events_on_loc.append(LocCallback(loc, crate::StateFn::new(callback))))
    }

    #[allow(unused_parens)]
    pub fn on_loc_mut<L, C: FnMut(&mut crate::StateRefMut<T, Y, S, I, Initial>)>(
        self,
        loc: L,
        callback: C,
    ) -> SolverType!(EventsOnLoc => (EventsOnLoc::Output::<LocCallback<L, (crate::state::StateFn<T, Y, (), C, true>)>>))
    {
        solver_set!(self, events_on_loc: events_on_loc.append(LocCallback(loc, crate::StateFn::new_mut(callback))))
    }

    #[allow(unused_parens)]
    pub fn on_zero<
        LocF: FnMut(&crate::StateRef<T, Y, S, I, Initial>) -> T,
        CallbackF: FnMut(&crate::StateRef<T, Y, S, I, Initial>),
    >(
        self,
        loc: LocF,
        callback: CallbackF,
    ) -> SolverType!(EventsOnLoc => (EventsOnLoc::Output::<LocCallback<Loc<T, Y, S, I, Initial, crate::state::StateFn<T, Y, T, LocF>, crate::loc::detect::Zero, crate::loc::locate::Bisection>, (crate::state::StateFn<T, Y, (), CallbackF>)>>))
    where
        Initial: InitialCondition<T, Y>,
    {
        solver_set!(self, events_on_loc: events_on_loc.append(LocCallback(Loc::zero(loc), crate::StateFn::new(callback))))
    }

    #[allow(unused_parens)]
    pub fn on_zero_mut<
        LocF: FnMut(&crate::StateRef<T, Y, S, I, Initial>) -> T,
        CallbackF: FnMut(&mut crate::StateRefMut<T, Y, S, I, Initial>),
    >(
        self,
        loc: LocF,
        callback: CallbackF,
    ) -> SolverType!(EventsOnLoc => (EventsOnLoc::Output::<LocCallback<Loc<T, Y, S, I, Initial, crate::state::StateFn<T, Y, T, LocF>, crate::loc::detect::Zero, crate::loc::locate::Bisection>, (crate::state::StateFn<T, Y, (), CallbackF, true>)>>))
    where
        Initial: InitialCondition<T, Y>,
    {
        solver_set!(self, events_on_loc: events_on_loc.append(LocCallback(Loc::zero(loc), crate::StateFn::new_mut(callback))))
    }

    pub fn run(mut self)
    where
        Equation: crate::state::EvalStateFn<T, Y, S, I, Initial, Y>,
        Interval: crate::interval::IntegrationInterval<T>,
        Initial: crate::initial_condition::InitialCondition<T, Y>,
        Stepsize: StepsizeController<T, Y>,
        EventsOnStart: crate::state::EvalMutStateFnHList<T, Y, S, I, Initial, ()>,
        EventsOnStep: crate::state::EvalMutStateFnHList<T, Y, S, I, Initial, ()>,
        EventsOnStop: crate::state::EvalMutStateFnHList<T, Y, S, I, Initial, ()>,
        EventsOnLoc: crate::loc::loc_hlist::HListLocateEarliest<T, Y, S, I, Initial>
            + crate::state::EvalMutStateFnHList<T, Y, S, I, Initial, ()>,
    {
        let t_init = self.interval.start_bound();
        let t_end = self.interval.end_bound();

        let mut rhs = self.equation;
        let mut state = crate::state::State::new(t_init, self.max_delay, self.initial, self.rk);

        let mut stepsize = self.stepsize;

        self.events_on_start.eval_mut(&mut state);
        self.events_on_step.eval_mut(&mut state);

        while state.t_curr < t_end {
            state.make_step(&mut rhs, stepsize.get().min(t_end - state.t_curr));

            while matches!(stepsize.update(&state.e_curr), StepStatus::Rejected) {
                state.undo_step();
                state.make_step(&mut rhs, stepsize.get().min(t_end - state.t_curr));
            }

            if let Some((index, time)) = self.events_on_loc.locate_earliest(&state) {
                state.undo_step();
                state.make_step(&mut rhs, time - state.t_curr);
                state.commit_step();
                state.make_zero_step();
                self.events_on_loc.eval_mut_at_index(&mut state, index);
            }
            state.commit_step();
            self.events_on_step.eval_mut(&mut state);
        }
        self.events_on_stop.eval_mut(&mut state);
    }
}
