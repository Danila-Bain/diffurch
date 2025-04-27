use super::{CoordinateFunction, State};

// use super::CoordinateFunction;

pub trait ToStateFunction<S, Arg, Ret> {
    fn to_state_function(self) -> impl for<'b> FnMut<(&'b S,), Output = Ret>;
}

// impl<const N: usize, const S: usize, InitialFunction: Fn(f64) -> [f64; N], F, Ret>
//     ToStateFunction<State<N, S, InitialFunction>, (&State<N, S, InitialFunction>,), Ret> for F
// where
//     F: for<'a> Fn<(&'a State<N, S, InitialFunction>,), Output = Ret>,
// {
//     fn to_state_function(self) -> impl Fn(&State<N, S, InitialFunction>) -> Ret {
//         self
//     }
// }

impl<const N: usize, const S: usize, InitialFunction: Fn(f64) -> [f64; N], F, Ret>
    ToStateFunction<State<N, S, InitialFunction>, (f64,), Ret> for F
where
    F: FnMut<(f64,), Output = Ret>,
{
    fn to_state_function(
        mut self,
    ) -> impl for<'b> FnMut<(&'b State<N, S, InitialFunction>,), Output = Ret> {
        move |state| self(state.t)
    }
}

impl<const N: usize, const S: usize, InitialFunction: Fn(f64) -> [f64; N], F, Ret>
    ToStateFunction<State<N, S, InitialFunction>, ([f64; N],), Ret> for F
where
    F: FnMut<([f64; N],), Output = Ret>,
{
    fn to_state_function(
        mut self,
    ) -> impl for<'b> FnMut<(&'b State<N, S, InitialFunction>,), Output = Ret> {
        move |state| self(state.x)
    }
}

impl<const N: usize, const S: usize, InitialFunction: Fn(f64) -> [f64; N], F, Ret>
    ToStateFunction<State<N, S, InitialFunction>, (f64, [f64; N]), Ret> for F
where
    F: FnMut<(f64, [f64; N]), Output = Ret>,
{
    fn to_state_function(
        mut self,
    ) -> impl for<'b> FnMut<(&'b State<N, S, InitialFunction>,), Output = Ret> {
        move |state| self(state.t, state.x)
    }
}

impl<const N: usize, const S: usize, InitialFunction: Fn(f64) -> [f64; N], F, Ret>
    ToStateFunction<
        State<N, S, InitialFunction>,
        (
            f64,
            [f64; N],
            [CoordinateFunction<'_, N, S, InitialFunction>; N],
        ),
        Ret,
    > for F
where
    F: for<'a> FnMut<
            (
                f64,
                [f64; N],
                [CoordinateFunction<'a, N, S, InitialFunction>; N],
            ),
            Output = Ret,
        >,
{
    fn to_state_function(
        mut self,
    ) -> impl for<'b> FnMut<(&'b State<N, S, InitialFunction>,), Output = Ret> {
        move |state| {
            self(
                state.t,
                state.x,
                std::array::from_fn(|i| CoordinateFunction {
                    state_ref: state,
                    coordinate: i,
                }),
            )
        }
    }
}
