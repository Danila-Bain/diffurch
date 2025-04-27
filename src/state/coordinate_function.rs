use crate::state::State;

pub trait CoordinateFunction<'a> {
    type CoordinateFunctionType: Fn<(f64,), Output = f64> + 'a;
    fn coordinate_function(&'a self, coordinate: usize) -> Self::CoordinateFunctionType;
}

impl<'a, const N: usize, const S: usize, InitialFunction: Fn(f64) -> [f64; N] + 'a>
    CoordinateFunction<'a> for State<N, S, InitialFunction>
{
    type CoordinateFunctionType = impl Fn<(f64,), Output = f64> + 'a;

    fn coordinate_function(&'a self, coordinate: usize) -> Self::CoordinateFunctionType {
        move |t| self.eval_i(t, coordinate)
    }
}
