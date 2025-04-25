For solution to be computed, several parts need to be specified:
- RHS of the equation
- The initial conditions
- Events, including ones that are responsible for saving
- Numerical method specifics, i.e. stepsize, rk_table, 

In different applications it can make sense to specify those parts together or separately.

- Equation object specifies the RHS, and events that modify the state of the equation. It can, but don't have to, specify the initial functions that can be used (one, or several), which can depend on parameters of the equation.

Possible syntax for macros: 
- equation: `x'' = -x;` or `(x', y') = (-y, x);`
- event: `x < 0 => x = -x;`
- initial conditions `{x: sigma; y: 2*t;}` or `(x,y): (1,t)`


<!-- What is not specified by the equation itself, must be specified in the solution function. -->


# State functions interface:

Under the hood, there is a state object, which holds the current and past states. It is the object on which the runge-kutta scheme is acting upon, making use of past states by means of interpolation, for delay differential equations. 

## Right hand side of the equation

For ordinary differential equations, the idea is to convert regular closures to state functions, supporting signatures like
`|t, [x, dx]| [dx, -x + t.sin()]`, 
`|[x,y,z]| [sigma * (y-x), x*(rho - z) - y, x*y - beta*z]`,
`|t| [(t*w).sin(), (t*w).cos()*w]`.


Internally, closure like `|t, [x, dx]| {...}` is called like `closure(state.t(), state.x())`.

The harder question, is how the user is supposed to use the delayed argument. The first instinct is, to make a closure, that accepts the `f64` for time, `[f64; N]` for immediate state, `[Fn(f64) -> f64; N]` for coordinate evaluation functions, and even aditional `[Fn(f64)-> f64; N]` for coordinate derivatives evaluation functions. So, the Hutchinson equation would be written as
`|t, [x], [x_]| [r * x * (1 - x_(t - tau)])`, and a neutral delay equation would be written as `|t, [x], _, [dx]| [-x + (1 + epsilon)*dx(t - T)]`.

For the Hutchinson equaiton, this closure will be called internally like `closure(state.t(), state.x(), [|t| state.eval::<0>(t)])`
