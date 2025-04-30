use std::marker::Tuple;

use crate::{Event, util::tutle::BoolTutle};

use super::{CoordFn, State};

pub trait ToStateFn<S, Arg, Ret> {
    fn to_state_function(self) -> impl for<'b> FnMut<(&'b S,), Output = Ret>;
}

macro_rules! to_state_function_impl {
    ($args_tuple:ty, $self:ident, $state:ident, $body:block) => {
        impl<const N: usize, const S: usize, IF: Fn(f64) -> [f64; N], F, Ret>
            ToStateFn<State<N, S, IF>, $args_tuple, Ret> for F
        where
            F: FnMut<$args_tuple, Output = Ret>,
        {
            fn to_state_function(
                mut $self,
            ) -> impl for<'b> FnMut<(&'b State<N, S, IF>,), Output = Ret> {
                move |$state| $body
            }
        }
    };
}

to_state_function_impl!((), self, _state, { self() });
to_state_function_impl!((f64,), self, state, { self(state.t) });
to_state_function_impl!(([f64; N],), self, state, { self(state.x) });
to_state_function_impl!((f64, [f64; N],), self, state, { self(state.t, state.x) });

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
    > for Event<Callback, Stream, Filter>
where
    Callback: ToStateFn<State<N, S, IF>, CallbackArgs, StreamArg>,
    Stream: FnMut<(StreamArg,), Output = StreamRet>,
    Filter: ToStateFn<State<N, S, IF>, FilterArgs, FilterRet>,
    FilterRet: BoolTutle,
{
    fn to_state_function(
        self,
    ) -> impl for<'b> FnMut<(&'b State<N, S, IF>,), Output = Option<StreamRet>> {
        let Self {
            callback,
            stream,
            filter,
        } = self;

        let mut callback = callback.to_state_function();
        let mut filter = filter.to_state_function();
        let mut stream = stream;
        // stream.call_mut((callback.call_once(args),))

        move |state| {
            if filter(state).all() {
                Some(stream.call_mut((callback.call_mut((state,)),)))
            } else {
                None
            }
        }
    }
}

impl<const N: usize, const S: usize, IF: Fn(f64) -> [f64; N], F, Ret>
    ToStateFn<State<N, S, IF>, (f64, [f64; N], [CoordFn<'_, N, S, IF>; N]), Ret> for F
where
    F: for<'a> FnMut<(f64, [f64; N], [CoordFn<'a, N, S, IF>; N]), Output = Ret>,
{
    fn to_state_function(mut self) -> impl for<'b> FnMut<(&'b State<N, S, IF>,), Output = Ret> {
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
}
