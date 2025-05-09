// use crate::{
//     Event,
//     util::tutle::{BoolTutle, LazyBoolTutle, TutleLevel},
// };
//
// use super::{CoordFn, State, ToBoolStateTutle, ToStateTutle};
//
//

use super::State;

pub trait ToStateFn<const N: usize, const S: usize, Args, Ret> {
    fn to_state_function(self) -> Box<dyn Fn(&State<N, S>) -> Ret>;
    fn to_state_eval_function(self) -> Box<dyn Fn(&State<N, S>, f64) -> Ret>;
}

macro_rules! to_state_function_impl {
    ($(lifetime: $lifetime:tt,)? args: $args_tuple:ty, t: $t:ident, state: $state:ident, body: $body:expr, body_eval: $body_eval:expr,) => {
        impl<const N: usize, const S: usize, Ret, F> ToStateFn<N, S, $args_tuple, Ret> for F
        where
            F: 'static + $(for<$lifetime>)? Fn<$args_tuple, Output = Ret>,
        {
            fn to_state_function(self) -> Box<dyn Fn(&State<N, S>) -> Ret> {
                Box::new(move |$state| self.call($body))
            }

            fn to_state_eval_function(self) -> Box<dyn Fn(&State<N, S>, f64) -> Ret> {
                Box::new(move |$state, $t| self.call($body_eval))
            }
        }
    };
}

// to_state_function_impl!( lifetime: 'a, args: (&'a State<N,S>,), t: _t, state: _state, body: (_state,), body_eval: (_state, _t,),);
to_state_function_impl!( args: (), t: _t, state: _state, body: (), body_eval: (),);
to_state_function_impl!( args: (f64,), t: _t, state: _state, body: (_state.t,), body_eval: (_t,),);
to_state_function_impl!( args: ([f64; N],), t: t, state: state, body: (state.x,), body_eval: (state.eval_all(t),),);
to_state_function_impl!( args: (f64, [f64; N]), t: t, state: state, body: (state.t, state.x), body_eval: (t, state.eval_all(t)),);

// impl<const N: usize, const S: usize, IF: Fn(f64) -> [f64; N], F, Ret>
//     ToStateFn<State<N, S, IF>, (f64, [f64; N], [CoordFn<'_, N, S, IF>; N]), Ret> for F
// where
//     F: for<'a> FnMut<(f64, [f64; N], [CoordFn<'a, N, S, IF>; N]), Output = Ret>,
// {
//     type StateFn = impl for<'b> FnMut<(&'b State<N, S, IF>,), Output = Ret>;
//     type StateEvalFn = impl for<'b> FnMut<(&'b State<N, S, IF>, f64), Output = Ret>;
//     fn to_state_function(mut self) -> Self::StateFn {
//         move |state| {
//             self(
//                 state.t,
//                 state.x,
//                 std::array::from_fn(|i| CoordFn {
//                     state_ref: state,
//                     coordinate: i,
//                 }),
//             )
//         }
//     }
//
//     fn to_state_eval_function(mut self) -> Self::StateEvalFn {
//         move |state, t| {
//             self(
//                 t,
//                 state.eval_all(t),
//                 std::array::from_fn(|i| CoordFn {
//                     state_ref: state,
//                     coordinate: i,
//                 }),
//             )
//         }
//     }
// }
//
// impl<
//     const N: usize,
//     const S: usize,
//     IF: Fn(f64) -> [f64; N],
//     Callback,
//     Stream,
//     Filter,
//     CallbackArgs,
//     StreamArg,
//     FilterArgs,
//     FilterRet,
// > ToStateFn<State<N, S, IF>, (CallbackArgs, StreamArg, FilterArgs, FilterRet), ()>
//     for Event<Callback, Stream, Filter, ()>
// where
//     Callback: ToStateFn<State<N, S, IF>, CallbackArgs, StreamArg>,
//     Stream: FnMut<(StreamArg,)>,
//     Filter: TutleLevel,
//     Filter: ToBoolStateTutle<State<N, S, IF>, FilterArgs, FilterRet, Filter::Level>,
//     FilterRet: BoolTutle,
//     // Filter::StateTutle: TutleLevel, // works if we add TutleLevel to this associated type in
//     // ToStateTutle impl
//     // Filter::StateTutle: for <'a> FnMut<(&'a State<N, S, IF>,)>, // ok
//     // Filter::StateTutle : for<'a> LazyBoolTutle<(&'a State<N,S,IF>,)>,
//     // Filter::StateEvalTutle : for <'a> LazyBoolTutle<(&'a State<N,S,IF>, f64)>,
// {
//     type StateFn = impl for<'b> FnMut<(&'b State<N, S, IF>,), Output = ()>;
//     type StateEvalFn = impl for<'b> FnMut<(&'b State<N, S, IF>, f64), Output = ()>;
//
//     fn to_state_function(self) -> Self::StateFn {
//         let Self {
//             callback,
//             stream,
//             filter,
//             subdivision: _,
//         } = self;
//
//         let mut callback = callback.to_state_function();
//         let mut filter = filter.to_state_tutle();
//         let mut stream = stream;
//
//         move |state| {
//             if filter.lazy_all((state,)) {
//                 // if filter(state).all() {
//                 stream.call_mut((callback.call_mut((state,)),));
//             }
//         }
//     }
//
//     fn to_state_eval_function(self) -> Self::StateEvalFn {
//         let Self {
//             callback,
//             stream,
//             filter,
//             subdivision: _,
//         } = self;
//
//         let mut callback = callback.to_state_eval_function();
//         let mut filter = filter.to_state_eval_tutle();
//         let mut stream = stream;
//
//         move |state, t| {
//             if filter.lazy_all((state, t)) {
//                 // if filter(state, t).all() {
//                 stream.call_mut((callback.call_mut((state, t)),));
//             }
//         }
//     }
// }
//
// impl<
//     const N: usize,
//     const S: usize,
//     IF: Fn(f64) -> [f64; N],
//     Callback,
//     Stream,
//     Filter,
//     CallbackArgs,
//     StreamArg,
//     FilterArgs,
//     FilterRet,
// > ToStateFn<State<N, S, IF>, (CallbackArgs, StreamArg, FilterArgs, FilterRet), ()>
//     for Event<Callback, Stream, Filter, usize>
// where
//     Callback: ToStateFn<State<N, S, IF>, CallbackArgs, StreamArg>,
//     Stream: FnMut<(StreamArg,)>,
//     Filter: TutleLevel,
//     Filter: ToStateTutle<State<N, S, IF>, FilterArgs, FilterRet, Filter::Level>,
//     FilterRet: BoolTutle,
// {
//     type StateFn = impl for<'b> FnMut<(&'b State<N, S, IF>,), Output = ()>;
//     type StateEvalFn = impl for<'b> FnMut<(&'b State<N, S, IF>, f64), Output = ()>;
//
//     fn to_state_function(self) -> Self::StateFn {
//         let n = self.subdivision;
//
//         let mut self_eval = self.to_state_eval_function();
//
//         move |state| {
//             let step = state.t - state.t_prev;
//
//             for i in 1..=n {
//                 self_eval.call_mut((state, state.t_prev + (i as f64 / n as f64 * step)));
//             }
//         }
//     }
//
//     fn to_state_eval_function(self) -> Self::StateEvalFn {
//         let Self {
//             callback,
//             stream,
//             filter,
//             subdivision: _,
//         } = self;
//
//         let mut callback = callback.to_state_eval_function();
//         let mut filter = filter.to_state_tutle();
//         let mut stream = stream;
//
//         move |state, t| {
//             if filter(state).all() {
//                 stream.call_mut((callback.call_mut((state, t)),));
//             }
//         }
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     fn new_state() -> State<1, 1, impl Fn(f64) -> [f64; 1]> {
//         let ic = |_: f64| [0.42];
//         return State::new(0.69, ic, &crate::rk::EULER);
//     }
//
//     #[test]
//     fn constant_fn() {
//         let state = new_state();
//         let f = || 123.;
//         let mut f = f.to_state_function();
//         assert_eq!(f(&state), 123.);
//     }
//
//     #[test]
//     fn time_fn() {
//         let state = new_state();
//         let f = |t: f64| (t, -t);
//         let mut f = f.to_state_function();
//         assert_eq!(f(&state), (state.t, -state.t));
//     }
//
//     #[test]
//     fn ode_fn() {
//         let state = new_state();
//         let f = |[x]: [f64; 1]| (x, -x);
//         let mut f = f.to_state_function();
//         assert_eq!(f(&state), (state.x[0], -state.x[0]));
//     }
//
//     #[test]
//     fn ode2_fn() {
//         let state = new_state();
//         let f = |t: f64, [x]: [f64; 1]| (t, x);
//         let mut f = f.to_state_function();
//         assert_eq!(f(&state), (state.t, state.x[0]));
//     }
//
//     #[test]
//     fn const_event() {
//         let state = new_state();
//         let mut out = 0.;
//         {
//             let f = Event::new(|| 123.).to_var(&mut out);
//
//             println!("{:?}", (f.callback)());
//             let mut f = f.to_state_function();
//             f(&state);
//         }
//         assert_eq!(out, 123.);
//     }
// }
