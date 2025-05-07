In each section, roughly in order of simplicity and necessity:

# Equation types support

[x] implement to_state_function for functions with signatures fn(f64) and fn()

[x] enlarge the scope of supported events
    [x] subdivide option for Events
    [x] on_start
    [x] on_stop
    [ ] on_rejected_step

[ ] add support for NDDEs

<!-- Syntax: (|t, [x], [x_], [dx_]| [x_(t-1) - dx_(t-2)]) or (|t, [x], [x_]| [x_(t-1) - x_.1(t-1)]) -->
<!-- or even (|t, [x]| [x() + x(t-1) - x.1(t-2)]) -->

[ ] add support for events that change state

[ ] add support for detected events
    [ ] interface like Solver::on(Detection, Event) for saving special values like zero crosses
    [ ] different detection methods: Sign and DerivativeSign (like in WM)
    [ ] different location methods: StepBegin, StepEnd, Lerp, Brent, Bisection

[ ] delay propagated events
    [ ] treat initial discontinuities for DDEs and NDDEs

[ ] support hybrid and discontinuous DEs (hard?)
    [ ] automatic support for functions `signum, clamp, abs, floor, ceil, round, fract, % (mod)`
    [ ] make a way to define piecewise right-hand sides.

[ ] delta functions


# Event Output


[x] add filtering of the events, which is a closure that returns bool
    [x] filter_by(...), like filter_by(|t| t >= 10.)
    [x] every(n: usize) (doesn't need arguments)
    [x] separated_by(delta: f64) (needs current time)
    [x] once, first, times
    [ ] .no_init() to exclude points at the initial points 

[x] make filtering be iterative, such that .every(2).every(2) is equivallent to every(4), make the order of their calls correct (it is reversed at the time)

[ ] implement more output handlers for events
    [ ] to_file (output as in to_std)
    [ ] to_csv (same as to_file, but formatted)
    [ ] to_table (specify the separators manually)
    [ ] to_hist (basic datashading), probably make a histogram class, that can grow for values outside of the current range
    [ ] to_plot(window: f64, realtime: true) (for crude accessible realtime plotting)

[ ] for text output handlers, make the destination and formatting orthogonal, like
    [ ] .to_file_format("filename", Format::CSV)
    [ ] .to_std_format(Format::Plain)

[ ] add some built-in events
    [ ] stop integration

[ ] ability to turn off an event in case of other event

[ ] return the whole state

# Differentiation

[ ] support extending closures with the method like .with_derivative(||...), which produces and object that implements Fn to call the inital closure, and a method .d to invoke a derivative
[ ] use that for a polynomial macro

# Solver

[ ] adaptive step size controller
    [ ] PI stepsize controll

# Validation

[ ] provide a function, that solves the equation with different stepsizes, estimating the error of the method

[ ] rk stability regions plot

# Testing

[ ] plain integral calculation

[ ] try to pipe the solution into a real-time plotter


# Detecting events

Naming: `on_root(|t: f64, [x, dx]: [f64; 2]| x, Event::new(...)...)`

[ ] on_root
[ ] on_above_root, on_below_root (naming should change)
[ ] on_period

[ ] delay propagated events

# API convenience and macros

[ ] Constructor functions for functions with arguments () - Equation::const or (f64,) - Equation::t

[ ] Figure out closure type inference to use one `new` in place of `ode`, `ode2`, `dde`, `ndde`.

[ ] equation! macro, which also saves the string representation of the equation

[ ] allow .times(1..) for excluding the first step

[ ] allow the initial function to have no arguments

# Internal optimizations

[?] Make the streams in events return (), such that to_state_func and to_state_eval_func are not weird for subdivided callbacks
[ ] For polynomials, multiplication by zero and adding zero can be optimized away, but it is not done by the compiler.

# Direction of further development

[ ] Automatic differentiation numerical types (for root finding?)
[ ] Support for numbers with higher precision
