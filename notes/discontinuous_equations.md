`let eq = Equation::ode(|[x, dx]| [dx, x.signum()]).discontinuity(|[x, dx]| x)`

Is it possible to automatically infer the need of event (with macros --- maybe, for known functions)

    Known non-smooth functions:
    - signum, abs
    - clamp, min, max
    - floor, ceil, trunc, round, fract

    Using discontinuous functions as they are is knowingly problematic, because the step containing the discontinuity has poor precision, and getting the discontinuity position from the interpolation leads to erroneous result. Instead, we want to fix the smooth branch of the function, and calculate the step, for example, as if signum function does not change, even though its argument changes sign. Then, after we detected changed sign, we find the discontinuity point from interpolant, and continue integration from that point, with switched value of sign.

    Who is going to be responsible for storing the value of a discrete variable, and update it as needed? 

    naive approach, which borrows s mutably and immutably:

    `let mut s; let eq = Equation::ode(|[x, dx]| [dx, s]).add_event(Detection::ode(|[x,dx]| x), Event::new(|| s = -s))`

    ofc it is possible, to wrap s with mutex or something, but that would complicate the api



    we also can introduce a discrete variable directly, if mutating events are supported:

    `let eq = Equation::ode(|[x, dx, sign_x]| [dx, -sign_x, 0]).detect(|[x,dx,sign_x]| x, Event::ode(|[x,dx,sign_x]| {sign_x = -sign_x}));`

    but that also would require adjusting the initial conditions


    we can support general piecewise definition for equations:

    ```
    let eq = Equation::piecewise_ode(
            |i| move |[x, dx]|
            match i { // match is inside our function, such that two functions can have the same type and indexed normally
            0 => [dx, -1.],
            _ => [dx, 1.]
            },
            Condition::signum(|[x, dx]| x)
            )
    ```


    perfect world:
    ```
    let eq = equation!(|[x, dx]|
            x < 0 => [dx, -1.],
            x > 0 => [dx, 1.]
            )
    ```
