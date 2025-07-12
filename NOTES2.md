# On the problem of creating a solver for differential equations.

The main problem I see is an infrustructural one, i.e. how to make an easy-to-use library while maintaining both the zero-costness of abstractions and variablilty of the available methods.

I find it hard to make a simple library (i.e. not complex, consisting of a few parts that can be considered in separation), because solving differential equations numerically seems to involve many coordinated parts.

In the case of neutral delay differential equations, which is the most general form of equations that this library attepts to solve, the following informatin must be provided by the user to achieve his goals:

- RHS of the equation
- discontinuities in the equation `*`
- delays for discontinuity tracking and history length estimation
    - delay function `*`
    - type (retarded or neutral) `*`
    - max_delay (`*` for constant delays)
- initial_condition
    - either constant, or function, or function + derivatives
    - initial discontinuities
- integration interval
- events:
    - event detection-location or integration stage: i.e. step, stop, propagate, etc
    - event filtering
    - event callback
        - output to outer variable
        - mutation of the state 
- stepsize controll




