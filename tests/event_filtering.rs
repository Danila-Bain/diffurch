#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use diffurch::*;

#[test]
fn main() {
    let eq = equation!(|t| [polynomial![t => 1.,-2.,3.]]);
    let solution = |t: f64| [polynomial![t => 0.,1.,-1.,1.]];
    let ic = |t: f64| [polynomial![t => 0.,1.,-1.,1.]];
    let interval = 0. ..50.;

    let f = state_fn!(|t, [x]| (t, x));

    let mut p = Vec::new();
    let event = Event::new(f).to_vec(&mut p);

    let mut p_e2 = Vec::new();
    let event_e2 = Event::new(f).to_vec(&mut p_e2).every(2);

    let mut p_e2_e2 = Vec::new();
    let event_e2_e2 = Event::new(f).to_vec(&mut p_e2_e2).every(2).every(2);

    let mut p_s4 = Vec::new();
    let event_s4 = Event::new(f).to_vec(&mut p_s4).separated_by(4.);

    let mut p_r = Vec::new();
    let event_r = Event::new(f).to_vec(&mut p_r).in_range(10. ..20.);

    let mut p_r_e3 = Vec::new();
    let event_r_e3 = Event::new(f)
        .to_vec(&mut p_r_e3)
        .in_range(10. ..20.)
        .every(3);

    let mut p_e3_r = Vec::new();
    let event_e3_r = Event::new(f)
        .to_vec(&mut p_e3_r)
        .every(3)
        .in_range(10. ..20.);

    let mut p_t_5_10 = Vec::new();
    let event_t_5_10 = Event::new(f).to_vec(&mut p_t_5_10).times(5..10);

    let mut p_t_0_oo = Vec::new();
    let event_t_0_oo = Event::new(f).to_vec(&mut p_t_0_oo).times(0..usize::MAX);

    let mut p_t_0_5 = Vec::new();
    let event_t_0_5 = Event::new(f).to_vec(&mut p_t_0_5).times(0..5);

    let mut p_take_5 = Vec::new();
    let event_take_5 = Event::new(f).to_vec(&mut p_take_5).take(5);

    let mut p_g_100 = Vec::new();
    let event_g_100 = Event::new(f)
        .to_vec(&mut p_g_100)
        .filter(state_fn!(|[x]| x > 100.));

    let mut p_g_100_last = (0., 0.);
    let event_g_100_last = Event::new(f)
        .to_var(&mut p_g_100_last)
        .filter(state_fn!(|[x]| x > 100.));

    let mut p_g_100_first = (0., 0.);
    let event_g_100_first = Event::new(f)
        .to_var(&mut p_g_100_first)
        .filter(state_fn!(|[x]| x > 100.))
        .once();

    let mut p_sub2 = Vec::new();
    let event_sub2 = Event::new(f).to_vec(&mut p_sub2).subdivide(2);

    let mut p_sub4 = Vec::new();
    let event_sub4 = Event::new(f).to_vec(&mut p_sub4).subdivide(4);

    Solver::new()
        .rk(&rk::RK98) // it is exact for polynomials up to 8th or 9th order
        .stepsize(1.)
        .on_step(event)
        .on_step(event_e2)
        .on_step(event_e2_e2)
        .on_step(event_s4)
        .on_step(event_r)
        .on_step(event_e3_r)
        .on_step(event_r_e3)
        .on_step(event_t_5_10)
        .on_step(event_t_0_oo)
        .on_step(event_t_0_5)
        .on_step(event_take_5)
        .on_step(event_g_100)
        .on_step(event_g_100_last)
        .on_step(event_g_100_first)
        .on_step(event_sub2)
        .on_step(event_sub4)
        .run(eq, ic, interval);

    let f_i = |i| (i as f64, solution(i as f64)[0]);
    let f_t = |t| (t, solution(t)[0]);

    assert_eq!(p, (0..=50).map(f_i).collect::<Vec<_>>());
    assert_eq!(p_e2, (0..=50).step_by(2).map(f_i).collect::<Vec<_>>());
    assert_eq!(p_e2_e2, (0..=50).step_by(4).map(f_i).collect::<Vec<_>>());
    assert_eq!(p_s4, p_e2_e2);
    assert_eq!(p_r, (10..20).map(f_i).collect::<Vec<_>>());
    assert_eq!(p_r_e3, (10..20).step_by(3).map(f_i).collect::<Vec<_>>());
    assert_eq!(p_e3_r, (12..20).step_by(3).map(f_i).collect::<Vec<_>>());
    assert_eq!(p_t_5_10, (5..10).map(f_i).collect::<Vec<_>>());
    assert_eq!(p_t_0_oo, p);
    assert_eq!(p_t_0_5, (0..5).map(f_i).collect::<Vec<_>>());
    assert_eq!(p_take_5, p_t_0_5);

    assert!(solution(4.)[0] <= 100.);
    assert!(solution(5.)[0] > 100.);
    assert_eq!(p_g_100, (5..=50).map(f_i).collect::<Vec<_>>());
    assert_eq!(p_g_100_last, f_t(50.));
    assert_eq!(p_g_100_first, f_t(5.));

    assert!(
        (0..=100)
            .map(|i| {
                let t = i as f64 * 0.5;
                (p_sub2[i + 1].1 - f_t(t).1).abs()
            })
            .fold(0f64, |acc, x| acc.max(x))
            < 1e-7
    );
    assert!(
        (0..=200)
            .map(|i| {
                let t = i as f64 * 0.25;
                (p_sub4[i + 3].1 - f_t(t).1).abs()
            })
            .fold(0f64, |acc, x| acc.max(x))
            < 1e-7
    );
}
