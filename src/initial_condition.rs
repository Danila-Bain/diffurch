//! Defines [InitialCondition].

/// Trait for objects that can be interpreted as valid initial conditions for a differential
/// equation (ODE or DDE).
pub trait InitialCondition<const N: usize, T> {
    /// evaluate a derivative of order `D`
    fn eval<const D: usize>(&self, t: T) -> [T; N];
}

/// For this type, the value is interpreted as a constant function. All its derivatives are zero
impl<const N: usize, T> InitialCondition<N, T> for [T; N]
where
    [T; N]: Clone,
    T: Copy + From<f64>,
{
    fn eval<const D: usize>(&self, _t: T) -> [T; N] {
        match D {
            0 => self.clone(),
            _ => [T::from(0.); N],
        }
    }
}

/// For this type, the value is interpreted as an initial function. Calling [Self::eval] for `D >=
/// 1` will panic.
impl<const N: usize, F, T> InitialCondition<N, T> for F
where
    F: Fn(T) -> [T; N],
{
    fn eval<const D: usize>(&self, t: T) -> [T; N] {
        match D {
            0 => self(t),
            _ => unimplemented!("Differentiation is not implemented for Fn(f64) -> [f64; N]"),
        }
    }
}

/// For this type, the value is interpreted as an initial function and its derivative. Calling [Self::eval] for `D >=
/// 2` will panic.
impl<const N: usize, F, DF, T> InitialCondition<N, T> for (F, DF)
where
    F: Fn(T) -> [T; N],
    DF: Fn(T) -> [T; N],
{
    fn eval<const D: usize>(&self, t: T) -> [T; N] {
        match D {
            0 => self.0(t),
            1 => self.1(t),
            _ => unimplemented!(
                "Differentiation of higher order (>1) is not implemented for a pair of Fn(f64) -> [f64; N]"
            ),
        }
    }
}
