use crate::{State, ToStateFn};
// use std::marker::Tuple;

/// Event type holds several handlers that determine *what* happens when the event happens. Event
/// struct does not specify under what conditions event is triggered, the "when" part is determined
/// in [crate::solver::Solver] struct.
pub struct Event<const N: usize = 0, const S: usize = 0, Output = ()> {
    /// Function, which is called on a state. Its output is then fed to `stream`.
    pub callback: Box<dyn for<'a> Fn(&'a State<N, S>) -> Output>,
    /// Function (or rather a collection of functions), which handles the output destination and
    /// formatting provided by `callback`. It takes a single argument: the return type of `callback`.
    pub stream: Vec<Box<dyn Fn(Output)>>,
    /// Function, that filters invocations of `callback` and `stream`. Can be used to produce a
    /// more sparse output (such that there are not too many output points), or limit outputing
    /// values to a certain range, etc. It is a function, that is invoked on a state and returns
    /// bool.
    pub filter: Vec<Box<dyn for<'a> Fn<&'a State<N, S>, Output = bool>>>,
    /// When it has unit type it does nothing, when it has type `usize`, it produces dense output:
    /// the event's "filter->stream->callback" sequence is triggered not on the current state, but
    /// on `subdivision` number of points of the current step in the state, making use of dense
    /// output feature of the state.
    pub subdivision: Option<usize>,
}

impl Event {
    // /// Constructor, which defaults `steam`, and `filter` to `Tutle(())`, and `subdvision` to `()`.
    // ///
    // /// # Usage
    // /// ```
    // /// let event = diffurch::Event::new(|| (42., 69.));
    // ///
    // /// let event = diffurch::Event::new(|t: f64| t.sin());
    // ///
    // /// let event = diffurch::Event::new(|t: f64, [x, y]: [f64; 2]| (t, x, y));
    // /// ```
    pub fn new<const N: usize, const S: usize, Args, Output, C>(callback: C) -> Event<N, S, Output>
    where
        C: 'static + ToStateFn<N, S, Args, Output>,
    {
        Event {
            callback: callback.to_state_function(),
            stream: Vec::new(),
            filter: Vec::new(),
            subdivision: None,
        }
    }

    // /// Like `new`, but expects function with `[f64; N]` argument, representing coordinates of the
    // /// state. Used to avoid type annotations when using closures as an argument.
    // /// # Usage
    // /// ```
    // /// let event = diffurch::Event::ode(|[x, y]| [x + y, x - y]);
    // /// ```
    // pub fn ode<const N: usize, const S: usize, Output, C>(callback: C) -> Event<N, S, Output>
    // where
    //     C: 'static + ToStateFn<N, S, ([f64; N], ), Output>,
    // {
    //     Event {
    //         callback: callback.to_state_function(),
    //         stream: [],
    //         filter: [],
    //         subdivision: (),
    //     }
    // }
    //
    // /// Like `new`, but expects function with `f64` and `[f64; N]` arguments, representing time and
    // /// coordinates of the state. Used to avoid type annotations when using closures as an argument.
    // /// # Usage
    // /// ```
    // /// let event = diffurch::Event::ode2(|t, [x, y]| (t, x, y));
    // /// ```
    // pub fn ode2<const N: usize, C: Fn<(f64, [f64; N]), Output = Output>, Output>(
    //     callback: C,
    // ) -> Event<C> {
    //     Event::new(callback)
    // }
    //
    // /// Like `new`, but expects function with `f64`, `[f64; N]`, `[CoordFn<...>; N]` arguments, representing time, coordinates, and coordinate functions (for delayed evaluation) of the state. Used to avoid type annotations when using closures as an argument. May be necessary, because `CoordFn<...>` type could be unnameable.
    // /// # Usage
    // /// ```
    // /// use diffurch::{ToStateFn, Event, State, rk};
    // ///
    // /// let event = Event::dde(|t, [x, y], [x_, y_]| (t, x, y, x_(t - 1.), y_(t - 2.)));
    // ///
    // /// // calling actual state is needed for type inference for x_ and y_
    // /// let init_f = |t: f64| [ t.sin(), t.cos() ];
    // /// let state = State::new(0., init_f, &rk::RK98);
    // /// let mut event = event.to_state_function();
    // /// event(&state);
    // /// ```
    // pub fn dde<
    //     const N: usize,
    //     const S: usize,
    //     IF,
    //     C: for<'a> Fn<(f64, [f64; N], [CoordFn<'a, N, S, IF>; N]), Output = Output>,
    //     Output,
    // >(
    //     callback: C,
    // ) -> Event<C> {
    //     Event::new(callback)
    // }
}

impl<const N: usize, const S: usize, Output> Event<N, S, Output> {
    /// Appends given stream function to the field `stream`, and returns modified event.
    pub fn to<Args, O, Stream>(mut self, s: Stream) -> Self
    where
        Stream: 'static + Fn(Output),
    {
        self.stream.push(Box::new(s));
        self
    }
    //
    //     /// Appends function that prints its argument to `stream`, and returns modified event.
    //     pub fn to_std<Args, O>(self) -> Event<C, Tutle<(impl FnMut<(O,)>, Tutle<S>)>, Tutle<F>, D>
    //     where
    //         Args: Tuple,
    //         C: Fn<Args, Output = O>,
    //         O: std::fmt::Debug,
    //     {
    //         self.to(|value: O| println!("{:?}", value))
    //     }
    //
    //     pub fn to_file<Args, O>(
    //         self,
    //         filename: &str,
    //     ) -> Event<C, Tutle<(impl FnMut<(O,)>, Tutle<S>)>, Tutle<F>, D>
    //     where
    //         Args: Tuple,
    //         C: Fn<Args, Output = O>,
    //         O: std::fmt::Debug,
    //     {
    //         use std::io::Write;
    //         let mut file = std::fs::File::create_buffered(filename).unwrap();
    //         self.to(move |value: O| writeln!(&mut file, "{:?}", value).unwrap())
    //     }
    //
    //     pub fn to_csv<const N: usize, Args, O>(
    //         self,
    //         filename: &str,
    //     ) -> Event<C, Tutle<(impl FnMut<([O; N],)>, Tutle<S>)>, Tutle<F>, D>
    //     where
    //         Args: Tuple,
    //         C: Fn<Args, Output = [O; N]>,
    //         O: std::fmt::Display,
    //     {
    //         use std::io::Write;
    //         let mut file = std::fs::File::create_buffered(filename).unwrap();
    //         self.to(move |values: [O; N]| {
    //             for val in values {
    //                 write!(&mut file, "{},", val).unwrap();
    //             }
    //             writeln!(&mut file, "").unwrap();
    //         })
    //     }
    //
    //     pub fn to_table<const N: usize, Args, O>(
    //         self,
    //         filename: &str,
    //         separator: &str,
    //         header: Option<&str>,
    //     ) -> Event<C, Tutle<(impl FnMut<([O; N],)>, Tutle<S>)>, Tutle<F>, D>
    //     where
    //         Args: Tuple,
    //         C: Fn<Args, Output = [O; N]>,
    //         O: std::fmt::Display,
    //     {
    //         use std::io::Write;
    //         let mut file = std::fs::File::create_buffered(filename).unwrap();
    //
    //         if let Some(header) = header {
    //             writeln!(&mut file, "{header}").unwrap();
    //         }
    //         self.to(move |values: [O; N]| {
    //             for val in values {
    //                 write!(&mut file, "{}{separator}", val).unwrap();
    //             }
    //             writeln!(&mut file, "").unwrap();
    //         })
    //     }
    //
    //     /// The function that pushes its argument to provided mutable vector is appended to `stream`
    //     /// field. The modified event is returned.
    //     ///
    //     /// # Usage
    //     /// ```
    //     /// let mut points = Vec::new();
    //     ///
    //     /// let event = diffurch::Event::ode2(|t, [x, y]| (t, x, y)).to_vec(&mut points);
    //     ///
    //     /// ```
    //     pub fn to_vec<Args, Output>(
    //         self,
    //         vec: &mut Vec<Output>,
    //     ) -> Event<C, Tutle<(impl FnMut<(Output,)>, Tutle<S>)>, Tutle<F>, D>
    //     where
    //         Args: Tuple,
    //         C: Fn<Args, Output = Output>,
    //     {
    //         self.to(|value: Output| vec.push(value))
    //     }
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
}

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
