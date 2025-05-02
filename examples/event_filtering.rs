use diffurch::polynomial;
use diffurch::{Equation, Event, Solver, rk};
fn main() {
    let eq = Equation::new(|t: f64| [polynomial![t => 1.,-2.,3.]]);
    let solution = |t: f64| [polynomial![t => 0.,1.,-1.,1.]];
    let interval = 0. ..50.;

    let f = |t: f64, [x]: [f64; 1]| (t, x);

    let mut p0 = Vec::new();
    let mut p1 = Vec::new();
    let mut p2 = Vec::new();
    let mut p3 = Vec::new();
    let mut p4 = Vec::new();
    let mut p5 = Vec::new();
    let mut p6 = Vec::new();
    let mut p7 = Vec::new();
    let mut p8 = Vec::new();
    let mut p9 = Vec::new();
    let mut p10 = Vec::new();
    let mut p11 = (0., 0.);
    let mut p12 = (0., 0.);
    let mut p13 = Vec::new();
    let mut p14 = Vec::new();
    // let mut p15 = Vec::new();

    let event0 = Event::ode2(f).to_vec(&mut p0);
    let event1 = Event::ode2(f).to_vec(&mut p1).every(2);
    let event2 = Event::ode2(f).to_vec(&mut p2).every(2).every(2);
    let event3 = Event::ode2(f).to_vec(&mut p3).separated_by(4.);
    let event4 = Event::ode2(f).to_vec(&mut p4).in_range(10. ..20.);
    let event5 = Event::ode2(f).to_vec(&mut p5).in_range(10. ..20.).every(3);
    let event6 = Event::ode2(f).to_vec(&mut p6).times(5..10);
    let event7 = Event::ode2(f).to_vec(&mut p7).times(0..usize::MAX);
    let event8 = Event::ode2(f).to_vec(&mut p8).times(0..5);
    let event9 = Event::ode2(f).to_vec(&mut p9).take(5);
    let event10 = Event::ode2(f).to_vec(&mut p10).filter_by(|[x] : [f64; 1]| x > 100.);
    let event11 = Event::ode2(f).to_var(&mut p11).filter_by(|[x] : [f64; 1]| x > 100.);
    let event12 = Event::ode2(f).to_var(&mut p12).filter_by(|[x] : [f64; 1]| x > 100.).once();
    let event13 = Event::ode2(f).to_vec(&mut p13).subdivide(2);
    let event14 = Event::ode2(f).to_vec(&mut p14).subdivide(4);

    Solver::new()
        .rk(&rk::RK98) // it is exact for polynomials up to 8th or 9th order
        .stepsize(1.)
        .on_step(event0)
        .on_step(event1)
        .on_step(event2)
        .on_step(event3)
        .on_step(event4)
        .on_step(event5)
        .on_step(event6)
        .on_step(event7)
        .on_step(event8)
        .on_step(event9)
        .on_step(event10)
        .on_step(event11)
        .on_step(event12)
        .on_step(event13)
        .on_step(event14)
        .run(eq, solution, interval);

    let f_i = |i| (i as f64, solution(i as f64)[0]);
    let f_t = |t| (t, solution(t)[0]);

    assert_eq!(p0, (0..=50).map(f_i).collect::<Vec<_>>());
    assert_eq!(p1, (0..=50).step_by(2).map(f_i).collect::<Vec<_>>());
    assert_eq!(p2, (0..=50).step_by(4).map(f_i).collect::<Vec<_>>());
    assert_eq!(p3, p2);
    assert_eq!(p4, (10..20).map(f_i).collect::<Vec<_>>());
    assert_eq!(p5, (10..20).step_by(3).map(f_i).collect::<Vec<_>>());
    assert_eq!(p6, (5..10).map(f_i).collect::<Vec<_>>());
    assert_eq!(p7, p0);
    assert_eq!(p8, (0..5).map(f_i).collect::<Vec<_>>());
    assert_eq!(p9, p8);

    assert!(solution(4.)[0] <= 100.);
    assert!(solution(5.)[0] > 100.);
    assert_eq!(p10, (5..=50).map(f_i).collect::<Vec<_>>());
    assert_eq!(p11, (50., solution(50.)[0]));
    assert_eq!(p12, (5., solution(5.)[0]));

    assert!((0..=100).map(|i| {let t = i as f64 * 0.5; (p13[i+1].1 - f_t(t).1).abs()}).fold(0f64, |acc, x| acc.max(x)) < 1e-7);
    assert!((0..=200).map(|i| {let t = i as f64 * 0.25; (p14[i+3].1 - f_t(t).1).abs()}).fold(0f64, |acc, x| acc.max(x)) < 1e-7);
}
