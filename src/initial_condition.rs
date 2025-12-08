//! Defines [InitialCondition].

use nalgebra::RealField;

use crate::traits::RealVectorSpace;

/// Trait for objects that can be interpreted as valid initial conditions for a differential
/// equation (ODE or DDE).
pub trait InitialCondition<T: RealField + Copy, Y: RealVectorSpace<T>> {
    /// evaluate a derivative of order `D`
    fn eval<const D: usize>(&self, t: T) -> Y;
}

/// For this type, the value is interpreted as a constant function. All its derivatives are zero
impl<T: RealField + Copy, Y: RealVectorSpace<T> + From<U>, U: Copy> InitialCondition<T, Y> for U {
    fn eval<const D: usize>(&self, _t: T) -> Y {
        match D {
            0 => self.clone().into(),
            _ => Y::zero(),
        }
    }
}

struct InitialFunction<F>(F);

/// For this type, the value is interpreted as an initial function. Calling [Self::eval] for `D >=
/// 1` will panic.
impl<T: RealField + Copy, F, Y: RealVectorSpace<T>> InitialCondition<T, Y> for InitialFunction<F>
where
    F: Fn(T) -> Y,
{
    fn eval<const D: usize>(&self, t: T) -> Y {
        match D {
            0 => (self.0)(t),
            _ => unimplemented!("Differentiation is not implemented for this type."),
        }
    }
}

// /// For this type, the value is interpreted as an initial function and its derivative. Calling [Self::eval] for `D >=
// /// 2` will panic.
// impl<const N: usize, F, DF, T> InitialCondition<N, T> for (F, DF)
// where
//     F: Fn(T) -> [T; N],
//     DF: Fn(T) -> [T; N],
// {
//     fn eval<const D: usize>(&self, t: T) -> [T; N] {
//         match D {
//             0 => self.0(t),
//             1 => self.1(t),
//             _ => unimplemented!(
//                 "Differentiation of higher order (>1) is not implemented for a pair of Fn(f64) -> [f64; N]"
//             ),
//         }
//     }
// }
