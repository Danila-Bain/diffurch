#set raw(lang: "rust")

= Usage example

```
use diffurch::*;

fn main() {

    let sigma = 10.;
    let rho = 28.;
    let beta = 8. / 3.;

    let mut t = Vec::new();
    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut z = Vec::new();

    Solver::new()
        .equation(ODE(|[x, y, z]| [
            sigma * (y - x),
            x * (rho - z) - y,
            x * y - beta * z
        ]))
        .initial([1., 2., 20.])
        .interval(0. ..100.)
        .on_step(
            diffurch::event!(|t, [x, y, z]| [t, x, y, z])
                .subdivide(20) // dense output: save 4 points per step for smoother plot
                .to_vecs([&mut t, &mut x, &mut y, &mut z]), // save values to individual `Vec<f64>`s
        )
        .run();


}
```
= Notation?

Hard to implement, stiff in practice:
```
Solver::new()
.equation(ODE(|[x, y, z]| [
  sigma * (y - x),
  x * (rho - z) - y,
  x * y - beta * z
]))
```
More verbouse, easier to implement and maintain. 
Unclear how to support `eval_prev` and `eval_at`.
```
Solver::new()
.equation(StateFn::new(|state| {
  let [x, y, z] = state.x();
  [
    sigma * (y - x),
    x * (rho - z) - y,
    x * y - beta * z
  ]
}))
.variation(State::new(|state| {
  let [x,   y,  z] = state.x();
  let [dx, dy, dz] = state.y();
  [
    sigma * (dy - dx),
    dx * (rho - z) - x * dz - dy,
    dx * y + x * dy - beta * dz,
  ]
}))
```
= State aspects

- What:
  - `t`: time
  - `x`: position(s)
  - `v`: velocity(s)
  - `y`: variation(s)
  - `disco`: discontinuity
- when:
  - now
  - `_prev`: on previous step
  - `_seq`: history
  - `_eval`: interpolated history
  - `_init`: initial

= State functions functionality usage
- `eval`: RK step, event callback
- `eval_mut`: mut event callback
- `eval_at`: event subdivision filtering and callback
- `eval_prev`: event detection

= Dependencies

ODEs do not require solution or discontinuity history


