use crate::{state::CoordFn, util::tutle::Tutle};
use std::marker::Tuple;

pub struct Event<C = (), S = Tutle, F = Tutle> {
    pub callback: C,
    pub stream: S,
    pub filter: F,
}

impl Event {
    pub fn new<C>(callback: C) -> Event<C> {
        Event {
            callback,
            stream: Tutle(()),
            filter: Tutle(()),
        }
    }

    pub fn ode<const N: usize, C, Output>(callback: C) -> Event<C>
    where
        C: Fn<([f64; N],), Output = Output>,
    {
        Event {
            callback,
            stream: Tutle(()),
            filter: Tutle(()),
        }
    }

    pub fn ode2<const N: usize, C, Output>(callback: C) -> Event<C>
    where
        C: Fn<(f64, [f64; N]), Output = Output>,
    {
        Event {
            callback,
            stream: Tutle(()),
            filter: Tutle(()),
        }
    }

    pub fn dde<const N: usize, C, Output, const S: usize, IF>(callback: C) -> Event<C>
    where
        C: for<'a> Fn<(f64, [f64; N], [CoordFn<'a, N, S, IF>; N]), Output = Output>,
    {
        Event {
            callback,
            stream: Tutle(()),
            filter: Tutle(()),
        }
    }
}


impl<C, S, F> Event<C, Tutle<S>, Tutle<F>> {
    pub fn to<Args, O, S_>(self, s: S_) 
        -> Event<C, Tutle<(S_, Tutle<S>)>, Tutle<F>>
    where
        Args: Tuple,
        C: Fn<Args, Output = O>,
        S_: FnMut<(O,)>,
    {
        Event {
            callback: self.callback,
            stream: self.stream.append(s),
            filter: self.filter,
        }
    }

    pub fn to_std<Args, O>(
        self,
    ) -> Event<C, Tutle<(impl FnMut<(O,)>, Tutle<S>)>, Tutle<F>>
    where
        Args: Tuple,
        C: Fn<Args, Output = O>,
        O: std::fmt::Debug,
    {
        self.to(|value: O| println!("{:?}", value))
    }

    pub fn to_vec<Args, Output>(
        self,
        vec: &mut Vec<Output>,
    ) -> Event<C, Tutle<(impl FnMut<(Output,)>, Tutle<S>)>, Tutle<F>>
    where
        Args: Tuple,
        C: Fn<Args, Output = Output>,
    {
        self.to(|value: Output| vec.push(value))
    }

    pub fn to_var<Args, Output>(
        self,
        value: &mut Output,
    ) -> Event<C, Tutle<(impl FnMut<(Output,)>, Tutle<S>)>, Tutle<F>>
    where
        C: Fn<Args, Output = Output>,
        Args: Tuple,
    {
        self.to(|v: Output| *value = v)
    }

    pub fn to_vecs<const N: usize, Args>(
        self,
        vecs: [&mut Vec<f64>; N],
    ) -> Event<C, Tutle<(impl FnMut<([f64; N],)>, Tutle<S>)>, Tutle<F>>
    where
        Args: Tuple,
        C: Fn<Args, Output = [f64; N]>,
    {
        self.to(move |value: [f64; N]| {
            for i in 0..N {
                vecs[i].push(value[i]);
            }
        })
    }

    pub fn filter_by<Args, F_>(self, f: F_) -> Event<C, Tutle<S>, Tutle<(F_, Tutle<F>)>>
    where
        Args: Tuple,
        F_: FnMut<Args, Output = bool>,
    {
        Event {
            callback: self.callback,
            stream: self.stream,
            filter: self.filter.append(f),
        }
    }

    pub fn every(
        self,
        n: usize,
    ) -> Event<C, Tutle<S>, Tutle<(impl FnMut<(), Output = bool>, Tutle<F>)>> {
        let mut counter = n - 1;
        self.filter_by(move || {
            counter += 1;
            counter -= n * (counter >= n) as usize;
            return counter == 0;
        })
    }

    pub fn separated_by(
        self,
        delta: f64,
    ) -> Event<C, Tutle<S>, Tutle<(impl FnMut<(f64,), Output = bool>, Tutle<F>)>> {
        let mut last_trigger = f64::NEG_INFINITY;
        self.filter_by(move |t| {
            if t >= last_trigger + delta {
                last_trigger = t;
                true
            } else { false }
        })
    }

    pub fn in_range(
        self,
        interval: std::ops::Range<f64>,
    ) -> Event<C, Tutle<S>, Tutle<(impl FnMut<(f64,), Output = bool>, Tutle<F>)>> {
        self.filter_by(move |t| interval.contains(&t))
    }

    pub fn once( self) -> Event<C,Tutle<S>, Tutle<(impl FnMut<(), Output = bool>, Tutle<F>)>> {
        let mut flag = true;
        self.filter_by(move || {
            if flag {
                flag = false;
                true
            } else {
                false
            }
        })
    }

    pub fn first(
        self,
        n: usize,
    ) -> Event<C, Tutle<S>, Tutle<(impl FnMut<(), Output = bool>, Tutle<F>)>> {
        let mut counter = 0;
        self.filter_by(move || {
            counter += 1;
            counter <= n
        })
    }


    pub fn times(
        self,
        range: std::ops::Range<usize>,
    ) -> Event<C, Tutle<S>, Tutle<(impl FnMut<(), Output = bool>, Tutle<F>)>> {
        let mut counter = 0;
        self.filter_by(move || {
            let ret = range.contains(&counter);
            counter += 1;
            ret
        })
    }
}
