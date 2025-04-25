use std::marker::Tuple;

pub struct Event<
    // Args: Tuple = (),
    // Output = (),
    Callback = (),
    Stream = (),
> {
    callback: Callback,
    stream: Stream,
}

impl Event {
    // pub fn new<
    //     'a,
    //     const N: usize,
    //     const S: usize,
    //     InitialFunction: Fn(f64) -> [f64; N],
    //     Args: Tuple,
    //     Output,
    //     Callback: Fn<Args, Output = Output> + ToStateFunction<State<N, S, InitialFunction>, Args, Output>,
    // >(
    //     callback: Callback,
    // ) -> Event<impl for<'b> Fn<(&'b State<N, S, InitialFunction>,), Output = Output>, ()> {
    //     Event::new_unfiltered(callback.to_state_function())
    // }

    pub fn new<Args: Tuple, Output, Callback: Fn<Args, Output = Output>>(
        callback: Callback,
    ) -> Event<Callback, ()> {
        Event::<Callback, ()> {
            callback,
            stream: (),
        }
    }
    //
    // pub fn new_with_stream<Args: Tuple, Output, Callback: Fn<Args, Output = Output>, Stream>(
    //     callback: Callback,
    //     stream: Stream,
    // ) -> Event<Args, Output, Callback, Stream> {
    //     Event::<Args, Output, Callback, Stream> {
    //         callback,
    //         stream,
    //         args: std::marker::PhantomData::<Args> {},
    //         output: std::marker::PhantomData::<Output> {},
    //     }
    // }
}

impl<Callback> Event<Callback, ()> {
    pub fn save<Args, Output, Stream: FnMut(Output)>(
        self,
        stream: Stream,
    ) -> Event<Callback, Stream>
    where
        Args: Tuple,
        Callback: Fn<Args, Output = Output>,
    {
        Event::<Callback, Stream> {
            callback: self.callback,
            stream,
        }
    }

    pub fn to_std<Args, Output>(self) -> Event<Callback, impl FnMut(Output)>
    where
        Args: Tuple,
        Callback: Fn<Args, Output = Output>,
        Output: std::fmt::Debug,
    {
        self.save(|value: Output| println!("{:?}", value))
    }

    pub fn to_vec<Args, Output>(self, vec: &mut Vec<Output>) -> Event<Callback, impl FnMut(Output)>
    where
        Callback: Fn<Args, Output = Output>,
        Args: Tuple,
    {
        self.save(|value: Output| vec.push(value))
    }

    pub fn to<Args, Output>(self, value: &mut Output) -> Event<Callback, impl FnMut(Output)>
    where
        Callback: Fn<Args, Output = Output>,
        Args: Tuple,
    {
        self.save(|v: Output| *value = v)
    }

    // pub fn call<Args: Tuple>(&self, args: Args)
    // where
    //     Callback: Fn<Args>,
    // {
    //     self.callback.call(args);
    // }

    pub fn to_vecs<Args, const N: usize>(
        self,
        vecs: [&mut Vec<f64>; N],
    ) -> Event<Callback, impl FnMut([f64; N])>
    where
        Args: Tuple,
        Callback: Fn<Args, Output = [f64; N]>,
    {
        self.save(move |value: [f64; N]| {
            for i in 0..N {
                vecs[i].push(value[i]);
            }
        })
    }
}

impl<Callback, Stream> Event<Callback, Stream> {
    pub fn call<Args, Output>(&mut self, args: Args)
    where
        Args: Tuple,
        Callback: Fn<Args, Output = Output>,
        Stream: FnMut(Output),
    {
        let output = self.callback.call(args);
        (self.stream)(output);
    }
}

pub trait CallEventTower<Args> {
    fn call_event_tower(&mut self, args: Args);
}

impl<Args> CallEventTower<Args> for () {
    fn call_event_tower(&mut self, _args: Args) {}
}

impl<Args, Output, Callback, Stream, Tail> CallEventTower<Args> for (Event<Callback, Stream>, Tail)
where
    Args: Tuple + Copy,
    Callback: Fn<Args, Output = Output>,
    Stream: FnMut(Output),
    Tail: CallEventTower<Args>,
{
    fn call_event_tower(&mut self, args: Args) {
        let (head, tail) = self;

        head.call(args);
        tail.call_event_tower(args);
    }
}

// impl<Args, Output, Callback, Tail> CallEventTower<Args>
//     for (Event<Args, Output, Callback, ()>, Tail)
// where
//     Args: Tuple + Copy,
//     Callback: Fn<Args, Output = Output>,
//     Tail: CallEventTower<Args>,
// {
//     fn call_event_tower(&mut self, args: Args) {
//         let (head, tail) = self;
//
//         head.call(args);
//         tail.call_event_tower(args);
//     }
// }

use crate::state::{State, ToStateFunction};

//
// pub trait IsEvent {}
//
// impl<
//     Args: Tuple,
//     Output,
//     Callback: Fn<Args, Output = Output>,
//     Stream,
//     >
//     IsEvent for Event<Args, Output, Callback, Stream> {}

// pub trait ToStateEvent<S> {
//     fn to_state_event(self) -> impl IsEvent;
// }
// impl<
//     Args: Tuple,
//     Output,
//     Callback: Fn<Args, Output = Output> + ToStateFunction<State<N,S,InitialFunction>, Args, Output>,
//     Stream,
//     const N: usize,
//     const S: usize,
//     InitialFunction: Fn(f64) -> [f64; N],
// > ToStateEvent<State<N,S,InitialFunction>> for Event<Args, Output, Callback, Stream>
// {
//     fn to_state_event(self) -> impl IsEvent {
//         Event::new_with_stream(self.callback.to_state_function(), self.stream)
//     }
// }

pub trait ToStateEventTower<S> {
    fn to_state_event_tower(self) -> impl for<'a> CallEventTower<&'a S>;
}

impl<const N: usize, const S: usize, InitialFunction: Fn(f64) -> [f64; N]>
    ToStateEventTower<State<N, S, InitialFunction>> for ()
{
    fn to_state_event_tower(self) -> impl for<'a> CallEventTower<&'a State<N, S, InitialFunction>> {
        ()
    }
}

// trait ToStateEvent<S> {
//     fn to_state_event(self);
// }
//
// impl<
//     const N: usize,
//     const S: usize,
//     InitialFunction: Fn(f64) -> [f64; N],
//     Head: ToStateEvent<State<N, S, InitialFunction>>,
//     Tail: ToStateEventTower<State<N, S, InitialFunction>>,
// > ToStateEventTower<State<N, S, InitialFunction>> for (Head, Tail)
//     where
// {
//     fn to_state_event_tower(self) -> impl for<'a> CallEventTower<&'a State<N, S, InitialFunction>> {
//         let (head, tail) = self;
//         (head.to_state_event(), tail.to_state_event_tower())
//     }
// }

// impl<
//     const N: usize,
//     const S: usize,
//     InitialFunction: Fn(f64) -> [f64; N],
//     Args: Tuple,
//     Output,
//     Callback: Fn<Args, Output = Output> + ToStateFunction<State<N, S, InitialFunction>, Args, Output>,
//     Stream: FnMut(Output),
//     Tail: ToStateEventTower<State<N, S, InitialFunction>>,
// > ToStateEventTower<State<N, S, InitialFunction>>
//     for (Event<Args, Output, Callback, Stream>, Tail)
//     where
//         // (Event<&State<N, S, InitialFunction>, Output, ???, Stream>)
// {
//     fn to_state_event_tower(self) -> impl for<'a> CallEventTower<&'a State<N, S, InitialFunction>> {
//         let (head, tail) = self;
//         let head = Event::new_with_stream(head.callback.to_state_function(), head.stream);
//         let tail = tail.to_state_event_tower();
//         (head, tail)
//     }
// }

// pub trait ToStateFunction<S, Arg, Ret> {
//     fn to_state_function(self) -> impl Fn(&S) -> Ret;
// }
//
//
// impl<const N: usize, const S: usize, InitialFunction: Fn(f64) -> [f64; N], F, Ret>
//     ToStateFunction<State<N, S, InitialFunction>, (f64,), Ret> for F
// where
//     F: Fn<(f64,), Output = Ret>,
// {
//     fn to_state_function(self) -> impl Fn(&State<N, S, InitialFunction>) -> Ret {
//         move |state| self(state.t)
//     }
// }
//
