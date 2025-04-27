use crate::util::tuple_tower::TupleTower;
use std::marker::Tuple;

pub struct Event<Callback = (), Stream = ()> {
    callback: Callback,
    stream: Stream,
}

impl Event {
    pub fn new<Callback>(callback: Callback) -> Event<Callback, TupleTower<()>> {
        Event {
            callback,
            stream: TupleTower(()),
        }
    }

    pub fn new_with_stream<Callback, Stream>(
        callback: Callback,
        stream: Stream,
    ) -> Event<Callback, Stream> {
        Event::<Callback, Stream> { callback, stream }
    }
}

impl<Callback, Stream, Args> FnOnce<Args> for Event<Callback, Stream>
where
    Args: Tuple,
    Callback: FnOnce<Args>,
    Stream: FnOnce<(Callback::Output,)>,
{
    type Output = Stream::Output;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output {
        let Self { callback, stream } = self;
        stream.call_once((callback.call_once(args),))
    }
}

impl<Callback, Stream, Args> FnMut<Args> for Event<Callback, Stream>
where
    Args: Tuple,
    Callback: FnMut<Args>,
    Stream: FnMut<(Callback::Output,)>,
{
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output {
        let Self { callback, stream } = self;
        stream.call_mut((callback.call_mut(args),))
    }
}

impl<Callback, Stream, Args> Fn<Args> for Event<Callback, Stream>
where
    Args: Tuple,
    Callback: Fn<Args>,
    Stream: Fn<(Callback::Output,)>,
{
    extern "rust-call" fn call(&self, args: Args) -> Self::Output {
        let Self { callback, stream } = self;
        stream.call((callback.call(args),))
    }
}

impl<Callback, Stream> Event<Callback, TupleTower<Stream>> {
    pub fn to<Args, Output, S>(self, s: S) -> Event<Callback, TupleTower<(S, TupleTower<Stream>)>>
    where
        Args: Tuple,
        Callback: Fn<Args, Output = Output>,
        S: FnMut<(Output,)>,
    {
        Event {
            callback: self.callback,
            stream: self.stream.append(s),
        }
    }

    pub fn to_std<Args, Output>(
        self,
    ) -> Event<Callback, TupleTower<(impl FnMut<(Output,)>, TupleTower<Stream>)>>
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
    ) -> Event<Callback, TupleTower<(impl FnMut<(Output,)>, TupleTower<Stream>)>>
    where
        Args: Tuple,
        Callback: Fn<Args, Output = Output>,
    {
        self.to(|value: Output| vec.push(value))
    }

    pub fn to_var<Args, Output>(
        self,
        value: &mut Output,
    ) -> Event<Callback, TupleTower<(impl FnMut<(Output,)>, TupleTower<Stream>)>>
    where
        Callback: Fn<Args, Output = Output>,
        Args: Tuple,
    {
        self.to(|v: Output| *value = v)
    }

    pub fn to_vecs<const N: usize, Args>(
        self,
        vecs: [&mut Vec<f64>; N],
    ) -> Event<Callback, TupleTower<(impl FnMut<([f64; N],)>, TupleTower<Stream>)>>
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
}
