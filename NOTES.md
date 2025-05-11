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

```rust

// Possible syntax for macros:
// - equation: `x'' = -x;` or `(x', y') = (-y, x);`
// - event: `x < 0 => x = -x;`
// - initial conditions `{x: sigma; y: 2*t;}` or `(x,y): (1,t)`

// equation!{
//     Lorenz {
//         x' = sigma * (y - x),
//         y' = x * (rho - z) - y,
//         z' = x * y - beta * z,
//     }
// }

// equation!{
//     BouncingBall {
//         x'' = -g,
//         x < 0 => x' = -k * x',
//     }
// }

fn main() {
    todo!()
}

//
// // start counting after 5th
// Event::RejectedStep::new()
//     .set(|| rejected_counter++;)
//     .times(5..);
//
// Event::Detected::new().by_zero_cross(|s| s.x).save(|s| s.t).range(0..5); // save only first 5 zero crosses
// Event::Step::new().save(|s| s.t).which(|s| s.x > 0); // save steps only when x > 0;
// Event::Step::new().save(|s| s.t).except(|s| s.x > 0); // don't trigger event callbacks when x > 0;
// Event::Step::new().save(|s| s.t).until(|s| s.x > 0); // don't trigger event callbacks when x > 0;
// ```
//
// Callbacks:
// ```
// .save(|s| s.t)
// .set(|s| s.x)
// ```
//
// Filters:
// ```
// .times(3..10)
// .once()
// .every(10)
// .spaced_by(0.5)
// .which(|s| s.x > 0)
// .except(|s| s.x > 0)
// .while(|s| s.x > 0)
// .until(|s| s.x > 0)
// ```
//
// change saving destination:
// ```rust
// .to_return() // default
// .to(&mut tuple of arrays)
// .to_csv(&mut stream) ???
// .to_hist(&mut histogram_handler)
// ```

// # Solver Interface:
//
// ```rust
// let solver = Solver::new()
//     .with_rk(rk::CLASSIC4)
//     .with_stepsize(0.001)
//     .add_step_event(
//         Event::save(|t, [x,y,z]| { [t, x, y, z] })
//         .spaced_by(0.1)
//         .to_csv("datapoints.csv")?
//     )
//     .add_step_event(|[x,y,z]| x*x + y*y + z*z)
//         .every(100)
//         .to_std()
//     )
// ```
//
// # Event Interface:
//
// ```rust
// let event = Event::save(|[x, dx]| x*x + dx*dx)
//
// let event = Event::save(|t, [x], [x_]| {
//     [t, x, x_(t - tau)]
// }) // doubious
//
// let event = Event::new().save(|t, [x,y,z]| [t, x, y, z]);
// let event = Event::new().save(|t, [x, dx]| [t, x]);
// ```
//


```
