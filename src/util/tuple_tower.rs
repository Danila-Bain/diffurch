use std::marker::Tuple;

pub struct TupleTower<T = ()>(pub T);

impl<T> TupleTower<T> {
    pub fn append<H>(self, head: H) -> TupleTower<(H, TupleTower<T>)> {
        TupleTower((head, self))
    }
}

impl<Args, H, T> FnOnce<Args> for TupleTower<(H, T)>
where
    H: FnOnce<Args>,
    T: FnOnce<Args>,
    Args: Tuple + Copy,
{
    // type Output = (H::Output, <TupleTower<T> as FnOnce<Args>>::Output);
    type Output = TupleTower<(H::Output, T::Output)>;

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

impl<Args, H, T> FnMut<Args> for TupleTower<(H, T)>
where
    H: FnMut<Args>,
    T: FnMut<Args>,
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

impl<Args, H, T> Fn<Args> for TupleTower<(H, T)>
where
    H: Fn<Args>,
    T: Fn<Args>,
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


pub trait BoolTupleTower {
    fn any(&self) -> bool; 
    fn all(&self) -> bool; 
}

impl BoolTupleTower for TupleTower<()> {
    fn any(&self) -> bool { false }
    fn all(&self) -> bool { true }
}

impl<Rest> BoolTupleTower for TupleTower<(bool, Rest)>
where Rest: BoolTupleTower {
    fn any(&self) -> bool {
        let TupleTower((value, rest)) = self;
        *value || rest.any() 
    }

    fn all(&self) -> bool {
        let TupleTower((value, rest)) = self;
        *value && rest.all()
    }
}
