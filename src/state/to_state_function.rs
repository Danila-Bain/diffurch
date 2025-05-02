use crate::{
    util::tutle::{BoolTutle, TutleLevel}, Event
};

use super::{CoordFn, State, ToStateTutle};

pub trait ToStateFn<S, Arg, Ret> {
    type StateFn: for<'b> FnMut<(&'b S,), Output = Ret>;
    type StateEvalFn: for<'b> FnMut<(&'b S, f64), Output = Ret>;

    fn to_state_function(self) -> Self::StateFn;
    fn to_state_eval_function(self) -> Self::StateEvalFn;
}

macro_rules! to_state_function_impl {
    (args: $args_tuple:ty, self: $self:ident, t: $t:ident, state: $state:ident, body: $body:block, body_eval: $body_eval:block) => {
        impl<const N: usize, const S: usize, IF: Fn(f64) -> [f64; N], F, Ret>
            ToStateFn<State<N, S, IF>, $args_tuple, Ret> for F
        where F: FnMut<$args_tuple, Output = Ret>,
        {
            type StateFn = impl for<'b> FnMut<(&'b State<N,S,IF>,), Output = Ret>;
            type StateEvalFn = impl for<'b> FnMut<(&'b State<N,S,IF>, f64), Output = Ret>;
            fn to_state_function(mut $self) -> Self::StateFn { move |$state| $body }
            fn to_state_eval_function( mut $self) -> Self::StateEvalFn { move |$state, $t| $body_eval }
        }
    };
}

to_state_function_impl! {
    args: (),
    self: self,
    t: _t,
    state: _state,
    body: { self() },
    body_eval: { self() }
}

to_state_function_impl! {
    args: (f64,),
    self: self,
    t: t,
    state: _state,
    body: { self(_state.t) },
    body_eval: { self(t) }
}

to_state_function_impl! {
    args: ([f64; N],),
    self: self,
    t: t,
    state: state,
    body: { self(state.x) },
    body_eval: { self(state.eval_all(t)) }
}

to_state_function_impl! {
    args: (f64, [f64; N]),
    self: self,
    t: t,
    state: state,
    body: { self(state.t, state.x) },
    body_eval: { self(t, state.eval_all(t)) }
}

impl<const N: usize, const S: usize, IF: Fn(f64) -> [f64; N], F, Ret>
    ToStateFn<State<N, S, IF>, (f64, [f64; N], [CoordFn<'_, N, S, IF>; N]), Ret> for F
where
    F: for<'a> FnMut<(f64, [f64; N], [CoordFn<'a, N, S, IF>; N]), Output = Ret>,
{

    type StateFn = impl for<'b> FnMut<(&'b State<N,S,IF>,), Output = Ret>;
    type StateEvalFn = impl for<'b> FnMut<(&'b State<N,S,IF>, f64), Output = Ret>;
    fn to_state_function(mut self) -> Self::StateFn {
        move |state| {
            self(
                state.t,
                state.x,
                std::array::from_fn(|i| CoordFn {
                    state_ref: state,
                    coordinate: i,
                }),
            )
        }
    }

    fn to_state_eval_function(mut self) -> Self::StateEvalFn {
        move |state, t| {
            self(
                t,
                state.eval_all(t),
                std::array::from_fn(|i| CoordFn {
                    state_ref: state,
                    coordinate: i,
                }),
            )
        }
    }
}

impl<
    const N: usize,
    const S: usize,
    IF: Fn(f64) -> [f64; N],
    Callback,
    Stream,
    Filter,
    CallbackArgs,
    StreamArg,
    StreamRet,
    FilterArgs,
    FilterRet,
>
    ToStateFn<
        State<N, S, IF>,
        (CallbackArgs, StreamArg, StreamRet, FilterArgs, FilterRet),
        Option<StreamRet>,
    > for Event<Callback, Stream, Filter, ()>
where
    Callback: ToStateFn<State<N, S, IF>, CallbackArgs, StreamArg>,
    Stream: FnMut<(StreamArg,), Output = StreamRet>,
    Filter: TutleLevel,
    Filter: ToStateTutle<State<N, S, IF>, FilterArgs, FilterRet, Filter::Level>,
    FilterRet: BoolTutle,
    // Filter::StateTutle : for<'a> LazyBoolTutle<(&'a State<N,S,IF>,)>,
    // Filter::StateEvalTutle : for <'a> LazyBoolTutle<(&'a State<N,S,IF>, f64)>,
{

    type StateFn = impl for<'b> FnMut<(&'b State<N, S, IF>,), Output = Option<StreamRet>> ;
    type StateEvalFn = impl for<'b> FnMut<(&'b State<N, S, IF>, f64), Output = Option<StreamRet>> ;

    fn to_state_function(
        self,
    ) -> Self::StateFn {
        let Self {
            callback,
            stream,
            filter,
            subdivision: _,
        } = self;

        let mut callback = callback.to_state_function();
        let mut filter = filter.to_state_tutle();
        let mut stream = stream;

        move |state| {
            // if filter.lazy_all((state,)) {
            if filter(state).all() {
                Some(stream.call_mut((callback.call_mut((state,)),)))
            } else {
                None
            }
        }
    }

    fn to_state_eval_function(self) -> Self::StateEvalFn {
        let Self {
            callback,
            stream,
            filter,
            subdivision: _,
        } = self;

        let mut callback = callback.to_state_eval_function();
        let mut filter = filter.to_state_eval_tutle();
        let mut stream = stream;

        move |state, t| {
            // if filter.lazy_all((state, t)) {
            if filter(state, t).all() {
                Some(stream.call_mut((callback.call_mut((state, t)),)))
            } else {
                None
            }
        }
    }
}

// Event<{closure@examples/harmonic_oscillator.rs:42:25: 42:37}, Tutle<(impl FnMut<((f64, f64, f64, f64),)>, Tutle)>>
// Event<{closure@examples/harmonic_oscillator.rs:42:25: 42:37}, Tutle<(impl FnMut<((f64, f64, f64, f64),)>, Tutle)>>: ToStateFn<State<_, 26, _>, _, _>
// Tutle<(Event<{closure@examples/harmonic_oscillator.rs:42:25: 42:37}, Tutle<(impl FnMut<((f64, f64, f64, f64),)>, Tutle)>>, Tutle)>
// Tutle<(Event<{closure@examples/harmonic_oscillator.rs:49:29: 49:31}, Tutle<(impl FnMut<(&str,)>, Tutle)>>, Tutle<(Event<{closure@examples/harmonic_oscillator.rs:42:25: 42:37}, Tutle<(impl FnMut<((f64, f64, f64, f64),)>, Tutle)>>, Tutle)>)>



impl<
    const N: usize,
    const S: usize,
    IF: Fn(f64) -> [f64; N],
    Callback,
    Stream,
    Filter,
    CallbackArgs,
    StreamArg,
    StreamRet,
    FilterArgs,
    FilterRet,
>
    ToStateFn<
        State<N, S, IF>,
        (CallbackArgs, StreamArg, StreamRet, FilterArgs, FilterRet),
        Option<StreamRet>,
    > for Event<Callback, Stream, Filter, usize>
where
    Callback: ToStateFn<State<N, S, IF>, CallbackArgs, StreamArg>,
    Stream: FnMut<(StreamArg,), Output = StreamRet>,
    Filter: TutleLevel,
    Filter: ToStateTutle<State<N, S, IF>, FilterArgs, FilterRet, Filter::Level>,
    FilterRet: BoolTutle,
{


    type StateFn = impl for<'b> FnMut<(&'b State<N, S, IF>,), Output = Option<StreamRet>> ;
    type StateEvalFn = impl for<'b> FnMut<(&'b State<N, S, IF>, f64), Output = Option<StreamRet>> ;

    fn to_state_function(
        self,
    ) -> Self::StateFn {
        let n = self.subdivision;

        let mut self_eval = self.to_state_eval_function();

        move  |state| {
            let step = state.t - state.t_prev;

            for i in 1..n {
                self_eval.call_mut((state, state.t_prev + (i as f64 / n as f64 * step)));
            }
            self_eval.call_mut((state, state.t))
        }
    }

    fn to_state_eval_function(self) -> Self::StateEvalFn {
        let Self {
            callback,
            stream,
            filter,
            subdivision: _,
        } = self;

        let mut callback = callback.to_state_eval_function();
        let mut filter = filter.to_state_tutle();
        let mut stream = stream;

        move |state, t| {
            if filter(state).all() {
                Some(stream.call_mut((callback.call_mut((state, t)),)))
            } else {
                None
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::util::tutle::Tutle;

    use super::*;

    fn new_state() -> State<1, 1, impl Fn(f64) -> [f64;1]> {
        let ic = |_: f64| [0.42];
        return State::new(0.69, ic, &crate::rk::EULER);
    }

    #[test]
    fn constant_fn() {
        let state = new_state();
        let f = || 123.;
        let mut f = f.to_state_function();
        assert_eq!(f(&state), 123.);
    }


    #[test]
    fn time_fn() {
        let state = new_state();
        let f = |t: f64| (t, -t);
        let mut f = f.to_state_function();
        assert_eq!(f(&state), (state.t, -state.t));
    }

    #[test]
    fn ode_fn() {
        let state = new_state();
        let f = |[x]: [f64; 1]| (x, -x);
        let mut f = f.to_state_function();
        assert_eq!(f(&state), (state.x[0], -state.x[0]));
    }

    #[test]
    fn ode2_fn() {
        let state = new_state();
        let f = |t: f64, [x]: [f64; 1]| (t, x);
        let mut f = f.to_state_function();
        assert_eq!(f(&state), (state.t, state.x[0]));
    }

    #[test]
    fn const_event() {
        let state = new_state();
        let f = Event::new(|| 123.);

        println!("{:?}", (f.callback)());
        let mut f = f.to_state_function();
        assert_eq!(f(&state), Some(Tutle(())));
    }
}
