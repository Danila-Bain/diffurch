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
