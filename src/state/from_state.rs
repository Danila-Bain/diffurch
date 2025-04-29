use super::State;

pub trait FromState<T> {
    fn from_state(t: T) -> Self;
}

impl<const N: usize, const S: usize, InitialFunction: Fn(f64) -> [f64; N]>
    FromState<&State<N, S, InitialFunction>> for (f64, [f64; N])
{
    fn from_state(state: &State<N, S, InitialFunction>) -> Self {
        (state.t, state.x)
    }
}

impl<const N: usize, const S: usize, InitialFunction: Fn(f64) -> [f64; N]>
    FromState<(&State<N, S, InitialFunction>,)> for (f64, [f64; N])
{
    fn from_state(state: (&State<N, S, InitialFunction>,)) -> Self {
        let state = state.0;
        (state.t, state.x)
    }
}

impl<const N: usize, const S: usize, InitialFunction: Fn(f64) -> [f64; N]>
    FromState<&State<N, S, InitialFunction>> for ([f64; N],)
{
    fn from_state(state: &State<N, S, InitialFunction>) -> Self {
        (state.x,)
    }
}

impl<const N: usize, const S: usize, InitialFunction: Fn(f64) -> [f64; N]>
    FromState<(&State<N, S, InitialFunction>,)> for ([f64; N],)
{
    fn from_state(state: (&State<N, S, InitialFunction>,)) -> Self {
        let state = state.0;
        (state.x,)
    }
}
