use crate::{State, StateFn};

/// Event type holds several handlers that determine *what* happens when the event happens. Event
/// struct does not specify under what conditions event is triggered, the "when" part is determined
/// in [crate::solver::Solver] struct.
pub struct Event<const N: usize = 0, Output = ()> {
    /// Function, which is called on a state. Its output is then fed to `stream`.
    pub callback: StateFn<N, Output>,
    /// Function (or rather a collection of functions), which handles the output destination and
    /// formatting provided by `callback`. It takes a single argument: the return type of `callback`.
    pub stream: Vec<Box<dyn FnMut(Output)>>,
    /// Function, that filters invocations of `callback` and `stream`. Can be used to produce a
    /// more sparse output (such that there are not too many output points), or limit outputing
    /// values to a certain range, etc. It is a function, that is invoked on a state and returns
    /// bool.
    pub filter: Vec<StateFn<N, bool>>,
    /// When it has unit type it does nothing, when it has type `usize`, it produces dense output:
    /// the event's "filter->stream->callback" sequence is triggered not on the current state, but
    /// on `subdivision` number of points of the current step in the state, making use of dense
    /// output feature of the state.
    pub subdivision: Option<usize>,
}

impl Event {
    pub fn new<const N: usize, Output>(callback: StateFn<N, Output>) -> Event<N, Output> {
        Event::<N, Output> {
            callback,
            stream: Vec::new(),
            filter: Vec::new(),
            subdivision: None,
        }
    }

    pub fn constant<const N: usize, Output>(
        callback: impl 'static + Fn() -> Output,
    ) -> Event<N, Output> {
        Event::new(StateFn::Constant(Box::new(callback)))
    }
    pub fn time<const N: usize, Output>(
        callback: impl 'static + Fn(f64) -> Output,
    ) -> Event<N, Output> {
        Event::new(StateFn::Time(Box::new(callback)))
    }
    pub fn ode<const N: usize, Output>(
        callback: impl 'static + Fn([f64; N]) -> Output,
    ) -> Event<N, Output> {
        Event::new(StateFn::ODE(Box::new(callback)))
    }
    pub fn ode2<const N: usize, Output>(
        callback: impl 'static + Fn(f64, [f64; N]) -> Output,
    ) -> Event<N, Output> {
        Event::new(StateFn::ODE2(Box::new(callback)))
    }
}

impl<const N: usize, Output> Event<N, Output> {
    pub fn to(mut self, s: impl 'static + FnMut(Output)) -> Self {
        self.stream.push(Box::new(s));
        self
    }

    pub fn to_std(self) -> Self
    where
        Output: std::fmt::Debug,
    {
        self.to(|value: Output| println!("{value:?}"))
    }

    pub fn to_file(self, filename: &str) -> Self
    where
        Output: std::fmt::Debug,
    {
        use std::io::Write;
        let mut file = std::fs::File::create_buffered(filename).unwrap();
        self.to(move |value: Output| writeln!(&mut file, "{:?}", value).unwrap())
    }

    // pub fn to_vec<'a>(self, vec: &'a mut Vec<Output>) -> Self 
    // {
    //     self.to(|value: Output| vec.push(value))
    // }
}

impl<const N: usize, Item, const M: usize> Event<N, [Item; M]> {
    pub fn to_csv(self, filename: &str) -> Self
    where
        Item: std::fmt::Display,
    {
        use std::io::Write;
        let mut file = std::fs::File::create_buffered(filename).unwrap();
        self.to(move |values: [Item; M]| {
            for val in values {
                write!(&mut file, "{},", val).unwrap();
            }
            writeln!(&mut file, "").unwrap();
        })
    }
    pub fn to_table(self, filename: &str, separator: &'static str, header: Option<&str>) -> Self
    where
        Item: std::fmt::Display,
    {
        use std::io::Write;
        let mut file = std::fs::File::create_buffered(filename).unwrap();

        if let Some(header) = header {
            writeln!(&mut file, "{header}").unwrap();
        }
        self.to(move |values: [Item; M]| {
            for val in values {
                write!(&mut file, "{}{separator}", val).unwrap();
            }
            writeln!(&mut file, "").unwrap();
        })
    }
}
//
//     /// Like [Event::to_vec], but destributes its values across several vectors
//     ///
//     /// # Usage
//     /// ```
//     /// let mut t = Vec::new();
//     /// let mut x = Vec::new();
//     /// let mut y = Vec::new();
//     ///
//     /// let event = diffurch::Event::ode2(|t, [x, y]| [t, x, y]).to_vecs([&mut t, &mut x, &mut y]);
//     /// ```
//     pub fn to_vecs<const N: usize, Args>(
//         self,
//         vecs: [&mut Vec<f64>; N],
//     ) -> Event<C, Tutle<(impl FnMut<([f64; N],)>, Tutle<S>)>, Tutle<F>, D>
//     where
//         Args: Tuple,
//         C: Fn<Args, Output = [f64; N]>,
//     {
//         self.to(move |value: [f64; N]| {
//             for i in 0..N {
//                 vecs[i].push(value[i]);
//             }
//         })
//     }
//
//     /// The function that writes its argument to provided mutable variable is appended to `stream`
//     /// field. The modified event is returned.
//     pub fn to_var<Args, Output>(
//         self,
//         value: &mut Output,
//     ) -> Event<C, Tutle<(impl FnMut<(Output,)>, Tutle<S>)>, Tutle<F>, D>
//     where
//         C: Fn<Args, Output = Output>,
//         Args: Tuple,
//     {
//         self.to(|v: Output| *value = v)
//     }
//
//     pub fn to_range<Args, Output>(
//         self,
//         range: &mut std::ops::Range<Output>,
//     ) -> Event<C, Tutle<(impl FnMut<(Output,)>, Tutle<S>)>, Tutle<F>, D>
//     where
//         C: Fn<Args, Output = Output>,
//         Args: Tuple,
//         Output: num_traits::Float,
//     {
//         {
//             *range = Output::max_value()..Output::min_value();
//         }
//         self.to(|v: Output| *range = range.start.min(v)..range.end.max(v))
//     }
//
//     pub fn to_ranges<const N: usize, Args, Output>(
//         self,
//         mut ranges: [&mut std::ops::Range<Output>; N],
//     ) -> Event<C, Tutle<(impl FnMut<([Output; N],)>, Tutle<S>)>, Tutle<F>, D>
//     where
//         C: Fn<Args, Output = [Output; N]>,
//         Args: Tuple,
//         Output: num_traits::Float,
//     {
//         for range in ranges.iter_mut() {
//             **range = Output::max_value()..Output::min_value();
//         }
//         self.to(move |values: [Output; N]| {
//             for (range, v) in ranges.iter_mut().zip(values.iter()) {
//                 **range = range.start.min(*v)..range.end.max(*v)
//             }
//         })
//     }
//
//     pub fn filter_by<Args, F_>(self, f: F_) -> Event<C, Tutle<S>, Tutle<(F_, Tutle<F>)>, D>
//     where
//         Args: Tuple,
//         F_: FnMut<Args, Output = bool>,
//     {
//         Event {
//             callback: self.callback,
//             stream: self.stream,
//             filter: self.filter.append(f),
//             subdivision: self.subdivision,
//         }
//     }
//
//     pub fn every(
//         self,
//         n: usize,
//     ) -> Event<C, Tutle<S>, Tutle<(impl FnMut<(), Output = bool>, Tutle<F>)>, D> {
//         let mut counter = n - 1;
//         self.filter_by(move || {
//             counter += 1;
//             counter -= n * (counter >= n) as usize;
//             return counter == 0;
//         })
//     }
//
//     pub fn separated_by(
//         self,
//         delta: f64,
//     ) -> Event<C, Tutle<S>, Tutle<(impl FnMut<(f64,), Output = bool>, Tutle<F>)>, D> {
//         let mut last_trigger = f64::NEG_INFINITY;
//         self.filter_by(move |t| {
//             if t >= last_trigger + delta {
//                 last_trigger = t;
//                 true
//             } else {
//                 false
//             }
//         })
//     }
//
//     pub fn in_range(
//         self,
//         interval: std::ops::Range<f64>,
//     ) -> Event<C, Tutle<S>, Tutle<(impl FnMut<(f64,), Output = bool>, Tutle<F>)>, D> {
//         self.filter_by(move |t| interval.contains(&t))
//     }
//
//     pub fn once(self) -> Event<C, Tutle<S>, Tutle<(impl FnMut<(), Output = bool>, Tutle<F>)>, D> {
//         let mut flag = true;
//         self.filter_by(move || {
//             if flag {
//                 flag = false;
//                 true
//             } else {
//                 false
//             }
//         })
//     }
//
//     pub fn take(
//         self,
//         n: usize,
//     ) -> Event<C, Tutle<S>, Tutle<(impl FnMut<(), Output = bool>, Tutle<F>)>, D> {
//         let mut counter = 0;
//         self.filter_by(move || {
//             counter += 1;
//             counter <= n
//         })
//     }
//
//     pub fn times(
//         self,
//         range: impl std::ops::RangeBounds<usize>,
//     ) -> Event<C, Tutle<S>, Tutle<(impl FnMut<(), Output = bool>, Tutle<F>)>, D> {
//         let mut counter = 0;
//         self.filter_by(move || {
//             let ret = range.contains(&counter);
//             counter += 1;
//             ret
//         })
//     }

// impl<C, S, F> Event<C, Tutle<S>, Tutle<F>, ()> {
//     pub fn subdivide(self, n: usize) -> Event<C, Tutle<S>, Tutle<F>, usize> {
//         Event {
//             callback: self.callback,
//             stream: self.stream,
//             filter: self.filter,
//             subdivision: n,
//         }
//     }
// }
