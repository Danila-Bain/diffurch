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
