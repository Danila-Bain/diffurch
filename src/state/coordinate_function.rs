use super::State;

pub struct CoordFn<
    'state,
    const N: usize,
    const S: usize,
    IF: Fn(f64) -> [f64; N],
> {
    pub state_ref: &'state State<N, S, IF>,
    pub coordinate: usize,
}


impl<'state, const N: usize, const S: usize, IF: Fn(f64) -> [f64; N]> FnOnce<(f64,)>
    for CoordFn<'state, N, S, IF>
{
    type Output = f64;

    extern "rust-call" fn call_once(self, args: (f64,)) -> Self::Output {
        let (t,) = args;
        return self.state_ref.eval(t, self.coordinate);
    }
}

impl<'state, const N: usize, const S: usize, IF: Fn(f64) -> [f64; N]> FnMut<(f64,)>
    for CoordFn<'state, N, S, IF>
{
    extern "rust-call" fn call_mut(&mut self, args: (f64,)) -> Self::Output {
        let (t,) = args;
        return self.state_ref.eval(t, self.coordinate);
    }
}

impl<'state, const N: usize, const S: usize, IF: Fn(f64) -> [f64; N]> Fn<(f64,)>
    for CoordFn<'state, N, S, IF>
{
    extern "rust-call" fn call(&self, args: (f64,)) -> Self::Output {
        let (t,) = args;
        return self.state_ref.eval(t, self.coordinate);
    }
}
