//! Defines [InitialCondition].

/// Container of initial conditions.
pub enum InitialCondition<'a, const N: usize> {
    /// Represents an initial value (for ordinary differential equations) or a constant function (for (neutral) delay differential equations)
    Point([f64; N]),
    /// Represents an initial function (for delay differential equations)
    Function(Box<dyn 'a + Fn(f64) -> [f64; N]>),
    /// Represents an initial function (for delay differential equations), together with its
    /// derivative. The derivative of the initial function is required for neutral delay
    /// differential equations.
    FunctionWithDerivative(
        Box<dyn 'a + Fn(f64) -> [f64; N]>,
        Box<dyn 'a + Fn(f64) -> [f64; N]>,
    ),
}

impl<'a, const N: usize> InitialCondition<'a, N> {
    /// Evaluate the initial conditions at a given time.
    ///
    /// For Point variant, it is just its value,
    /// Function or FunctionWithDerivative, it is the value of a function at that time.
    pub fn eval(&self, t: f64) -> [f64; N] {
        match self {
            &Self::Point(value) => value,
            Self::Function(f) | Self::FunctionWithDerivative(f, _) => f(t),
        }
    }

    /// Evaluate the derivative of the inital condition at a given time.
    ///
    /// For Point variant, it is zero array.
    /// For Function variant, it panics.
    /// For FunctionWithDerivative variant, the second function is evaluated.
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
