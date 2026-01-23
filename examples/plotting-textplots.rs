#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use std::f64::consts::PI;

use diffurch::*;

fn main() {
    let range = 0. ..PI * 3.5;

    let mut t_x = Vec::new();
    let mut t_dx = Vec::new();

    Solver::new()
        .equation(state_fn!(|[x, dx]| [dx, -4. * x]))
        .initial([0., 1.])
        .interval(range.clone())
        .rk(&rk::RK98)
        .stepsize(1.)
        .on_step(
            event!(|t, [x, dx]| [(t as f32, x as f32), (t as f32, dx as f32)])
                .subdivide(10)
                .to_vecs([&mut t_x, &mut t_dx]),
        )
        .run();

    use textplots::*;
    Chart::new(160, 80, range.start as f32, range.end as f32)
        .lineplot(&textplots::Shape::Lines(&t_x))
        .linecolorplot(
            &textplots::Shape::Lines(&t_dx),
            rgb::Rgb { r: 0, g: 0, b: 255 },
        )
        .display();
}
