use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

// use nalgebra::{Complex, RealField, SMatrix};

pub trait RealVectorSpace<T: nalgebra::RealField>:
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

impl<T: nalgebra::RealField + Copy, Y> RealVectorSpace<T> for Y where
    Y: Clone
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
