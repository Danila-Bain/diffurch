use crate::{state::CoordinateFunction, util::tuple_tower::TupleTower};
use std::marker::Tuple;

pub struct Event<Callback = (), Stream = TupleTower, Filter = TupleTower> {
    pub callback: Callback,
    pub stream: Stream,
    pub filter: Filter,
}

impl Event {
    pub fn new<Callback>(callback: Callback) -> Event<Callback> {
        Event {
            callback,
            stream: TupleTower(()),
            filter: TupleTower(()),
        }
    }

    pub fn ode<const N: usize, Callback, Output>(
        callback: Callback,
    ) -> Event<Callback>
    where
        Callback: Fn<([f64; N],), Output = Output>,
    {
        Event {
            callback,
            stream: TupleTower(()),
            filter: TupleTower(()),
        }
    }

    pub fn ode2<const N: usize, Callback, Output>(
        callback: Callback,
    ) -> Event<Callback>
    where
        Callback: Fn<(f64, [f64; N]), Output = Output>,
    {
        Event {
            callback,
            stream: TupleTower(()),
            filter: TupleTower(()),
        }
    }

    pub fn dde<const N: usize, Callback, Output, const S: usize, InitialFunction>(
        callback: Callback,
    ) -> Event<Callback>
    where
        Callback: for<'a> Fn<
                (
                    f64,
                    [f64; N],
                    [CoordinateFunction<'a, N, S, InitialFunction>; N],
                ),
                Output = Output,
            >,
    {
        Event {
            callback,
            stream: TupleTower(()),
            filter: TupleTower(()),
        }
    }

    // pub fn new_with_stream<Callback, Stream>(
    //     callback: Callback,
    //     stream: Stream,
    // ) -> Event<Callback, Stream> {
    //     Event::<Callback, Stream, TupleTower<()>> { callback, stream, filter: TupleTower(()) }
    // }
}

// impl<Callback, Stream, Filter, Args> FnOnce<Args> for Event<Callback, Stream, Filter>
// where
//     Args: Tuple,
//     Callback: FnOnce<Args>,
//     Stream: FnOnce<(Callback::Output,), Output=()>,
// {
//     type Output = Stream::Output;
//
//     extern "rust-call" fn call_once(self, args: Args) -> Self::Output {
//         let Self { callback, stream, filter } = self;
//         stream.call_once((callback.call_once(args),))
//     }
// }
//
// impl<Callback, Stream, Args> FnMut<Args> for Event<Callback, Stream>
// where
//     Args: Tuple,
//     Callback: FnMut<Args>,
//     Stream: FnMut<(Callback::Output,), Output=()>,
// {
//     extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output {
//         let Self { callback, stream, filter } = self;
//         stream.call_mut((callback.call_mut(args),))
//     }
// }
//
// impl<Callback, Stream, Args> Fn<Args> for Event<Callback, Stream>
// where
//     Args: Tuple,
//     Callback: Fn<Args>,
//     Stream: Fn<(Callback::Output,), Output=()>,
// {
//     extern "rust-call" fn call(&self, args: Args) -> Self::Output {
//         let Self { callback, stream, filter } = self;
//         stream.call((callback.call(args),))
//     }
// }

impl<Callback, Stream, Filter> Event<Callback, TupleTower<Stream>, TupleTower<Filter>> {
    pub fn to<Args, Output, S>(self, s: S) -> Event<Callback, TupleTower<(S, TupleTower<Stream>)>, TupleTower<Filter>>
    where
        Args: Tuple,
        Callback: Fn<Args, Output = Output>,
        S: FnMut<(Output,)>,
    {
        Event {
            callback: self.callback,
            stream: self.stream.append(s),
            filter: self.filter,
        }
    }

    pub fn to_std<Args, Output>(
        self,
    ) -> Event<Callback, TupleTower<(impl FnMut<(Output,)>, TupleTower<Stream>)>, TupleTower<Filter>>
    where
        Args: Tuple,
        Callback: Fn<Args, Output = Output>,
        Output: std::fmt::Debug,
    {
        self.to(|value: Output| println!("{:?}", value))
    }

    pub fn to_vec<Args, Output>(
        self,
        vec: &mut Vec<Output>,
    ) -> Event<Callback, TupleTower<(impl FnMut<(Output,)>, TupleTower<Stream>)>, TupleTower<Filter>>
    where
        Args: Tuple,
        Callback: Fn<Args, Output = Output>,
    {
        self.to(|value: Output| vec.push(value))
    }

    pub fn to_var<Args, Output>(
        self,
        value: &mut Output,
    ) -> Event<Callback, TupleTower<(impl FnMut<(Output,)>, TupleTower<Stream>)>, TupleTower<Filter>>
    where
        Callback: Fn<Args, Output = Output>,
        Args: Tuple,
    {
        self.to(|v: Output| *value = v)
    }

    pub fn to_vecs<const N: usize, Args>(
        self,
        vecs: [&mut Vec<f64>; N],
    ) -> Event<Callback, TupleTower<(impl FnMut<([f64; N],)>, TupleTower<Stream>)>, TupleTower<Filter>>
    where
        Args: Tuple,
        Callback: Fn<Args, Output = [f64; N]>,
    {
        self.to(move |value: [f64; N]| {
            for i in 0..N {
                vecs[i].push(value[i]);
            }
        })
    }



    pub fn filter_by<Args, F>(self, f: F) -> Event<Callback, TupleTower<Stream>, TupleTower<(F, TupleTower<Filter>)>>
    where
        Args: Tuple,
        F: FnMut<Args, Output = bool>,
    {
        Event {
            callback: self.callback,
            stream: self.stream,
            filter: self.filter.append(f),
        }
    }

    pub fn every(self, n: usize) -> Event<Callback, TupleTower<Stream>, TupleTower<(impl FnMut<(), Output=bool>, TupleTower<Filter>)>> {
        let mut counter = n - 1;
        self.filter_by(move || {
            counter += 1;
            counter -= n * (counter >= n) as usize;
            return counter == 0;
        }) 
    }

    pub fn separated_by(self, delta: f64) -> Event<Callback, TupleTower<Stream>, TupleTower<(impl FnMut<(f64,), Output=bool>, TupleTower<Filter>)>> {
        let mut last_trigger = f64::NEG_INFINITY;
        self.filter_by(move |t| {
            let res = t >= last_trigger + delta;
            last_trigger = t;
            return res;
        }) 
    }

    pub fn in_range(self, interval: std::ops::Range<f64>) -> Event<Callback, TupleTower<Stream>, TupleTower<(impl FnMut<(f64,), Output=bool>, TupleTower<Filter>)>> {
        self.filter_by(move |t| {
           interval.contains(&t)
        }) 
    }


}
