use super::{State, ToStateFn};
use crate::util::tutle::{Tutle, TutleLevel, TutleLevel0, TutleNextLevel};

pub trait ToStateTutle<S, Arg, R, Level> {
    type StateTutle: for<'b> FnMut<(&'b S,), Output = R>;
    type StateEvalTutle: for<'b> FnMut<(&'b S, f64), Output = R>;

    fn to_state_tutle(self) -> Self::StateTutle;
    fn to_state_eval_tutle(self) -> Self::StateEvalTutle;
}

impl<const N: usize, const S: usize, IF: Fn(f64) -> [f64; N]>
    ToStateTutle<State<N, S, IF>, Tutle<()>, Tutle<()>, TutleLevel0> for Tutle<()>
where
    Tutle<()>: for<'b> Fn<(&'b State<N, S, IF>,), Output = Tutle<()>>,
{
    type StateTutle = impl for<'b> FnMut<(&'b State<N, S, IF>,), Output = Tutle<()>>;
    type StateEvalTutle = impl for<'b> FnMut<(&'b State<N, S, IF>, f64), Output = Tutle<()>>;

    fn to_state_tutle(self) -> Self::StateTutle {
        Tutle(())
    }

    fn to_state_eval_tutle(self) -> Self::StateEvalTutle {
        Tutle(())
    }
}

impl<const N: usize, const S: usize, IF: Fn(f64) -> [f64; N], H, T, HArg, TArg, HR, TR, TL>
    ToStateTutle<
        State<N, S, IF>,
        Tutle<(HArg, TArg)>,
        Tutle<(HR, TR)>,
        TutleNextLevel<TL>,
    > for Tutle<(H, T)>
where
    H: ToStateFn<State<N, S, IF>, HArg, HR>,
    T: ToStateTutle<State<N, S, IF>, TArg, TR, TL>,
    Tutle<(H, T)>: TutleLevel<Level = TutleNextLevel<TL>>,
    T: TutleLevel<Level = TL>,
{
    type StateTutle = impl for<'b> FnMut<(&'b State<N, S, IF>,), Output = Tutle<(HR, TR)>>;
    type StateEvalTutle =
        impl for<'b> FnMut<(&'b State<N, S, IF>, f64), Output = Tutle<(HR, TR)>>;

    fn to_state_tutle(self) -> Self::StateTutle {
        let Tutle((head, tail)) = self;
        Tutle((head.to_state_function(), tail.to_state_tutle()))
    }

    fn to_state_eval_tutle(self) -> Self::StateEvalTutle {
        let Tutle((head, tail)) = self;
        Tutle((head.to_state_eval_function(), tail.to_state_eval_tutle()))
    }
}
