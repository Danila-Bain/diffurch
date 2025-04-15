For solution to be computed, several parts need to be specified:
- RHS of the equation
- The initial conditions
- Events, including ones that are responsible for saving
- Numerical method specifics, i.e. stepsize, rk_table, 

In different applications it can make sense to specify those parts together or separately.

- Equation object specifies the RHS, and events that modify the state of the equation. It can, but don't have to, specify the initial functions that can be used (one, or several), which can depend on parameters of the equation.

Possible syntax: 
- equation: `x'' = -x;` or `(x', y') = (-y, x);`
- event: `x < 0 => x = -x;`
- initial conditions `{x: sigma; y: 2*t;}` or `(x,y): (1,t)`


What is not specified by the equation itself, must be specified in the solution function.


