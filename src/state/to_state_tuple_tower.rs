use crate::{
    state::{State, ToStateFunction},
    util::tuple_tower::{TupleTower, TupleTowerLevel, TupleTowerLevel0, TupleTowerNextLevel},
};

pub trait ToStateTupleTower<S, Arg, Ret, Level> {
    fn to_state_tuple_tower(self) -> impl for<'b> FnMut<(&'b S,), Output = Ret>;
}

impl<const N: usize, const S: usize, InitialFunction: Fn(f64) -> [f64; N]>
    ToStateTupleTower<
        State<N, S, InitialFunction>,
        TupleTower<()>,
        TupleTower<()>,
        TupleTowerLevel0,
    > for TupleTower<()>
where
    TupleTower<()>: for<'b> Fn<(&'b State<N, S, InitialFunction>,), Output = TupleTower<()>>,
{
    fn to_state_tuple_tower(
        self,
    ) -> impl for<'b> FnMut<(&'b State<N, S, InitialFunction>,), Output = TupleTower<()>> {
        TupleTower(())
    }
}

impl<
    const N: usize,
    const S: usize,
    InitialFunction: Fn(f64) -> [f64; N],
    Head,
    Tail,
    HeadArg,
    TailArg,
    HeadRet,
    TailRet,
    TailLevel,
>
    ToStateTupleTower<
        State<N, S, InitialFunction>,
        TupleTower<(HeadArg, TupleTower<TailArg>)>,
        TupleTower<(HeadRet, TupleTower<TailRet>)>,
        TupleTowerNextLevel<TailLevel>,
    > for TupleTower<(Head, TupleTower<Tail>)>
where
    Head: ToStateFunction<State<N, S, InitialFunction>, HeadArg, HeadRet>,
    TupleTower<Tail>: ToStateTupleTower<
            State<N, S, InitialFunction>,
            TupleTower<TailArg>,
            TupleTower<TailRet>,
            TailLevel,
        >,
    TupleTower<(Head, TupleTower<Tail>)>: TupleTowerLevel<Level = TupleTowerNextLevel<TailLevel>>,
    TupleTower<Tail>: TupleTowerLevel<Level = TailLevel>,
{
    fn to_state_tuple_tower(
        self,
    ) -> impl for<'b> FnMut<
        (&'b State<N, S, InitialFunction>,),
        Output = TupleTower<(HeadRet, TupleTower<TailRet>)>,
    > {
        let Self((head, tail)) = self;
        TupleTower((head.to_state_function(), tail.to_state_tuple_tower()))
    }
}
