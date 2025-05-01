use super::{State, ToStateFn};
use crate::util::tutle::{Tutle, TutleLevel, TutleLevel0, TutleNextLevel};

pub trait ToStateTutle<S, Arg, R, Level> {
    fn to_state_tutle(self) -> impl for<'b> FnMut<(&'b S,), Output = R>;
    fn to_state_eval_tutle(self) -> impl for<'b> FnMut<(&'b S, f64,), Output = R>;
}

impl<const N: usize, const S: usize, IF: Fn(f64) -> [f64; N]>
    ToStateTutle<
        State<N, S, IF>,
        Tutle<()>,
        Tutle<()>,
        TutleLevel0,
    > for Tutle<()>
where
    Tutle<()>: for<'b> Fn<(&'b State<N, S, IF>,), Output = Tutle<()>>,
{
    fn to_state_tutle(
        self,
    ) -> impl for<'b> FnMut<(&'b State<N, S, IF>,), Output = Tutle<()>> {
        Tutle(())
    }


    fn to_state_eval_tutle(
        self,
    ) -> impl for<'b> FnMut<(&'b State<N, S, IF>, f64,), Output = Tutle<()>> {
        Tutle(())
    }
}

impl<
    const N: usize,
    const S: usize,
    IF: Fn(f64) -> [f64; N],
    H,
    T,
    HArg,
    TArg,
    HR,
    TR,
    TL,
>
    ToStateTutle<
        State<N, S, IF>,
        Tutle<(HArg, Tutle<TArg>)>,
        Tutle<(HR, Tutle<TR>)>,
        TutleNextLevel<TL>,
    > for Tutle<(H, Tutle<T>)>
where
    H: ToStateFn<State<N, S, IF>, HArg, HR>,
    Tutle<T>: ToStateTutle<
            State<N, S, IF>,
            Tutle<TArg>,
            Tutle<TR>,
            TL,
        >,
    Tutle<(H, Tutle<T>)>: TutleLevel<Level = TutleNextLevel<TL>>,
    Tutle<T>: TutleLevel<Level = TL>,
{
    fn to_state_tutle(
        self,
    ) -> impl for<'b> FnMut<
        (&'b State<N, S, IF>,),
        Output = Tutle<(HR, Tutle<TR>)>,
    > {
        let Self((head, tail)) = self;
        Tutle((head.to_state_function(), tail.to_state_tutle()))
    }

    fn to_state_eval_tutle(
        self,
    ) -> impl for<'b> FnMut<
        (&'b State<N, S, IF>, f64,),
        Output = Tutle<(HR, Tutle<TR>)>,
    > {
        let Self((head, tail)) = self;
        Tutle((head.to_state_eval_function(), tail.to_state_eval_tutle()))
    }
}
