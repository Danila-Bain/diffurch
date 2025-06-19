use hlist2::{ops::ToRef, Cons, HList, Nil};

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

/// The trait for [HList]'s of ([Loc], [EventCall]) pairs.
pub trait LocEventHList<const N: usize>: ToRef {
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
impl<const N: usize, L, E, T> LocEventHList<N> for Cons<(L, E), T>
where
    L: Locate<N>,
    E: EventCall<N>,
    T: LocEventHList<N>,
{
    fn locate_first<S: State<N>>(
        &mut self,
        state: &mut S,
    ) -> Option<(f64, &mut dyn EventCallConcrete<N, S>)> {
        let Cons((loc, event), tail) = self;

        let head = loc.locate(state).and_then(|t| {
            let event: &mut dyn EventCallConcrete<N, S> = event;
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
    H: StateFnMut<N, bool>,
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
