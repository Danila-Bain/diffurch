use std::marker::Tuple;

use hlist2::{HList, Cons, Nil};

pub trait FnMutHList<Args: Tuple> : HList {
    type Output : HList;
    fn call_mut(&mut self, args: Args) -> Self::Output;
}

impl<Args: Tuple> FnMutHList<Args> for Nil {
    type Output = Nil;
    fn call_mut(&mut self, _args: Args) -> Self::Output {
        Nil
    }
}

impl<Args: Tuple + Copy, Head, Tail> FnMutHList<Args> for Cons<Head, Tail>
where Head: FnMut<Args>, Tail: FnMutHList<Args> {
    type Output = Cons<Head::Output, Tail::Output>;
    fn call_mut(&mut self, args: Args) -> Self::Output {
        let Cons(head, tail) = self;
        let head = head.call_mut(args);
        let tail = tail.call_mut(args);
        Cons(head, tail)
    }
}


pub trait FnHList<Args: Tuple> : HList {
    type Output : HList;
    fn call(&self, args: Args) -> Self::Output;
}

impl<Args: Tuple> FnHList<Args> for Nil {
    type Output = Nil;
    fn call(&self, _args: Args) -> Self::Output {
        Nil
    }
}

impl<Args: Tuple + Copy, Head, Tail> FnHList<Args> for Cons<Head, Tail>
where Head: Fn<Args>, Tail: FnHList<Args> {
    type Output = Cons<Head::Output, Tail::Output>;
    fn call(&self, args: Args) -> Self::Output {
        let Cons(head, tail) = self;
        let head = head.call(args);
        let tail = tail.call(args);
        Cons(head, tail)
    }
}
