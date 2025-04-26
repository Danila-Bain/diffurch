use std::marker::Tuple;

pub struct TupleTower<T = ()>(pub T);

impl<T> TupleTower<T> {
    pub fn append<H>(self, head: H) -> TupleTower<(H, TupleTower<T>)> {
        TupleTower((head, self))
    }
}

impl<Args, H, T> FnOnce<Args> for TupleTower<(H, TupleTower<T>)>
where
    H: FnOnce<Args>,
    TupleTower<T>: FnOnce<Args>,
    Args: Tuple + Copy,
{
    // type Output = (H::Output, <TupleTower<T> as FnOnce<Args>>::Output);
    type Output = TupleTower<(H::Output, <TupleTower<T> as FnOnce<Args>>::Output)>;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output {
        let TupleTower((head, tail)) = self;
        TupleTower((head.call_once(args), tail.call_once(args)))
    }
}

impl<Args> FnOnce<Args> for TupleTower<()>
where
    Args: Tuple,
{
    type Output = TupleTower<()>;

    extern "rust-call" fn call_once(self, _args: Args) -> Self::Output {
        TupleTower(())
    }
}

impl<Args, H, T> FnMut<Args> for TupleTower<(H, TupleTower<T>)>
where
    H: FnMut<Args>,
    TupleTower<T>: FnMut<Args>,
    Args: Tuple + Copy,
{
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output {
        let TupleTower((head, tail)) = self;
        TupleTower((head.call_mut(args), tail.call_mut(args)))
    }
}

impl<Args> FnMut<Args> for TupleTower<()>
where
    Args: Tuple,
{
    extern "rust-call" fn call_mut(&mut self, _args: Args) -> Self::Output {
        TupleTower(())
    }
}

impl<Args, H, T> Fn<Args> for TupleTower<(H, TupleTower<T>)>
where
    H: Fn<Args>,
    TupleTower<T>: Fn<Args>,
    Args: Tuple + Copy,
{
    extern "rust-call" fn call(&self, args: Args) -> Self::Output {
        let TupleTower((head, tail)) = self;
        TupleTower((head.call(args), tail.call(args)))
    }
}

impl<Args> Fn<Args> for TupleTower<()>
where
    Args: Tuple,
{
    extern "rust-call" fn call(&self, _args: Args) -> Self::Output {
        TupleTower(())
    }
}

pub struct TupleTowerLevel0 {}
pub struct TupleTowerNextLevel<Level> {_prev: Level}
pub trait TupleTowerLevel {
 type Level;
}
impl TupleTowerLevel for TupleTower<()> {
    type Level = TupleTowerLevel0;
}
impl<Head, T> TupleTowerLevel for TupleTower<(Head, T)> where
T : TupleTowerLevel {
    type Level = TupleTowerNextLevel<T::Level>;
}

