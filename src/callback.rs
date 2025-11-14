// use crate::collections::hlists::*;
use hlist2;

pub struct Callback<
    const N: usize,
    const MUT: bool = false,
    Subdivision = (),
    Func = (),
    Filter = hlist2::Nil,
> {
    pub func: Func,
    pub filter: Filter,
    pub subdivision: Subdivision,
}


