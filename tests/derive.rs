use derive_state::State;
use diffurch::Solver;
use nalgebra::Vector2;

#[test]
fn simple_struct() {
    #[derive(State, Default)]
    struct State {
        x: f64,
        y: Vector2<f64>,
    }

    Solver::new::<f64, State>()
        .stepsize(0.001)
        .initial(State {
            x: 1.,
            y: [2., 3.].into(),
        })
        .interval(0. ..10.)
        .equation(|s| State { x: s.p.x, y: s.p.y })
        .on_step(|s| {
            let diff = *s.p
                - State {
                    x: s.t.exp(),
                    y: [2. * s.t.exp(), 3. * s.t.exp()].into(),
                };
            assert!(diff.x.abs() < 0.001);
            assert!(diff.y.x.abs() < 0.001);
            assert!(diff.y.y.abs() < 0.001);
        })
        .run();
}

#[test]
fn simple_tuple() {
    #[derive(State, Default)]
    struct State(f64, Vector2<f64>);

    Solver::new::<f64, State>()
        .stepsize(0.001)
        .initial(State(1., [2., 3.].into()))
        .interval(0. ..10.)
        .equation(|s| State(s.p.0, [s.p.1.x, s.p.1.y].into()))
        .on_step(|s| {
            let diff = *s.p - State(s.t.exp(), [2. * s.t.exp(), 3. * s.t.exp()].into());
            assert!(diff.0.abs() < 0.001);
            assert!(diff.1.x.abs() < 0.001);
            assert!(diff.1.y.abs() < 0.001);
        })
        .run();
}

#[test]
fn generic_simple_tuple() {
    #[derive(State)]
    struct State<T: num_traits::Float>(T)
    where
        T: std::ops::AddAssign + std::fmt::Debug;

    Solver::new::<f32, State<f32>>()
        .initial(State(0.))
        .equation(|_| State(1.))
        .interval(0. ..1.)
        .on_stop(|s| assert!((s.p.0 - s.t).abs() < 1e-15))
        .stepsize(1. / 64.)
        .run();

    Solver::new::<f64, State<f64>>()
        .initial(State(0.))
        .equation(|_| State(1.))
        .interval(0. ..1.)
        .on_step(|s| assert!((s.p.0 - s.t).abs() < 1e-15))
        .stepsize(1. / 64.)
        .run();
}

#[test]
fn simple_array_field() {
    #[derive(State, derive_more::Deref)]
    struct State([f64; 2]);

    Solver::new::<f64, State>()
        .initial(State([1., 1.]))
        .equation(|s| State([s.p[0], 2. * s.p[1]]))
        .interval(0. ..1.)
        .on_step(|s| {
            assert!((s.p[0] - s.t.exp()).abs() < 1e-10);
            assert!((s.p[1] - (2. * s.t).exp()).abs() < 1e-10);
        })
        .run();
}

#[test]
fn simple_tuple_field() {
    #[derive(State, derive_more::Deref)]
    struct State {
        point: (f64, f64),
    }

    Solver::new::<f64, State>()
        .initial(State { point: (1., 1.) })
        .equation(|s| State {
            point: (s.p.0, 2. * s.p.1),
        })
        .interval(0. ..1.)
        .on_step(|s| {
            assert!((s.p.0 - s.t.exp()).abs() < 1e-10);
            assert!((s.p.1 - (2. * s.t).exp()).abs() < 1e-10);
        })
        .run();
}

#[test]
fn nested_tuples() {
    #[derive(State)]
    struct State(f64, (f64, f64), ((f64, f64), (f64, f64)));

    Solver::new::<f64, State>()
        .initial(State(1., (1., 1.), ((1., 1.), (1., 1.))))
        .interval(0. ..1.)
        .equation(|s| {
            let p = s.p;
            State(
                p.0,
                (2. * p.1.0, 3. * p.1.1),
                ((4. * p.2.0.0, 5. * p.2.0.1), (6. * p.2.1.0, 7. * p.2.1.1)),
            )
        })
        .stepsize(0.001)
        .on_step(|s| {
            let diff = *s.p
                - State(
                    s.t.exp(),
                    ((2. * s.t).exp(), (3. * s.t).exp()),
                    (
                        ((4. * s.t).exp(), (5. * s.t).exp()),
                        ((6. * s.t).exp(), (7. * s.t).exp()),
                    ),
                );
            assert!(diff.0.abs() < 1e-10);
            assert!(diff.1.0.abs() < 1e-10);
            assert!(diff.1.1.abs() < 1e-10);
            assert!(diff.2.0.0.abs() < 1e-10);
            assert!(diff.2.0.1.abs() < 1e-10);
            assert!(diff.2.1.0.abs() < 1e-10);
            assert!(diff.2.1.1.abs() < 1e-10);
        })
        .run();
}
