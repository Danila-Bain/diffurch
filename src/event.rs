//! Defines [Event]

use crate::{mut_state_fn, state::*, collections::hlists::FilterHList, collections::hlists::StreamHList};
use hlist2::{HList, Nil, ops::Append};
// pub trait EventStream: HList + Append {}
// impl<T: HList + Append> EventStream for T {}

/// Event type holds several handlers that determine *what* happens when the event happens. Event
/// struct does not specify under what conditions event is triggered, the "when" part is determined
/// in [crate::solver::Solver] struct.
pub struct Event<
    const N: usize,
    const MUT: bool = false,
    Subdivision = (),
    Callback = (),
    Output = (),
    Stream = Nil,
    Filter = Nil,
> {
    /// Function, which is called on a state. Its output is then fed to `stream`.
    pub callback: Callback,
    /// Function (or rather a collection of functions), which handles the output destination and
    /// formatting provided by `callback`. It takes a single argument: the return type of `callback`.
    pub stream: Stream, 
    /// Function, that filters invocations of `callback` and `stream`. Can be used to produce a
    /// more sparse output (such that there are not too many output points), or limit outputing
    /// values to a certain range, etc. It is a function, that is invoked on a state and returns
    /// bool.
    pub filter: Filter, // Vec<StateFn<'a, N, bool>>,
    /// When it has unit type it does nothing, when it has type `usize`, it produces dense output:
    /// the event's "filter->stream->callback" sequence is triggered not on the current state, but
    /// on `subdivision` number of points of the current step in the state, making use of dense
    /// output feature of the state.
    pub subdivision: Subdivision,
    output_marker: std::marker::PhantomData<fn(Output)>,
}

// Constructors
impl<const N: usize> Event<N> {
    /// Generic constructor for events that do not mutate state.
    pub fn new<F: StateFnMut<N, Output = Output>, Output>(callback: F) -> Event<N, false, (), F, Output> {
        Event {
            callback,
            stream: Nil,
            filter: Nil,
            subdivision: (),
            output_marker: Default::default(),
        }
    }
    /// Generic constructor for events that mutate state.
    pub fn new_mut<F: MutStateFnMut<N, Output = Output>, Output>(
        callback: F,
    ) -> Event<N, true, (), F, Output> {
        Event {
            callback,
            stream: Nil,
            filter: Nil,
            subdivision: (),
            output_marker: Default::default(),
        }
    }

    /// Creates an event, that sets the time of the state to `f64::INFINITY`, effectively stopping the integration.
    ///
    /// A short-hand for `Event::time_mut(|t| *t = f64::INFINITY))`.
    pub fn stop_integration() -> Event<N, true, (), impl MutStateFnMut<N, Output = f64>, f64> {
        Event::new_mut(mut_state_fn!(
                |t| {
                    let tt = *t;
                    *t = f64::INFINITY;
                    tt
                }
        ))
    }
    /// Creates an event, the callback of which returns the coordinate vector of the state.
    ///
    /// A short-hand for `Event::new(MutStateFn::ode(|x| x))`.
    pub fn ode_state() -> Event<N, false, (), impl StateFnMut<N, Output= [f64; N]>, [f64; N]> {
        Event::new(ODEStateFnMut(|x| x))
    }

    /// Creates an event, the callback of which returns the time and coordinate vector of the state.
    ///
    /// A short-hand for `Event::new(MutStateFn::ode2(|t, x| (t, x)))`.
    pub fn ode2_state() -> Event<N, false, (), impl StateFnMut<N, Output = (f64, [f64; N])>, (f64, [f64; N])>
    {
        Event::new(ODE2StateFnMut(|t, x| (t, x)))
    }
}

// impl Filter for Event
impl<const N: usize, const MUT: bool, Subdivision, Callback, Output, Stream, Filter: HList + Append>
    crate::Filter<N> for Event<N, MUT, Subdivision, Callback, Output, Stream, Filter>
{
    type Output<T> =
        Event<N, MUT, Subdivision, Callback, Output, Stream, <Filter as Append>::Output<T>>;

    fn filter<F: StateFnMut<N, Output = bool>>(self, f: F) -> Self::Output<F> {
        let callback = self.callback;
        let stream = self.stream;
        let filter = self.filter.append(f);
        let subdivision = self.subdivision;
        Event {
            callback,
            stream,
            filter,
            subdivision,
            output_marker: self.output_marker,
        }
    }
}

impl<const N: usize, const MUT: bool, Subdivision, Callback, Stream, Filter, Output>
    Event<N, MUT, Subdivision, Callback, Output, Stream, Filter>
{
    /// Set [Event::subdivision] field to `Some(n)`, that tells the solver, that event needs to be
    /// triggered `n` times on the current step.
    pub fn subdivide(self, n: usize) -> Event<N, MUT, usize, Callback, Output, Stream, Filter> {
        let callback = self.callback;
        let stream = self.stream;
        let filter = self.filter;
        let subdivision = n;
        Event {
            callback,
            stream,
            filter,
            subdivision,
            output_marker: self.output_marker,
        }
    }
}

// appenders for stream field
impl<const N: usize, const MUT: bool, Subdivision, Callback, Stream, Filter, Output>
    Event<N, MUT, Subdivision, Callback, Output, Stream, Filter>
where
    Self: Sized,
    Stream: HList + Append,
{
    /// Push a new function to [Event::stream].
    pub fn to<F: FnMut(Output)>(
        self,
        s: F,
    ) -> Event<N, MUT, Subdivision, Callback, Output, <Stream as Append>::Output<F>, Filter> {
        let callback = self.callback;
        let stream = self.stream.append(s);
        let filter = self.filter;
        let subdivision = self.subdivision;
        Event {
            callback,
            stream,
            filter,
            subdivision,
            output_marker: self.output_marker,
        }
    }

    /// Push a new function to [Event::stream], that prints the output of [Event::callback] to the
    /// standard output using `println!`.
    pub fn to_std(
        self,
    ) -> Event<
        N,
        MUT,
        Subdivision,
        Callback,
        Output,
        <Stream as Append>::Output<impl FnMut(Output)>,
        Filter,
    >
    where
        Output: std::fmt::Debug,
    {
        self.to(|value: Output| println!("{value:?}"))
    }

    /// Push a new function to [Event::stream], that opens file with a given filename and writes
    /// the output of [Event::callback] in that file.
    ///
    /// Panics, if file opening or writing fails.
    pub fn to_file(
        self,
        filename: &str,
    ) -> Event<
        N,
        MUT,
        Subdivision,
        Callback,
        Output,
        <Stream as Append>::Output<impl FnMut(Output)>,
        Filter,
    >
    where
        Output: std::fmt::Debug,
    {
        use std::io::Write;
        let mut file = std::fs::File::create_buffered(filename).unwrap();
        self.to(move |value: Output| writeln!(&mut file, "{:?}", value).unwrap())
    }

    /// Push a new function to [Event::stream], that pushes the output of [Event::callback] to the
    /// provided vector.
    pub fn to_vec(
        self,
        vec: &mut Vec<Output>,
    ) -> Event<
        N,
        MUT,
        Subdivision,
        Callback,
        Output,
        <Stream as Append>::Output<impl FnMut(Output)>,
        Filter,
    > {
        self.to(|value: Output| vec.push(value))
    }

    /// Push a new function to [Event::stream], that writes the output of [Event::callback] to the
    /// provided variable.
    pub fn to_var(
        self,
        value: &mut Output,
    ) -> Event<
        N,
        MUT,
        Subdivision,
        Callback,
        Output,
        <Stream as Append>::Output<impl FnMut(Output)>,
        Filter,
    > {
        self.to(|v: Output| *value = v)
    }

    /// Push a new function to [Event::stream], that updates the range such that it represents the
    /// minimal closed interval that contains all the values outputed from [Event::callback] so
    /// far.
    ///
    /// The range is initialized to `(+oo .. -oo)`.
    pub fn to_float_range(
        self,
        range: &mut std::ops::Range<Output>,
    ) -> Event<
        N,
        MUT,
        Subdivision,
        Callback,
        Output,
        <Stream as Append>::Output<impl FnMut(Output)>,
        Filter,
    >
    where
        Output: num_traits::Float,
    {
        *range = Output::infinity()..Output::neg_infinity();
        self.to(|v: Output| *range = range.start.min(v)..range.end.max(v))
    }
}

// appenders for stream field for array output
impl<const N: usize, const M: usize, const MUT: bool, Subdivision, Callback, Item, Stream, Filter>
    Event<N, MUT, Subdivision, Callback, [Item; M], Stream, Filter>
where
    Stream: HList + Append,
{
    /// Like [Event::to_file] but only works with arrays, and prints array as a comma-separated values.
    pub fn to_csv(
        self,
        filename: &str,
    ) -> Event<
        N,
        MUT,
        Subdivision,
        Callback,
        [Item; M],
        <Stream as Append>::Output<impl FnMut([Item; M])>,
        Filter,
    >
    where
        Item: std::fmt::Display,
    {
        use std::io::Write;
        let mut file = std::fs::File::create_buffered(filename).unwrap();
        self.to(move |values: [Item; M]| {
            for val in values {
                write!(&mut file, "{},", val).unwrap();
            }
            write!(&mut file, "\n").unwrap();
        })
    }
    /// Like [Event::to_file] but only works with arrays, and prints array values separated by a separator, and adds the header line if it is provided.
    pub fn to_table(
        self,
        filename: &str,
        separator: &str,
        header: Option<&str>,
    ) -> Event<
        N,
        MUT,
        Subdivision,
        Callback,
        [Item; M],
        <Stream as Append>::Output<impl FnMut([Item; M])>,
        Filter,
    >
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

    /// Like [Event::to_vec], but pushes the values of [Event::callback] output in individual
    /// vectors.
    pub fn to_vecs(
        self,
        vecs: [&mut Vec<Item>; M],
    ) -> Event<
        N,
        MUT,
        Subdivision,
        Callback,
        [Item; M],
        <Stream as Append>::Output<impl FnMut([Item; M])>,
        Filter,
    >
    where
        Item: Copy,
    {
        self.to(move |value: [Item; M]| {
            for i in 0..M {
                vecs[i].push(value[i]);
            }
        })
    }

    /// Like [Event::to_float_range], but updates several individual ranges.
    pub fn to_float_ranges(
        self,
        mut ranges: [&mut std::ops::Range<Item>; M],
    ) -> Event<
        N,
        MUT,
        Subdivision,
        Callback,
        [Item; M],
        <Stream as Append>::Output<impl FnMut([Item; M])>,
        Filter,
    >
    where
        Item: num_traits::Float,
    {
        for range in ranges.iter_mut() {
            **range = Item::max_value()..Item::min_value();
        }
        self.to(move |values: [Item; M]| {
            for (range, v) in ranges.iter_mut().zip(values.iter()) {
                **range = range.start.min(*v)..range.end.max(*v)
            }
        })
    }
}

/// Trait that abstracts over struct [Event]
pub trait EventCall<const N: usize> {
    /// Initiate event callback
    fn call(&mut self, state: &mut impl State<N>);
}

/// Like [EventCall], but generic state is attached to a trait instead of the method.
pub trait EventCallConcrete<const N: usize, S: State<N>> {
    /// Initiate event callback
    fn call(&mut self, state: &mut S);
}
impl<const N: usize, S: State<N>, EC: EventCall<N>> EventCallConcrete<N, S> for EC {
    fn call(&mut self, state: &mut S) {
        self.call(state);
    }
}


impl<const N: usize, Callback, Output, Stream, Filter> EventCall<N>
    for Event<N, false, (), Callback, Output, Stream, Filter>
where
    Callback: StateFnMut<N, Output = Output>,
    Output: Copy,
    Stream: StreamHList<Output>,
    Filter: FilterHList<N>,
{
    fn call(&mut self, state: &mut impl State<N>) {
        if self.filter.all(state) {
            let output = self.callback.eval(state);
            self.stream.call_each(output);
        }
    }
}

impl<const N: usize, Callback, Output, Stream, Filter> EventCall<N>
    for Event<N, false, usize, Callback, Output, Stream, Filter>
where
    Callback: StateFnMut<N, Output = Output>,
    Output: Copy,
    Stream: StreamHList<Output>,
    Filter: FilterHList<N>,
{
    fn call(&mut self, state: &mut impl State<N>) {
        for i in 1..self.subdivision {
            let t = state.t_prev()
                + (state.t() - state.t_prev()) * (i as f64) / (self.subdivision as f64);
            if self.filter.all_at(state, t) {
                let output = self.callback.eval_at(state, t);
                self.stream.call_each(output);
            }
        }
        if self.filter.all(state) {
            let output = self.callback.eval(state);
            self.stream.call_each(output);
        }
    }
}

impl<const N: usize, Callback, Output, Stream, Filter> EventCall<N>
    for Event<N, true, (), Callback, Output, Stream, Filter>
where
    Callback: MutStateFnMut<N, Output=  Output>,
    Output: Copy,
    // Stream: FnMutHList<(Output,)>,
    Stream: StreamHList<Output>,
    Filter: FilterHList<N>,
{
    fn call(&mut self, state: &mut impl State<N>) {
        if self.filter.all(state) {
            state.make_zero_step();
            let output = self.callback.eval_mut(state);
            state.push_current();
            self.stream.call_each(output);
        }
    }
}

/// Creates a [crate::Event] from a closure.
///
/// `event!` allows `Event` to be defined with closures of different calling signatures,
/// being a replacement of some constructors of [crate::Event]:
///
/// ```rust
/// #![feature(generic_const_exprs)]
/// #![allow(incomplete_features)]
///
/// use diffurch::event;
///
/// // use in solver for generic parameters inference
/// let solver = diffurch::Solver::new()
///     .on_step(event!(|| 1.)) // equivalent to .on_step(Event::constant(...))
///     .on_step(event!(|t| t + t.cos())) // equivalent to .on_step(Event::time(...))
///     .on_step(event!(|[x, y]| [x, y, x+y])) // equivalent to .on_step(Event::ode(...))
///     .on_step(event!(|t, [x, y]| [t, x, y])) // equivalent to .on_step(Event::ode2(...))
///     .on_step(event!(|t, [x, y], [x_, y_]| [t, x, x_(t - 1.)])) // equivalent to .on_step(Event::dde(...))
///     .on_step(event!(|t, [x, y], [x_, y_]| [t, x, x_(t - 1.), x_.d(t - 1.)])); // equivalent to .on_step(Event::dde(...))
/// ```
///
/// For state mutating events, use [event_mut!].
#[macro_export]
macro_rules! event {
    ($($expr:tt)*) => {
        $crate::Event::new($crate::state_fn!($($expr)*))
    };
}

/// State-mutating counter-part of [event!].
///
/// `event_mut!` allows `Event` to be defined with closures of different calling signatures,
/// being a replacement of some constructors of [crate::Event]:
///
/// ```rust
/// #![feature(generic_const_exprs)]
/// #![allow(incomplete_features)]
///
/// use diffurch::event_mut;
///
/// // use in solver for generic parameters inference
/// let solver = diffurch::Solver::new()
///     .on_step(event_mut!(|t| *t = f64::INFINITY))
///     .on_step(event_mut!(|[x, y]| {*x = -*x; [*x, *y, *x + *y]}))
///     .on_step(event_mut!(|t, [x, y]| {*x = -*y; *t = f64::INFINITY;}));
/// ```
///
#[macro_export]
macro_rules! event_mut {
    ($($expr:tt)*) => {
        $crate::Event::new_mut($crate::mut_state_fn!($($expr)*))
    };
}
