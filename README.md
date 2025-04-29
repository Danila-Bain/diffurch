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
