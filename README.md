# Diffurch-rc

This is a just-started project that implements numerical methods for various kinds of differential equations, including ordinary, delay, and discontinuous differential equations.

Sketch usage:
```rust

equation!{
    Lorenz {
        x' = sigma * (y - x), 
        y' = x * (rho - z) - y, 
        z' = x * y - beta * z,
    }
}

equation!{
    BouncingBall {
        x'' = -g,
        x < 0 => x' = -k * x',
    }
}

struct HarmonicOscillator {
    w: f64,
}
impl Equation for HarmonicOscillator {
    fn rhs (&self, s: impl State<2>) {
        let [x, Dx] = &s.x;
        [
            Dx,
            -self.w.pow(2)*x,
        ] 
    }

    fn ic(&self, t: f64) {
        [(t*w).sin(), (t*w).cos()*w] 
    }
}


fn main() {

    let eq = Lorenz { sigma: 10., rho: 28., beta: 8./3. };

    let sol = eq.solution<rk4::Classic>(
                || [0.1, 0.2, 0.3], 
                0..10, 
                (Event::Step::new().save(|s| (s.t, s.x, s.y, s.z)), )
              );

    let (t, x, y, z) = eq.solution<rk98::RK98>(
                            |t| [sin(t), 0.2, 0.3], 
                            0..100,
                            (Event::Stop::new().save(|s| (s.t, s.x, s.y, s.z)), )
                        );

    let t = eq.solution<rk1::euler>(
        |t| [sin(t), 0.2, 0.3], 
0..100,
(Event::Stop::new().save(|s| (s.t, s.x, s.y, s.z)), )
);
    
}
```

State interface:
`
|s| (s.t, s.x, s.y, s.prev.t, s.prev.x, s.eval.x(s.t - 1), s.eval<1>.x(s.t));
state!{(t,x,y,prev.t,prev.x,x(t-1),x'(t))};
`

Event types:
```
enum Event {
    Detected,
    Step,
    RejectedStep,
    Call,
    Start,
    Stop,
}
```

Event callbacks specification:
```
Event::Detected::new()
    .by_zero_cross(|s| s.x)
    .save(|s| s.t)
    .set(|s| {s.x += 1.; s.y += eq.beta;});
Event::Detected::new()
    .by_zero_cross(|s| s.x)
    .save(|s| s.t)
    .set(|s| s.stop_integration());

// start counting after 5th
Event::RejectedStep::new()
    .set(|| rejected_counter++;)
    .times(5..); 

Event::Detected::new().by_zero_cross(|s| s.x).save(|s| s.t).range(0..5); // save only first 5 zero crosses
Event::Step::new().save(|s| s.t).which(|s| s.x > 0); // save steps only when x > 0;
Event::Step::new().save(|s| s.t).except(|s| s.x > 0); // don't trigger event callbacks when x > 0;
Event::Step::new().save(|s| s.t).until(|s| s.x > 0); // don't trigger event callbacks when x > 0;
```

Callbacks:
```
.save(|s| s.t)
.set(|s| s.x)
```

Filters:
```
.times(3..10)
.once()
.every(10)
.spaced_by(0.5)
.which(|s| s.x > 0)
.except(|s| s.x > 0)
.while(|s| s.x > 0)
.until(|s| s.x > 0)
```

change saving destination:
```rust
.to_return() // default
.to(&mut tuple of arrays)
.to_csv(&mut stream) ???
.to_hist(&mut histogram_handler)
```


# Structure of the project

There are several components, that contribute to the desired output: the data of the numerical solution provided in some form. From the perspective of the interface, it is
- The equation itself (determines the mathematical part)
    - right-hand side
    - intrusive events
    - initial conditions
- The solver (determines the technical realization part)
    - runge kutta scheme
    - stepsize controller
    - saving events


# Solver Interface:

```rust
let solver = Solver::new()
    .with_rk(rk::CLASSIC4)
    .with_stepsize(0.001)
    .add_step_event(
        Event::save(|s| {
            let t = s.t();      // this makes me upset
            let [x,y,z] = s.x(); // and this
            [t, x, y, z]
        })
        .spaced_by(0.1)
        .to_csv("datapoints.csv")?
    )
    .add_step_event(
        Event::save(|s| {
            let [x,y,z] = s.x();
            x*x + y*y + z*z
        })
        .every(100)
        .to_std()
    )
```

# Event Interface:

```rust
let event = Event::save(|s| {
        let [x, dx] = s.x();
        x*x + dx*dx
        })


let event = event_save_xyz!{[t, x, y, z]};
let event = event_save_x1!{[t, ]};
```
