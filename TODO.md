In each section, roughly in order of simplicity and necessity:


# Event Location
[ ] Brent root finding method for event location
[ ] DerivativeSign event detection
[ ] Periodic events
[ ] Add equation events and Equation::disco method

[ ] delay propagated events
[ ] delay propagated events for initial discontinuities for DDEs and NDDEs

[ ] support hybrid and discontinuous DEs
    [ ] automatic support for functions `signum, clamp, abs, floor, ceil, round, fract, % (mod)`
    [ ] make a way to define piecewise right-hand sides.
[ ] delta functions

# Event callbacks

[ ] add empty event like Event::none()
[ ] add support for events, that have access, to the data from the previous instances of event triggering. it's unclear yet should it be in event calback or a stream. Anyway get inspired by Iter::scan or fold or iter_map_windows.

# Filtering

[ ] .while(|s| s.x > 0)
[ ] .until(|s| s.x > 0)
[ ] rename "every" -> "step_by" to make it simillar to iterators api?

[ ] ability to turn off an event in case of other event
[ ] combine events with std::ops::Add? like Event::... + Event::stop_integration()
[ ] definition for StateFnMut::DDEMut

# Event streams

[ ] to_hist (basic datashading), probably make a histogram class, that can grow for values outside of the current range
[ ] to_plot(window: f64, realtime: true) (for crude accessible realtime plotting)

[ ] for text output handlers, make the destination and formatting orthogonal, like
    [ ] .to_file_format("filename", Format::CSV)
    [ ] .to_std_format(Format::Plain)

[ ] output the whole state as an interpolation function
[ ] to_return(): probably more advanced generic are needed than rust currently has
[ ] real-time plotting for interactive researching

# StepsizeController

[ ] adaptive step size controller
[ ] PI stepsize controll
[ ] on_reject events

# Validation

[ ] provide a function, that solves the equation with different stepsizes, estimating the error of the method

[ ] rk stability regions plot

# More Runge-Kutta schemes
Just overkill it

# Internal optimizations
[ ] For polynomials, multiplication by zero and adding zero can be optimized away, but it is not done by the compiler.

# Advanced number types
[ ] Automatic differentiation numerical types (for root finding?)
[ ] Support for numbers with higher precision
[ ] Interval arithmetic

# Documentation
Add more examples.

Add tutorials, showcasing the workflow for paper-quality plotting and for interactive plotting.
