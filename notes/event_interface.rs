// Event callbacks specification:
// ```
// Event::Detected::new()
//     .by_zero_cross(|s| s.x)
//     .save(|s| s.t)
//     .set(|s| {s.x += 1.; s.y += eq.beta;});
// Event::Detected::new()
//     .by_zero_cross(|s| s.x)
//     .save(|s| s.t)
//     .set(|s| s.stop_integration());
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

fn main() {
    todo!()
}
