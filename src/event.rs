use crate::state::StateInto;
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

    pub fn call<T: StateInto<Args>>(&self, args: T) {
        let args: Args = args.state_into();
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

    pub fn to_csv(self, filename: &str) 
        // -> Event<Args, [f64; N], Callback, impl FnMut([f64; N])> 
        {
        todo!();
    }
}

impl<Args: Tuple, Output, Callback: Fn<Args, Output = Output>, Stream: FnMut(Output)> Event<Args, Output, Callback, Stream> {
    pub fn call<T: StateInto<Args>>(&mut self, args: T) {
        let args: Args = args.state_into();
        let output = self.callback.call(args);
        (self.stream)(output);
    }
}

// trait EventCall<Args> {
//     fn call<T: StateInto<Args>>(&self, args: T);
// }
//
// impl<C, Args> EventCall<Args> for Event<C, ()>
// where
//     C: Fn<Args>,
//     Args: std::marker::Tuple,
// {
//     fn call<T: StateInto<Args>>(&self, args: T) {
//         let args: Args = args.state_into();
//         self.callback.call(args);
//     }
// }
//
// impl<C, SC, Args, Ret> EventCall<Args> for Event<C, SC>
// where
//     C: Fn<Args, Output = Ret>,
//     SC: Fn(Ret),
//     Args: std::marker::Tuple,
// {
//     fn call<T: StateInto<Args>>(&self, args: T) {
//         let args: Args = args.state_into();
//         (self.save_callback)(self.callback.call(args));
//     }
// }

pub trait CallEventTower<Args> {
    fn call_event_tower(&mut self, args: Args);
}

impl<Args> CallEventTower<Args> for () {
    fn call_event_tower(&mut self, _args: Args) {}
}

impl<CallArgs: StateInto<Args> + Copy, Args: Tuple, Output, Callback: Fn<Args, Output=Output>, Stream: FnMut(Output), Tail: CallEventTower<CallArgs>> CallEventTower<CallArgs> for (Event<Args, Output, Callback, Stream>, Tail)
{
    fn call_event_tower(&mut self, args: CallArgs) {
        let (head, tail) = self;

        head.call(args);
        tail.call_event_tower(args);
    }
}

// pub fn call_event_tower<E, Tail, Args, EArgs>(tower: (E, Tail), args: Args)
// where
//     E: EventCall<EArgs>,
//     Args: std::marker::Tuple + Copy + StateInto<EArgs>,
// {
//         let (f, tail) = tower;
//         f.call(args);
//         // call_event_tower(tail, args);
// }

// pub trait CallEventTower<Args> {
//     fn call_event_tower<EArgs>(&self, args: Args);
// }
//
// impl<Args> CallEventTower<Args> for ()
// {
//     #[inline]
//     fn call_event_tower<EArgs>(&self, _arg: Args) {}
// }
//
// impl<Args, C, SC, Tail> CallEventTower<Args> for (Event<C, SC>, Tail)
//     where
// {
//     #[inline]
//     fn call_event_tower<EArgs>(&self, args: Args) where
//     C: Fn<EArgs>,
//     Args: std::marker::Tuple + Copy + StateInto<EArgs>,
//     Tail: CallEventTower<Args>,
//     {
//         let (f, tail) = self;
//         f.call(args.state_into());
//         tail.call_event_tower(args);
//     }
// }
