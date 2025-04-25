use std::marker::Tuple;
pub struct Event<
    Args: Tuple = (),
    Output = (),
    Callback: Fn<Args, Output = Output> = fn(),
    Stream = (),
> {
    callback: Callback,
    stream: Stream,
    args: std::marker::PhantomData<Args>,
    output: std::marker::PhantomData<Output>,
}

impl Event {
    pub fn new<Args: Tuple, Output, Callback: Fn<Args, Output = Output>>(
        callback: Callback,
    ) -> Event<Args, Output, Callback, ()> {
        Event::<Args, Output, Callback, ()> {
            callback,
            stream: (),
            args: std::marker::PhantomData::<Args> {},
            output: std::marker::PhantomData::<Output> {},
        }
    }

    pub fn new_with_stream<Args: Tuple, Output, Callback: Fn<Args, Output = Output>, Stream>(
        callback: Callback,
        stream: Stream,
    ) -> Event<Args, Output, Callback, Stream> {
        Event::<Args, Output, Callback, Stream> {
            callback,
            stream,
            args: std::marker::PhantomData::<Args> {},
            output: std::marker::PhantomData::<Output> {},
        }
    }
}

impl<Args: Tuple, Output, Callback: Fn<Args, Output = Output>> Event<Args, Output, Callback, ()> {
    pub fn save<Stream: FnMut(Output)>(
        self,
        stream: Stream,
    ) -> Event<Args, Output, Callback, Stream> {
        Event::<Args, Output, Callback, Stream> {
            callback: self.callback,
            stream,
            args: std::marker::PhantomData::<Args> {},
            output: std::marker::PhantomData::<Output> {},
        }
    }

    pub fn to_std(self) -> Event<Args, Output, Callback, impl FnMut(Output)>
    where
        Output: std::fmt::Debug,
    {
        self.save(|value: Output| println!("{:?}", value))
    }

    pub fn to_vec(
        self,
        vec: &mut Vec<Output>,
    ) -> Event<Args, Output, Callback, impl FnMut(Output)> {
        self.save(|value: Output| vec.push(value))
    }

    pub fn to(self, value: &mut Output) -> Event<Args, Output, Callback, impl FnMut(Output)> {
        self.save(|v: Output| *value = v)
    }

    pub fn call(&self, args: Args) {
        self.callback.call(args);
    }
}

impl<Args: Tuple, const N: usize, Callback: Fn<Args, Output = [f64; N]>>
    Event<Args, [f64; N], Callback, ()>
{
    pub fn to_vecs(
        self,
        vecs: [&mut Vec<f64>; N],
    ) -> Event<Args, [f64; N], Callback, impl FnMut([f64; N])> {
        self.save(move |value: [f64; N]| {
            for i in 0..N {
                vecs[i].push(value[i]);
            }
        })
    }

    // pub fn to_csv(self, filename: &str)
    //     // -> Event<Args, [f64; N], Callback, impl FnMut([f64; N])>
    //     {
    //     todo!();
    // }
}

impl<Args: Tuple, Output, Callback: Fn<Args, Output = Output>, Stream: FnMut(Output)>
    Event<Args, Output, Callback, Stream>
{
    pub fn call(&mut self, args: Args) {
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

impl<Args, Output, Callback, Stream, Tail> CallEventTower<Args>
    for (Event<Args, Output, Callback, Stream>, Tail)
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

use crate::state::{State, ToStateFunction};

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

// impl<
//     Args,
//     Output,
//     Callback,
//     Stream,
//     Tail,
//     const N: usize,
//     const S: usize,
//     InitialFunction: Fn(f64) -> [f64; N],
// > ToStateEventTower<State<N, S, InitialFunction>> for (Event<Args, Output, Callback, Stream>, Tail)
// where
//     Args: Tuple + Copy,
//     Callback: Fn<Args, Output = Output>,
//     Callback: ToStateFunction<State<N, S, InitialFunction>, Args, Output>,
//     Tail: ToStateEventTower<State<N, S, InitialFunction>>,
//     (Event<(State<N,S,InitialFunction>,), Output, _, Stream>, ()) : for <'a> CallEventTower<&'a State<N, S, InitialFunction>>,
// {
//     fn to_state_event_tower(self) -> impl for<'a> CallEventTower<&'a State<N, S, InitialFunction>> {
//         let (head, tail) = self;
//         let event = Event::new_with_stream(head.callback.to_state_function(), head.stream);
//         ()
//         // (Event::new_with_stream(head.callback.to_state_function(), head.stream), tail.to_state_event_tower())
//     }
// }
//     fn to_state_event_tower<'a>(
//         self,
//     ) ->
//         impl CallEventTower<&'a State<N, S, InitialFunction>>
//      {
//         let (head, tail) = self;
//         (
//             Event::new_with_stream(head.callback.to_state_function(), head.stream),
//             tail.to_state_event_tower(),
//         )
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
