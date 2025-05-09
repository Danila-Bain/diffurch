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

impl<'a, const N: usize> From<[f64; N]> for InitialCondition<'a, N> {
    fn from(value: [f64; N]) -> Self {
        Self::Point(value)
    }
}

impl<'a, const N: usize, F: 'a + Fn(f64) -> [f64; N]> From<F> for InitialCondition<'a, N> {
    fn from(value: F) -> Self {
        Self::Function(Box::new(value))
    }
}

impl<'a, const N: usize, F: 'a + Fn(f64) -> [f64; N], DF: 'a + Fn(f64) -> [f64; N]> From<(F, DF)>
    for InitialCondition<'a, N>
{
    fn from(value: (F, DF)) -> Self {
        Self::FunctionWithDerivative(Box::new(value.0), Box::new(value.1))
    }
}
