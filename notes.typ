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

= State Interface Object

curr: 
  // needed everywhere
  t() -> t_curr
  x() -> x_curr
  dx() -> k_curr
  x_at(t) -> eval::<0>(t)
  dx_at(t) -> eval::<1>(t)
  x_prev() -> x_prev

  // events only
  t_mut() -> &mut t_curr
  x_mut() -> &mut x_curr
  tx_mut() -> (&mut t_curr, &mut x_curr)

prev: 
  // event detection
  t() -> t_prev
  x() -> x_prev
  dx() -> k_prev
  x_at(t) -> eval::<0>(t)
  dx_at(t) -> eval::<1>(t)
  x_prev() -> x_prev or k_deque.last()

at(t_this):
  // event location and 
  t() -> t_this
  x() -> eval::<0>(t_this)
  dx() -> eval::<1>(t_this)
  x_at(t) -> eval::<0>(t)
  dx_at(t) -> eval::<1>(t)
  x_prev() -> x_deque.at(get_interval())


to implement this with lambdas statically, they need to be generic

= Interface via trait implementation?

```rust
struct LorenzSystem {
    sigma: f64,
    rho: f64,
    beta: f64,
};
impl DifferentialEquation<f64, 3> for LorenzSystem {
    fn rhs(&mut self, state: &impl State<f64, 3>) -> [f64; 3] {
        let Self{sigma, rho, beta, ..} = self;
        let [x, y, z] = state.x();
        [
          sigma * (y - x),   //
          x * (rho - z) - y, //
          x * y - beta * z, //
        ]
    }

    fn jacobian(&mut self, state: &impl State<f64, 3>) -> Matrix3<f64> {
        let Self{sigma, rho, beta, ..} = self;
        let [x, y, z] = state.x();
        matrix![
            -sigma, sigma, 0.;
            rho - z, -1., -x;
            y, x, -beta;
        ];
    }
}

struct Bounce;
impl Event<f64, 3> for Bounce {
    type Locator = Bisection;
    type Detector = Sign;

    fn event(&mut self, state: &impl State<f64, 3>) -> f64 {
        state.x()[0]
    }
    // fn jacobian(&mut self, state: &impl State<f64, 3>) -> [f64, 3] {};

    fn action(&mut self, state: &mut impl State<f64, 3>) {
        *state.x_mut()[0] *= -1.;
        println!("Bounced at {}!", state.t());
    }
}

struct EvRefOutput<'a>{
  tt: &'a mut Vec<f64> 
};
impl Event<f64, 3> for EvRefOutput {
    type Locator = Bisection;
    type Detector = Sign; // AboveZero, BelowZero, Start, Step, Stop

    fn event(&mut self, state: &impl State<f64, 3>) -> f64 {
        state.x()[0]
    }
    // fn jacobian(&mut self, state: &impl State<f64, 3>) -> [f64, 3] {};

    fn action(&mut self, state: &mut impl State<f64, 3>) {
        self.tt.push(state.t());
    }
}

struct EvOutput{
  tt: Vec<f64>,
  xx: Vec<f64>,
  yy: Vec<f64>,
  zz: Vec<f64>,
}
impl Event<f64, 3> for EvOutput {
    type Detector: Step;
    fn action(&mut self, state: &mut impl State<f64, 3>) {
        self.tt.push(state.t());
        let [x, y, z] = state.x();
        self.xx.push(x);
        self.yy.push(y);
        self.zz.push(z);
    }
}


let solver = Solver::new(LorenzSystem {sigma: 10., rho: 28., beta: 8./3.})
  .method(erk::euler().stepsize(0.001))
  .interval(0. ..10.)
  .initial_value([2.,3.,4.])
  // .initial_function()
  // .initial_derivative()
  .event(Bounce)
  .event(EvRefOutput{tt: &mut tt})
  .event(EvOutput)
  ;

let (t, [x, y, z]) = solver.final();
let (tt, xx) = solver.solution();
let (tt, xx, events) = solver.solution();
let EvOutput{tt,xx,yy,zz} = events.get();

for state in solver.into_iter().filter(|state| state.t() > 100.) {
    let t = state.t();
    let [x, y, z] = state.x();
    tt.push(t);
    xx.push(t);
    yy.push(t);
    zz.push(t);
}

solver.even(0.1).into_inter()
  .map(|state| (state.t(), state.x()))
  .for_each(|t, [x,y,z]| {tt.push(t); xx.push(x); yy.push(y); zz.push(z);});

for (state, events) in solver.dense(10).events().into_iter() {
  match event {
    Event::Step() => {
      let t = state.t();
      let [x, y, z] = state.x();
      tt.push(t);
      xx.push(t);
      yy.push(t);
      zz.push(t);
    }
    Event::Discontinuity(order) => {},
    Event::Located(Bounce) => {
      let [x, y, z] = state.x_mut();
      *x = -(*x);
    }
    _ => {};
  }
}
```


