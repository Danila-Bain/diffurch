#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use diffurch::*;

fn main() {
    /* Parameters */
    let alpha = 10.;
    assert!(alpha >= 0.);
    let sigma = 0.1;
    assert!(sigma >= 0.);
    assert!(sigma <= 2.);
    const T: f64 = 1.;
    assert!(T > 0.);
    let v0 = 1.;
    assert!(v0 >= 0.);

    let eq =
        equation!(|t, [x, dx], [x_, __]| [dx, -sigma * dx - x - alpha * x_(t - T).clamp(-1., 1.)]);
    let ic = [1., -alpha * v0];
    let interval = 0. ..100. * T;

    let mut t = Vec::new();
    let mut x = Vec::new();

    Solver::new()
        .stepsize(0.1)
        .on_step(
            event!(|t, [x, _dx]| [t, x])
                .to_vecs([&mut t, &mut x])
                .to_std(),
        )
        .on_loc(Loc::sign(DDEStateFnMut(|t, _, [x, _]| x(t-T) - 1.)), event!()) // discontinuity
        .on_loc(Loc::sign(DDEStateFnMut(|t, _, [x, _]| x(t-T) + 1.)), event!()) // discontinuity
        .run(eq, ic, interval);

    let mut plot = pgfplots::axis::plot::Plot2D::new();
    plot.coordinates = (0..t.len()).map(|i| (t[i], x[i]).into()).collect();
    pgfplots::Picture::from(plot)
        .show_pdf(pgfplots::Engine::PdfLatex)
        .unwrap();
}
