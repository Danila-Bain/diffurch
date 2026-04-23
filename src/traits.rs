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

// impl<T: RealField + Copy> RealVectorSpace<T> for T {}
// impl<T: RealField + Copy> RealVectorSpace<T> for Complex<T> {}
// impl<T: RealField + Copy, const R: usize, const C: usize> RealVectorSpace<T> for SMatrix<T, R, C> {}

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

// pub trait State:
//     Clone
//     + Copy
//     + std::fmt::Debug
//     + Add<Output = Self>
//     + Sub<Output = Self>
//     + AddAssign
//     + Mul<Self::Time, Output = Self>
//     + Div<Self::Time, Output = Self>
//     + Neg<Output = Self>
//     + num_traits::Zero
// {
//     type Time: RealField;
// }

// impl State for f32 {
//     type Time = f32;
// }

// impl State for f64 {
//     type Time = f64;
// }

// impl<T: RealField + Copy> State for Complex<T> {
//     type Time = T;
// }

// impl<T: RealField + Copy, const R: usize, const C: usize> State for SMatrix<T, R, C> {
//     type Time = T;
// }
