# Diffurch-rc

This is a developing project that implements numerical methods for various kinds of differential equations, including ordinary, (neutral) delay, discontinuous, hybrid differential equations, and anything in between.


# Structure of the project

There are several components, that contribute to the production of desired output, which is the data of the numerical solution provided in some form. 

From the perspective of the interface, it is
- The equation and its initial conditions (determines the mathematical part)
    - right-hand side of the equation
    - intrusive events (switches, jumps, etc)
    - initial time
    - intial value or initial function (for delay differential equations)
- The solver (determines the technical realization part)
    - runge kutta scheme
    - stepsize controller
    - saving events

