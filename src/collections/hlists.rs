//! This module implements traits for heterogeneous lists, including
//! [EventHList], [LocHList], [LocEventHList], [FilterHList], [StreamHList].
//!
//! The purpose of those traits is to implement specialized iterating behaviour over heterogeneous
//! lists.
//!
//! For [EventHList] and [StreamHList], we just want to call each object in the list.
//!
//! For [FilterHList], we want a short-circuiting method `all`.
//!
//! For [LocHList] we want to get the minimal `Some` value among callbacks, which is first located
//! event, if any event is located.
//!
//! For [LocEventHList] we want to get first located event, and a pointer to its callback, if any
//! event is located at the step.
use hlist2::{
    Cons, HList, Nil,
    ops::{Extend, ToRef},
};

use crate::{EventCall, EventCallConcrete, Locate, State, StateFnMut};

/// The trait for [HList]'s of [EventCall]'s
pub trait EventHList<const N: usize>: ToRef {
    /// call [EventCall::call] for each element of the [HList].
    fn call_each(&mut self, state: &mut impl State<N>);
}
impl<const N: usize> EventHList<N> for Nil {
    fn call_each(&mut self, _: &mut impl State<N>) {}
}
impl<const N: usize, H, T> EventHList<N> for Cons<H, T>
where
    H: EventCall<N>,
    T: EventHList<N>,
{
    fn call_each(&mut self, state: &mut impl State<N>) {
        let Cons(head, tail) = self;
        head.call(state);
        tail.call_each(state);
    }
}

/// The trait for [HList]'s of ([Loc], [EventCall]) pairs.
pub trait LocHList<const N: usize>: HList + ToRef {
    /// If any even is located, return location and callback handler for the earlyiest one.
    fn locate_first<S: State<N>>(&mut self, state: &mut S) -> Option<f64>;
}
impl<const N: usize> LocHList<N> for Nil {
    fn locate_first<S: State<N>>(&mut self, _: &mut S) -> Option<f64> {
        None
    }
}
impl<const N: usize, L, T> LocHList<N> for Cons<L, T>
where
    L: Locate<N>,
    T: LocHList<N>,
{
    fn locate_first<S: State<N>>(&mut self, state: &mut S) -> Option<f64> {
        let Cons(loc, tail) = self;

        let head = loc.locate(state);
        let tail = tail.locate_first(state);

        match head {
            None => tail,
            Some(head) => match tail {
                None => Some(head),
                Some(tail) => {
                    if head < tail {
                        Some(head)
                    } else {
                        Some(tail)
                    }
                }
            },
        }
    }
}

impl<const N: usize, L, E> Locate<N> for (L, E)
where
    L: Locate<N>,
    E: EventCall<N>,
{
    fn locate(&mut self, state: &impl State<N>) -> Option<f64> {
        self.0.locate(state)
    }
}
impl<const N: usize, L, E> EventCall<N> for (L, E)
where
    L: Locate<N>,
    E: EventCall<N>,
{
    fn call(&mut self, state: &mut impl State<N>) {
        self.1.call(state)
    }
}

/// The trait for [HList]'s of ([Loc], [EventCall]) pairs.
pub trait LocEventHList<const N: usize>: ToRef + Extend {
    /// If any even is located, return location and callback handler for the earlyiest one.
    fn locate_first<S: State<N>>(
        &mut self,
        state: &mut S,
    ) -> Option<(f64, &mut dyn EventCallConcrete<N, S>)>;
}
impl<const N: usize> LocEventHList<N> for Nil {
    fn locate_first<S: State<N>>(
        &mut self,
        _: &mut S,
    ) -> Option<(f64, &mut dyn EventCallConcrete<N, S>)> {
        None
    }
}
impl<const N: usize, LE, T> LocEventHList<N> for Cons<LE, T>
where
    LE: Locate<N> + EventCall<N>,
    T: LocEventHList<N>,
{
    fn locate_first<S: State<N>>(
        &mut self,
        state: &mut S,
    ) -> Option<(f64, &mut dyn EventCallConcrete<N, S>)> {
        let Cons(locevent, tail) = self;

        let head = locevent.locate(state).and_then(|t| {
            let event: &mut dyn EventCallConcrete<N, S> = locevent;
            Some((t, event))
        });
        let tail = tail.locate_first(state);

        match head {
            None => tail,
            Some(head) => match tail {
                None => Some(head),
                Some(tail) => {
                    if head.0 < tail.0 {
                        Some(head)
                    } else {
                        Some(tail)
                    }
                }
            },
        }
    }
}

/// [HList] of Streams in [Event]
pub trait StreamHList<Arg>: ToRef {
    /// Call each element of the list with `arg`
    fn call_each(&mut self, arg: Arg);
}
impl<Arg> StreamHList<Arg> for Nil {
    fn call_each(&mut self, _: Arg) {}
}
impl<Arg, H, T> StreamHList<Arg> for Cons<H, T>
where
    Arg: Copy,
    H: FnMut(Arg),
    T: StreamHList<Arg>,
{
    fn call_each(&mut self, arg: Arg) {
        let Cons(head, tail) = self;
        head(arg);
        tail.call_each(arg);
    }
}

/// [Hlist] of filters in [Event]
pub trait FilterHList<const N: usize>: ToRef {
    /// Short-circit evaluation of elements of the list
    fn all(&mut self, state: &impl State<N>) -> bool;
    /// Short-circit evaluation of elements of the list at the point `t`  
    fn all_at(&mut self, state: &impl State<N>, t: f64) -> bool;
}
impl<const N: usize> FilterHList<N> for Nil {
    fn all(&mut self, _: &impl State<N>) -> bool {
        true
    }
    fn all_at(&mut self, _: &impl State<N>, _: f64) -> bool {
        true
    }
}
impl<const N: usize, H, T> FilterHList<N> for Cons<H, T>
where
    H: StateFnMut<N, Output = bool>,
    T: FilterHList<N>,
{
    fn all(&mut self, state: &impl State<N>) -> bool {
        let Cons(head, tail) = self;
        head.eval(state) && tail.all(state)
    }
    fn all_at(&mut self, state: &impl State<N>, t: f64) -> bool {
        let Cons(head, tail) = self;
        head.eval_at(state, t) && tail.all_at(state, t)
    }
}

// macro_rules! declare_trait_hlist {
//     ($Trait:ident, ($($Args:expr),*), ($($Wheres:expr),*)) => {
//         paste::paste!{
//             pub trait [<$Trait HList>]<$($Args),*> where $($Wheres),* {
//                 for
//
//             }
//         }
//     };
// }
//
