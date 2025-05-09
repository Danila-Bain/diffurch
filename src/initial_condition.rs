pub enum InitialCondition<'a, const N: usize> {
    Point([f64; N]),
    Function(Box<dyn 'a + Fn(f64) -> [f64; N]>),
    FunctionWithDerivative(
        Box<dyn 'a + Fn(f64) -> [f64; N]>,
        Box<dyn 'a + Fn(f64) -> [f64; N]>,
    ),
}

impl<'a, const N: usize> InitialCondition<'a, N> {
    pub fn eval(&self, t: f64) -> [f64; N] {
        match self {
            &Self::Point(value) => value,
            Self::Function(f) | Self::FunctionWithDerivative(f, _) => f(t),
        }
    }

    pub fn eval_d(&self, t: f64) -> [f64; N] {
        match &self {
            &Self::Point(_) => [0.; N],
            Self::FunctionWithDerivative(_, df) => df(t),
            Self::Function(_) => panic!(
                "derivative is not supported for InitialCondition::Function variant.\n\
                    help: use InitialCondition::FunctionWithDerivative or InitialCondition::Point instead."
            ),
        }
    }
}
