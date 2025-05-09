use crate::{State, StateFn, StateFnMut};

/// Event type holds several handlers that determine *what* happens when the event happens. Event
/// struct does not specify under what conditions event is triggered, the "when" part is determined
/// in [crate::solver::Solver] struct.
pub struct Event<'a, const N: usize = 0, Output = ()> {
    /// Function, which is called on a state. Its output is then fed to `stream`.
    pub callback: StateFnMut<'a, N, Output>,
    /// Function (or rather a collection of functions), which handles the output destination and
    /// formatting provided by `callback`. It takes a single argument: the return type of `callback`.
    pub stream: Vec<Box<dyn 'a + FnMut(Output)>>,
    /// Function, that filters invocations of `callback` and `stream`. Can be used to produce a
    /// more sparse output (such that there are not too many output points), or limit outputing
    /// values to a certain range, etc. It is a function, that is invoked on a state and returns
    /// bool.
    pub filter: Vec<StateFnMut<'a, N, bool>>,
    /// When it has unit type it does nothing, when it has type `usize`, it produces dense output:
    /// the event's "filter->stream->callback" sequence is triggered not on the current state, but
    /// on `subdivision` number of points of the current step in the state, making use of dense
    /// output feature of the state.
    pub subdivision: Option<usize>,
}

impl<'a, const N: usize, Output> Event<'a, N, Output> {
    pub fn new(callback: StateFnMut<'a, N, Output>) -> Self {
        Event {
            callback,
            stream: Vec::new(),
            filter: Vec::new(),
            subdivision: None,
        }
    }

    pub fn constant(callback: impl 'a + Fn() -> Output) -> Self {
        Event::new(StateFnMut::Constant(Box::new(callback)))
    }
    pub fn time(callback: impl 'a + Fn(f64) -> Output) -> Self {
        Event::new(StateFnMut::Time(Box::new(callback)))
    }
    pub fn ode(callback: impl 'a + Fn([f64; N]) -> Output) -> Self {
        Event::new(StateFnMut::ODE(Box::new(callback)))
    }
    pub fn ode2(callback: impl 'a + Fn(f64, [f64; N]) -> Output) -> Self {
        Event::new(StateFnMut::ODE2(Box::new(callback)))
    }

    pub fn to(mut self, s: impl 'a + FnMut(Output)) -> Self {
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

    pub fn to_vec(self, vec: &'a mut Vec<Output>) -> Self {
        self.to(|value: Output| vec.push(value))
    }

    /// The function that writes its argument to provided mutable variable is appended to `stream`
    /// field. The modified event is returned.
    pub fn to_var(self, value: &'a mut Output) -> Self {
        self.to(|v: Output| *value = v)
    }

    pub fn to_float_range(self, range: &'a mut std::ops::Range<Output>) -> Self
    where
        Output: num_traits::Float,
    {
        *range = Output::max_value()..Output::min_value();
        self.to(|v: Output| *range = range.start.min(v)..range.end.max(v))
    }

    pub fn filter_by(mut self, f: StateFnMut<'a, N, bool>) -> Self {
        self.filter.push(f);
        self
    }

    pub fn every(self, n: usize) -> Self {
        let mut counter = n - 1;
        self.filter_by(StateFnMut::Constant(Box::new(move || {
            counter += 1;
            counter -= n * (counter >= n) as usize;
            return counter == 0;
        })))
    }
    pub fn separated_by(self, delta: f64) -> Self {
        let mut last_trigger = f64::NEG_INFINITY;
        self.filter_by(StateFnMut::Time(Box::new(move |t| {
            if t >= last_trigger + delta {
                last_trigger = t;
                true
            } else {
                false
            }
        })))
    }

    pub fn in_range(self, interval: std::ops::Range<f64>) -> Self {
        self.filter_by(StateFnMut::Time(Box::new(move |t| interval.contains(&t))))
    }
    pub fn once(self) -> Self {
        let mut flag = true;
        self.filter_by(StateFnMut::Constant(Box::new(move || {
            if flag {
                flag = false;
                true
            } else {
                false
            }
        })))
    }

    pub fn take(self, n: usize) -> Self {
        let mut counter = 0;
        self.filter_by(StateFnMut::Constant(Box::new(move || {
            counter += 1;
            counter <= n
        })))
    }

    pub fn times(self, range: impl 'a + std::ops::RangeBounds<usize>) -> Self {
        let mut counter = 0;
        self.filter_by(StateFnMut::Constant(Box::new(move || {
            let ret = range.contains(&counter);
            counter += 1;
            ret
        })))
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
}

impl<'a, const N: usize, Item, const M: usize> Event<'a, N, [Item; M]> {
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

    pub fn to_vecs(self, vecs: [&'a mut Vec<Item>; M]) -> Self
    where
        Item: Copy,
    {
        self.to(move |value: [Item; M]| {
            for i in 0..N {
                vecs[i].push(value[i]);
            }
        })
    }

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
