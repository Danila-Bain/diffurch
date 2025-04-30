use std::marker::Tuple;

use crate::{util::tuple_tower::BoolTupleTower, Event};

use super::{CoordinateFunction, State};

pub trait ToStateFunction<S, Arg, Ret> {
    fn to_state_function(self) -> impl for<'b> FnMut<(&'b S,), Output = Ret>;
}

macro_rules! to_state_function_impl {
    ($args_tuple:ty, $self:ident, $state:ident, $body:block) => {
        impl<const N: usize, const S: usize, InitialFunction: Fn(f64) -> [f64; N], F, Ret>
            ToStateFunction<State<N, S, InitialFunction>, $args_tuple, Ret> for F
        where
            F: FnMut<$args_tuple, Output = Ret>,
        {
            fn to_state_function(
                mut $self,
            ) -> impl for<'b> FnMut<(&'b State<N, S, InitialFunction>,), Output = Ret> {
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
    InitialFunction: Fn(f64) -> [f64; N],
    Callback,
    Stream,
    Filter,
    CallbackArgs,
    StreamArg,
    StreamRet,
    FilterArgs,
    FilterRet,
> ToStateFunction<State<N, S, InitialFunction>, (CallbackArgs, StreamArg, StreamRet, FilterArgs, FilterRet), Option<StreamRet>>
    for Event<Callback, Stream, Filter>
where
    Callback: ToStateFunction<State<N, S, InitialFunction>, CallbackArgs, StreamArg>,
    Stream: FnMut<(StreamArg,), Output = StreamRet>,
    Filter: ToStateFunction<State<N, S, InitialFunction>, FilterArgs, FilterRet>,
    FilterRet: BoolTupleTower,
{
    fn to_state_function(
        self,
    ) -> impl for<'b> FnMut<(&'b State<N, S, InitialFunction>,), Output = Option<StreamRet>> {
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

impl<const N: usize, const S: usize, InitialFunction: Fn(f64) -> [f64; N], F, Ret>
    ToStateFunction<
        State<N, S, InitialFunction>,
        (
            f64,
            [f64; N],
            [CoordinateFunction<'_, N, S, InitialFunction>; N],
        ),
        Ret,
    > for F
where
    F: for<'a> FnMut<
            (
                f64,
                [f64; N],
                [CoordinateFunction<'a, N, S, InitialFunction>; N],
            ),
            Output = Ret,
        >,
{
    fn to_state_function(
        mut self,
    ) -> impl for<'b> FnMut<(&'b State<N, S, InitialFunction>,), Output = Ret> {
        move |state| {
            self(
                state.t,
                state.x,
                std::array::from_fn(|i| CoordinateFunction {
                    state_ref: state,
                    coordinate: i,
                }),
            )
        }
    }
}
