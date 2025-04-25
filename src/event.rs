use crate::state::FromState;
use std::marker::Tuple;
pub struct Event<Args : Tuple = (), Output = (), Callback: Fn<Args, Output = Output> = fn(), Stream = ()> {
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
}

impl<Args: Tuple, Output, Callback: Fn<Args, Output = Output>> Event<Args, Output, Callback, ()> {

    pub fn save<Stream: FnMut(Output)>(self, stream: Stream) -> Event<Args, Output, Callback, Stream> {
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

    pub fn to_vec(self, vec: &mut Vec<Output>) -> Event<Args, Output, Callback, impl FnMut(Output)>
    {
        self.save(|value: Output| vec.push(value))
    }


    pub fn to(self, value: &mut Output) -> Event<Args, Output, Callback, impl FnMut(Output)> {
        self.save(|v: Output|  *value = v )
    }

    pub fn call<S>(&self, s: S) where Args: FromState<S> {
        let args = Args::from_state(s);
        self.callback.call(args);
    }

}

impl<Args: Tuple, const N: usize, Callback: Fn<Args, Output=[f64; N]>> Event<Args, [f64; N], Callback, ()> {
    pub fn to_vecs(self, vecs: [&mut Vec<f64>; N]) -> Event<Args, [f64; N], Callback, impl FnMut([f64; N])> {
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

impl<Args: Tuple, Output, Callback: Fn<Args, Output = Output>, Stream: FnMut(Output)> Event<Args, Output, Callback, Stream> {
    pub fn call<S>(&mut self, s: S) where Args: FromState<S> {
        let args: Args = Args::from_state(s);
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

impl<CallArgs: Copy, Args: Tuple + FromState<CallArgs>, Output, Callback: Fn<Args, Output=Output>, Stream: FnMut(Output), Tail: CallEventTower<CallArgs>> CallEventTower<CallArgs> for (Event<Args, Output, Callback, Stream>, Tail)
{
    fn call_event_tower(&mut self, args: CallArgs) {
        let (head, tail) = self;

        head.call(args);
        tail.call_event_tower(args);
    }
}
