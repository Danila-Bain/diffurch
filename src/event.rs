//! Defines [Event]

use crate::{MutStateFn, StateCoordFnTrait, StateFn};

/// Event type holds several handlers that determine *what* happens when the event happens. Event
/// struct does not specify under what conditions event is triggered, the "when" part is determined
/// in [crate::solver::Solver] struct.
pub struct Event<'a, const N: usize = 0, Output = ()> {
    /// Function, which is called on a state. Its output is then fed to `stream`.
    pub callback: MutStateFn<'a, N, Output>,
    /// Function (or rather a collection of functions), which handles the output destination and
    /// formatting provided by `callback`. It takes a single argument: the return type of `callback`.
    pub stream: Vec<Box<dyn 'a + FnMut(Output)>>,
    /// Function, that filters invocations of `callback` and `stream`. Can be used to produce a
    /// more sparse output (such that there are not too many output points), or limit outputing
    /// values to a certain range, etc. It is a function, that is invoked on a state and returns
    /// bool.
    pub filter: Vec<StateFn<'a, N, bool>>,
    /// When it has unit type it does nothing, when it has type `usize`, it produces dense output:
    /// the event's "filter->stream->callback" sequence is triggered not on the current state, but
    /// on `subdivision` number of points of the current step in the state, making use of dense
    /// output feature of the state.
    pub subdivision: Option<usize>,
}

impl<'a, const N: usize, Output> Event<'a, N, Output> {
    /// Constructor that initializes [Event::callback] from [MutStateFn], and the rest by
    /// Default::default().
    pub fn new(callback: MutStateFn<'a, N, Output>) -> Self {
        Event {
            callback,
            stream: Default::default(),
            filter: Default::default(),
            subdivision: Default::default(),
        }
    }

    /// Constructor that initializes [Event::callback] using [MutStateFn::constant].
    pub fn constant(callback: impl 'a + FnMut() -> Output) -> Self {
        Event::new(MutStateFn::constant(callback))
    }
    /// Constructor that initializes [Event::callback] using [MutStateFn::time].
    pub fn time(callback: impl 'a + FnMut(f64) -> Output) -> Self {
        Event::new(MutStateFn::time(callback))
    }
    /// Constructor that initializes [Event::callback] using [MutStateFn::time_mut].
    pub fn time_mut(callback: impl 'a + FnMut(&mut f64) -> Output) -> Self {
        Event::new(MutStateFn::time_mut(callback))
    }
    /// Constructor that initializes [Event::callback] using [MutStateFn::ode].
    pub fn ode(callback: impl 'a + FnMut([f64; N]) -> Output) -> Self {
        Event::new(MutStateFn::ode(callback))
    }
    /// Constructor that initializes [Event::callback] using [MutStateFn::ode_mut].
    pub fn ode_mut(callback: impl 'a + FnMut(&mut [f64; N]) -> Output) -> Self {
        Event::new(MutStateFn::ode_mut(callback))
    }
    /// Constructor that initializes [Event::callback] using [MutStateFn::ode2].
    pub fn ode2(callback: impl 'a + FnMut(f64, [f64; N]) -> Output) -> Self {
        Event::new(MutStateFn::ode2(callback))
    }
    /// Constructor that initializes [Event::callback] using [MutStateFn::ode2_mut].
    pub fn ode2_mut(callback: impl 'a + FnMut(&mut f64, &mut [f64; N]) -> Output) -> Self {
        Event::new(MutStateFn::ode2_mut(callback))
    }
    /// Constructor that initializes [Event::callback] using [MutStateFn::dde].
    pub fn dde(
        callback: impl 'a + FnMut(f64, [f64; N], [Box<dyn '_ + StateCoordFnTrait>; N]) -> Output,
    ) -> Self {
        Event::new(MutStateFn::dde(callback))
    }

    /// Push a new function to [Event::stream].
    pub fn to(mut self, s: impl 'a + FnMut(Output)) -> Self {
        self.stream.push(Box::new(s));
        self
    }

    /// Push a new function to [Event::stream], that prints the output of [Event::callback] to the
    /// standard output using `println!`.
    pub fn to_std(self) -> Self
    where
        Output: std::fmt::Debug,
    {
        self.to(|value: Output| println!("{value:?}"))
    }

    /// Push a new function to [Event::stream], that opens file with a given filename and writes
    /// the output of [Event::callback] in that file.
    ///
    /// Panics, if file opening or writing fails.
    pub fn to_file(self, filename: &str) -> Self
    where
        Output: std::fmt::Debug,
    {
        use std::io::Write;
        let mut file = std::fs::File::create_buffered(filename).unwrap();
        self.to(move |value: Output| writeln!(&mut file, "{:?}", value).unwrap())
    }

    /// Push a new function to [Event::stream], that pushes the output of [Event::callback] to the
    /// provided vector.
    pub fn to_vec(self, vec: &'a mut Vec<Output>) -> Self {
        self.to(|value: Output| vec.push(value))
    }

    /// Push a new function to [Event::stream], that writes the output of [Event::callback] to the
    /// provided variable.
    pub fn to_var(self, value: &'a mut Output) -> Self {
        self.to(|v: Output| *value = v)
    }

    /// Push a new function to [Event::stream], that updates the range such that it represents the
    /// minimal closed interval that contains all the values outputed from [Event::callback] so
    /// far.
    ///
    /// The range is initialized to `(+oo .. -oo)`.
    pub fn to_float_range(self, range: &'a mut std::ops::Range<Output>) -> Self
    where
        Output: num_traits::Float,
    {
        *range = Output::infinity()..Output::neg_infinity();
        self.to(|v: Output| *range = range.start.min(v)..range.end.max(v))
    }

    /// Set [Event::subdivision] field to `Some(n)`, that tells the solver, that event needs to be
    /// triggered `n` times on the current step.
    pub fn subdivide(mut self, n: usize) -> Self {
        self.subdivision = Some(n);
        self
    }
}

impl<'a, const N: usize, Output: 'a> crate::Filter<'a, N> for Event<'a, N, Output> {
    fn filter(mut self, f: StateFn<'a, N, bool>) -> Self {
        self.filter.push(f);
        self
    }
}

impl<'a, const N: usize> Event<'a, N, [f64; N]> {
    /// Creates an event, the callback of which returns the coordinate vector of the state.
    ///
    /// A short-hand for `Event::new(MutStateFn::ode(|x| x))`.
    pub fn ode_state() -> Self {
        Event::new(MutStateFn::ode(|x| x))
    }
}

// // requires generic_const_exprs
// // can cause ICE if user doesn't write #![feature(generic_const_exprs)]
// impl<'a, const N: usize> Event<'a, N, [f64; N + 1]> {
//     pub fn ode2_state() -> Self {
//         Event::new(StateFnMut::ode2(|t: f64, x: [f64; N]| {
//             let mut res = [0.; N + 1];
//             res[0] = t;
//             for i in 1..=N {
//                 res[i] = x[i - 1];
//             }
//             res
//         }))
//     }
// }

impl<'a, const N: usize> Event<'a, N, (f64, [f64; N])> {
    /// Creates an event, the callback of which returns the time and coordinate vector of the state.
    ///
    /// A short-hand for `Event::new(MutStateFn::ode2(|t, x| (t, x)))`.
    pub fn ode2_state() -> Self {
        Event::new(MutStateFn::ode2(|t, x| (t, x)))
    }
}

impl<'a, const N: usize> Event<'a, N, ()> {
    /// Creates an event, that sets the time of the state to `f64::INFINITY`, effectively stopping the integration.
    ///
    /// A short-hand for `Event::time_mut(|t| *t = f64::INFINITY))`.
    pub fn stop_integration() -> Self {
        Event::time_mut(|t| {
            *t = f64::INFINITY;
        })
    }
}

impl<'a, const N: usize, Item, const M: usize> Event<'a, N, [Item; M]> {
    /// Like [Event::to_file] but only works with arrays, and prints array as a comma-separated values.
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
            write!(&mut file, "\n").unwrap();
        })
    }
    /// Like [Event::to_file] but only works with arrays, and prints array values separated by a separator, and adds the header line if it is provided.
    pub fn to_table(self, filename: &str, separator: &'a str, header: Option<&str>) -> Self
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
    pub fn to_vecs(self, vecs: [&'a mut Vec<Item>; M]) -> Self
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
    pub fn to_float_ranges(self, mut ranges: [&'a mut std::ops::Range<Item>; M]) -> Self
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

/// Creates a [crate::Event] from a closure.
///
/// `event!` allows `Event` to be defined with closures of different calling signatures,
/// being a replacement of some constructors of [crate::Event]:
///
/// ```rust
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
    () => {
        $crate::Event::constant(|| {})
    };
    (|| $expr:expr) => {
        $crate::Event::constant(|| $expr)
    };
    (|$t:ident| $expr:expr) => {
        $crate::Event::time(|$t| $expr)
    };
    (|[$($x:ident),+]| $expr:expr) => {
        $crate::Event::ode(|[$($x),+]| $expr)
    };
    (|$t:ident, [$($x:ident),+]| $expr:expr) => {
        $crate::Event::ode2(|$t, [$($x),+]| $expr)
    };
    (|$t:ident, [$($x:ident),+], [$($x_:ident),+]| $expr:expr) => {
        $crate::Event::dde(|$t, [$($x),+], [$($x_),+]| $expr)
    };
}

/// State-mutating counter-part of [event!].
///
/// `event_mut!` allows `Event` to be defined with closures of different calling signatures,
/// being a replacement of some constructors of [crate::Event]:
///
/// ```rust
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
    (|$t:ident| $expr:expr) => {
        $crate::Event::time_mut(|$t| $expr)
    };
    (|[$($x:ident),+]| $expr:expr) => {
        $crate::Event::ode_mut(|[$($x),+]| $expr)
    };
    (|$t:ident, [$($x:ident),+]| $expr:expr) => {
        $crate::Event::ode2_mut(|$t, [$($x),+]| $expr)
    };
}
