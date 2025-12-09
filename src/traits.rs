use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

use nalgebra::{Complex, RealField, SMatrix};

pub trait RealVectorSpace<T: RealField>:
    Clone
    + Copy
    + std::fmt::Debug
    + Add<Output = Self>
    + Sub<Output = Self>
    + AddAssign
    + Mul<T, Output = Self>
    + Div<T, Output = Self>
    + Neg<Output = Self>
    + num_traits::Zero
{
}

impl<T: RealField + Copy> RealVectorSpace<T> for T {}
impl<T: RealField + Copy> RealVectorSpace<T> for Complex<T> {}
impl<T: RealField + Copy, const R: usize, const C: usize> RealVectorSpace<T> for SMatrix<T, R, C> {}
