use super::State;

pub struct CoordinateFunction<
    'state,
    const N: usize,
    const S: usize,
    InitialFunction: Fn(f64) -> [f64; N],
> {
    pub state_ref: &'state State<N, S, InitialFunction>,
    pub coordinate: usize,
}

impl<'state, const N: usize, const S: usize, InitialFunction: Fn(f64) -> [f64; N]> FnOnce<(f64,)>
    for CoordinateFunction<'state, N, S, InitialFunction>
{
    type Output = f64;

    extern "rust-call" fn call_once(self, args: (f64,)) -> Self::Output {
        let (t,) = args;
        return self.state_ref.eval(t, self.coordinate);
    }
}

impl<'state, const N: usize, const S: usize, InitialFunction: Fn(f64) -> [f64; N]> FnMut<(f64,)>
    for CoordinateFunction<'state, N, S, InitialFunction>
{
    extern "rust-call" fn call_mut(&mut self, args: (f64,)) -> Self::Output {
        let (t,) = args;
        return self.state_ref.eval(t, self.coordinate);
    }
}

impl<'state, const N: usize, const S: usize, InitialFunction: Fn(f64) -> [f64; N]> Fn<(f64,)>
    for CoordinateFunction<'state, N, S, InitialFunction>
{
    extern "rust-call" fn call(&self, args: (f64,)) -> Self::Output {
        let (t,) = args;
        return self.state_ref.eval(t, self.coordinate);
    }
}
