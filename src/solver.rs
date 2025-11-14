use hlist2::{HList, Nil};
use macro_find_and_replace::{replace_token_sequence};

macro_rules! SolverType {
    () => {Solver<'rk, N, S, S2, T, Equation, Initial, Interval, EventsOnStep, EventsOnStart, EventsOnStop, EventsOnLoc> };
    ($arg:ident => $replacement:expr) => {replace_token_sequence!([$arg], [$replacement], Solver<'rk, N, S, S2, T, Equation, Initial, Interval, EventsOnStep, EventsOnStart, EventsOnStop, EventsOnLoc>) };
}

macro_rules! solver_set {
    ($self:expr, $field:ident: $value:expr) => {
        {
            replace_token_sequence!(
                [$field], [$field],
                #[allow(unused_variables)]
                let Solver { equation, initial, initial_disco, interval, max_delay, rk, stepsize, events_on_step, events_on_start, events_on_stop, events_on_loc, } = $self;
            );
            replace_token_sequence!(
                [$field], [$field: $value],
                Solver { equation, initial, initial_disco, interval, max_delay, rk, stepsize, events_on_step, events_on_start, events_on_stop, events_on_loc, }
            )
        }
    };
}

pub struct Solver<
    'rk,
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
    pub rk: &'rk crate::rk::ExplicitRungeKuttaTable<S, S2, T>,
    pub stepsize: T,
    pub max_delay: T,
    pub events_on_step: EventsOnStep,
    pub events_on_start: EventsOnStart,
    pub events_on_stop: EventsOnStop,
    pub events_on_loc: EventsOnLoc,
}

impl<
    'rk,
    const N: usize,
    const S: usize,
    const S2: usize,
    T,
    Equation,
    Initial,
    Interval,
    EventsOnStep: HList,
    EventsOnStart: HList,
    EventsOnStop: HList,
    EventsOnLoc: HList,
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

    pub fn equation<NewEquation>(
        self,
        new_equation: NewEquation,
    ) -> SolverType!(Equation => NewEquation) {
        solver_set!(self, equation: new_equation)
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

    // pub fn rk<const NewS: usize, const NewS2: usize>(
    //     self,
    //     new_rk: &'rk crate::rk::ExplicitRungeKuttaTable<S, S2, T>,
    // ) -> SolverType!(S => NewS, S2 => NewS2) {
    //     solver_set!(self, rk: new_rk)
    // }

    pub fn on_step<NewCallback>(
        self,
        new_callback: NewCallback,
    ) -> SolverType!(EventsOnStep => <EventsOnStep as hlist2::ops::Append>::Output::<NewCallback>)
    where
        EventsOnStep: hlist2::ops::Append,
    {
        solver_set!(self, events_on_step: events_on_step.append(new_callback))
    }
}
