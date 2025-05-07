struct State {
    value: f64,
}

impl State {
    fn get_function<'a>(&'a self) -> Box<dyn Fn() -> f64 + 'a> {
        Box::new(|| self.value)
    }
}

fn apply<F>(f: F) -> f64
where
    F: for<'a> Fn(Box<dyn Fn() -> f64 + 'a>) -> f64, // Accepts any 'a
{
    let state = State { value: 3. };
    let state_function = state.get_function();
    f(state_function)
}

#[test]
fn main() {
    // let square = |func: Box<dyn Fn() -> f64>| func().powi(2); // fails
    let square = |func: Box<dyn Fn() -> f64 + '_>| func().powi(2);
    let double = |func: Box<dyn Fn() -> f64 + '_>| func() * 2.;
    assert_eq!(apply(square), 9.);
    assert_eq!(apply(double), 6.);

    assert_eq!(apply(|func| func() * 0.), 0.);
}
