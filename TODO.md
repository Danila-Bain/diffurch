Roughly in order of simplicity and necessity:

[ ] implement to_state_function for functions with signatures fn(f64) and fn()

[ ] add support for NDDEs

[ ] enlarge the scope of supported events
    [ ] on_substeps
    [ ] on_start
    [ ] on_stop
    [ ] on_rejected_step

[ ] add some built-in events
    [ ] stop integration

[ ] add filtering of the events, which is a closure that returns bool
    [ ] basic interface: Event::new(...).filter_by(...)
    [ ] every(n: usize) (doesn't need arguments)
    [ ] separated_by(delta: f64) (needs current time)

[ ] implement more output handlers for events
    [ ] to_csv
    [ ] to_table (specify the separators manually)

[ ] try to pipe the solution into a real-time plotter

[ ] add support for detected events
    [ ] interface like Solver::on(Detection, Event) for saving special values like zero crosses
    [ ] discrete variables for Equation to support hybrid and discontinuous DEs (hard?)

[ ] add support for events that change state

[ ] Figure out closure type inference to use one `new` in place of `ode`, `ode2`, `dde`, `ndde`.
