use diffurch::equation::Equation;
use diffurch::event::Event;
use diffurch::rk;
use diffurch::solver::Solver;

fn main() {
    // let theta = 0.5f64;
    let k = 1f64;
    let tau = 1f64;

    let a = k / (k * tau).tan();
    let b = -k / (k * tau).sin();

    let equation = Equation::dde(|t, [x], [x_]| [a * x + b * x_(t - tau)]);
    let ic = move |t: f64| [(k * t).sin()];
    let range = 0. .. 1.1;

    let mut t = Vec::new();
    let mut x = Vec::new();

    Solver::new()
        .stepsize(0.01)
        .rk(&rk::DP544)
        .on_step(Event::new(|t: f64, [x]: [f64; 1]| [t, x]).to_vecs([&mut t, &mut x]))
        .on_step(Event::new(|t: f64, [x]: [f64; 1]| [t, x, x - ic(t)[0]]).to_std())
        .run(equation, ic, range);

    
    //
    //
    // let mut plot = pgfplots::axis::plot::Plot2D::new();
    // plot.coordinates = (0..t.len()).map(|i| (t[i], x[i]).into()).collect();
    // pgfplots::Picture::from(plot)
    //     .show_pdf(pgfplots::Engine::PdfLatex)
    //     .unwrap();
}

/*
 
state function that we get after to_state_function application


our ingredients:

    s: &State,

    f: Fn(f64, [f64; N], [@closure; N]) -> Ret

our result:

    f(s.t, s.x, [|t| s.eval_i(t, 0)])


intermediate object, a function:
    |s: &State| f(s.t, s.x, [|t| s.eval_i(t, 0)])

for each s, the [|t| ...] is constructed, it captures the reference to s, and index by value

 */
