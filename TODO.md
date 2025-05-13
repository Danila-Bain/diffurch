In each section, roughly in order of simplicity and necessity:

# Equation types support

[x] implement to_state_function for functions with signatures fn(f64) and fn()

[x] enlarge the scope of supported events
    [x] subdivide option for Events
    [x] on_start
    [x] on_stop
    [ ] on_rejected_step

[x] add support for NDDEs

[x] add support for events that change state

[!] add support for detected events
    [ ] interface like Solver::on(Detection, Event) for saving special values like zero crosses
    [ ] different detection methods: Sign and DerivativeSign (like in WM)
    [ ] on_root
    [ ] on_above_root, on_below_root (naming should change)
    [ ] on_period (trigger event periodically)

[ ] delay propagated events

[ ] different location methods: 
    [x] StepBegin, 
    [x] StepEnd, 
    [x] Lerp, 
    [ ] Brent, 
    [x] Bisection

[ ] filtering for event location

[ ] different events attached to one location. For example: stop integration after 1000th located event, which also mutates the state

[ ] delay propagated events
    [ ] treat initial discontinuities for DDEs and NDDEs

[ ] support hybrid and discontinuous DEs (hard?)
    [ ] automatic support for functions `signum, clamp, abs, floor, ceil, round, fract, % (mod)`
    [ ] make a way to define piecewise right-hand sides.

[ ] delta functions


# Event Output

[ ] add empty event like Event::none()

[ ] add support for events, that haave access, to the data from the previous events. it's unclear yet should it be in event calback or a stream. Anyway get inspired by Iter::scan or fold or iter_map_windows.

[x] add filtering of the events, which is a closure that returns bool
    [x] filter(...), like filter(|t| t >= 10.)
    [x] every(n: usize) (doesn't need arguments)
    [x] separated_by(delta: f64) (needs current time)
    [x] once, times
    [ ] skip
    [ ] take
    [ ] .while(|s| s.x > 0)
    [ ] .until(|s| s.x > 0)
    [ ] rename "every" -> "step_by" to make it simillar to iterators api?

[x] make filtering be iterative, such that .every(2).every(2) is equivallent to every(4), make the order of their calls correct (it is reversed at the time)

[ ] implement more output handlers for events
    [x] to_file (output as in to_std)
    [x] to_csv (same as to_file, but formatted)
    [x] to_table (specify the separators manually)
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

[x] support extending closures with the method like .with_derivative(||...), which produces and object that implements Fn to call the inital closure, and a method .d to invoke a derivative
[x] use that for a polynomial macro

# Solver

[ ] adaptive step size controller
    [ ] PI stepsize controll

# Validation

[ ] provide a function, that solves the equation with different stepsizes, estimating the error of the method

[ ] rk stability regions plot

# Testing

[ ] plain integral calculation

[ ] try to pipe the solution into a real-time plotter



# API convenience and macros

[ ] Constructor functions for functions with arguments () - Equation::const or (f64,) - Equation::t

[ ] Figure out closure type inference to use one `new` in place of `ode`, `ode2`, `dde`, `ndde`.

[ ] equation! macro, which also saves the string representation of the equation

[x] allow .times(1..) for excluding the first step

[ ] allow the initial function to have no arguments

# Internal optimizations

[?] Make the streams in events return (), such that to_state_func and to_state_eval_func are not weird for subdivided callbacks
[ ] For polynomials, multiplication by zero and adding zero can be optimized away, but it is not done by the compiler.

# Direction of further development

[ ] Automatic differentiation numerical types (for root finding?)
[ ] Support for numbers with higher precision
