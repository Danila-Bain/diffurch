use std::marker::Tuple;

pub struct TupleTower<T = (), const N : usize = 0>(pub T);

impl<T, const N: usize> TupleTower<T, N> {
    pub fn append<H>(self, head: H) -> TupleTower<(H, TupleTower<T, N>), {N+1}> {
        TupleTower((head, self))
    }
}

impl<Args, H, T, const N: usize> FnOnce<Args> for TupleTower<(H, TupleTower<T, N>), {N+1}>
where
    H: FnOnce<Args>,
    TupleTower<T, N>: FnOnce<Args>,
    Args: Tuple + Copy,
{
    // type Output = (H::Output, <TupleTower<T> as FnOnce<Args>>::Output);
    type Output = TupleTower<(H::Output, <TupleTower<T, N> as FnOnce<Args>>::Output), {N+1}>;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output {
        let TupleTower((head, tail)) = self;
        TupleTower((head.call_once(args), tail.call_once(args)))
    }
}

impl<Args> FnOnce<Args> for TupleTower<(), 0>
where
    Args: Tuple,
{
    type Output = TupleTower<(), 0>;

    extern "rust-call" fn call_once(self, _args: Args) -> Self::Output {
       TupleTower(()) 
    }
}



impl<Args, H, T, const N: usize> FnMut<Args> for TupleTower<(H, TupleTower<T, N>), {N+1}>
where
    H: FnMut<Args>,
    TupleTower<T, N>: FnMut<Args>,
    Args: Tuple + Copy,
{
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output {
        let TupleTower((head, tail)) = self;
        TupleTower((head.call_mut(args), tail.call_mut(args)))
    }
}


impl<Args> FnMut<Args> for TupleTower<(), 0>
where
    Args: Tuple,
{
    extern "rust-call" fn call_mut(&mut self, _args: Args) -> Self::Output {
       TupleTower(()) 
    }
}


impl<Args, H, T, const N: usize> Fn<Args> for TupleTower<(H, TupleTower<T, N>), {N+1}>
where
    H: Fn<Args>,
    TupleTower<T, N>: Fn<Args>,
    Args: Tuple + Copy,
{
    extern "rust-call" fn call(&self, args: Args) -> Self::Output {
        let TupleTower((head, tail)) = self;
        TupleTower((head.call(args), tail.call(args)))
    }
}


impl<Args> Fn<Args> for TupleTower<(), 0>
where
    Args: Tuple,
{
    extern "rust-call" fn call(&self, _args: Args) -> Self::Output {
       TupleTower(()) 
    }
}
