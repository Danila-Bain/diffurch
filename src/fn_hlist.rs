// use hlist2::ops::MapFn;
//
// pub struct ReborrowMapFn<Arg>(pub Arg);
// impl<T, Arg, Ret> MapFn<T> for ReborrowMapFn<Arg>
// where
//     T: for<'a> FnMut(&'a mut Arg) -> Ret,
// {
//     type Output = Ret;
//     fn map(&mut self, mut f: T) -> Self::Output {
//         f(&mut self.0)
//     }
// }

// use std::marker::Tuple;
//
// use hlist2::{Cons, HList, Nil};
//
// pub trait FnMutHList<Args: Tuple>: HList {
//     type Output: HList;
//     fn call_mut(&mut self, args: Args) -> Self::Output;
// }
//
// impl<Args: Tuple> FnMutHList<Args> for Nil {
//     type Output = Nil;
//     fn call_mut(&mut self, _args: Args) -> Self::Output {
//         Nil
//     }
// }
//
// impl<Args: Tuple + Copy, Head, Tail> FnMutHList<Args> for Cons<Head, Tail>
// where
//     Head: FnMut<Args>,
//     Tail: FnMutHList<Args>,
// {
//     type Output = Cons<Head::Output, Tail::Output>;
//     fn call_mut(&mut self, args: Args) -> Self::Output {
//         let Cons(head, tail) = self;
//         let head = head.call_mut(args);
//         let tail = tail.call_mut(args);
//         Cons(head, tail)
//     }
// }
//
//
// pub trait MutRefFnMutHList<Arg>: HList {
//     type Output: HList;
//     fn call_mut(&mut self, arg: &mut Arg) -> Self::Output;
// }
//
// impl<Arg> MutRefFnMutHList<Arg> for Nil {
//     type Output = Nil;
//     fn call_mut(&mut self, _: &mut Arg) -> Self::Output {
//         Nil
//     }
// }
//
// impl<Arg, Head, Tail, Ret> MutRefFnMutHList<Arg> for Cons<Head, Tail>
// where
//     Head: FnMut(&mut Arg) -> Ret,
//     Tail: MutRefFnMutHList<Arg>,
// {
//     type Output = Cons<Ret, Tail::Output>;
//     fn call_mut(&mut self, arg: &mut Arg) -> Self::Output {
//         let Cons(head, tail) = self;
//         let head = head(arg);
//         let tail = tail.call_mut(arg);
//         Cons(head, tail)
//     }
// }
//
//
// pub trait BoolFnMutHList<Args: Tuple>: FnMutHList<Args> {
//     fn all(&mut self, args: Args) -> bool;
//     fn any(&mut self, args: Args) -> bool;
// }
//
// impl<Args: Tuple> BoolFnMutHList<Args> for Nil {
//     fn all(&mut self, _: Args) -> bool {
//         true
//     }
//
//     fn any(&mut self, _: Args) -> bool {
//         false
//     }
// }
//
// impl<Args: Tuple + Copy, Head, Tail> BoolFnMutHList<Args> for Cons<Head, Tail>
// where
//     Head: FnMut<Args, Output = bool>,
//     Tail: BoolFnMutHList<Args>,
// {
//     fn all(&mut self, args: Args) -> bool {
//         let Cons(head, tail) = self;
//         head.call_mut(args) && tail.all(args)
//     }
//     fn any(&mut self, args: Args) -> bool {
//         let Cons(head, tail) = self;
//         head.call_mut(args) || tail.all(args)
//     }
// }
//
// pub trait FnHList<Args: Tuple>: HList {
//     type Output: HList;
//     fn call(&self, args: Args) -> Self::Output;
// }
//
// impl<Args: Tuple> FnHList<Args> for Nil {
//     type Output = Nil;
//     fn call(&self, _args: Args) -> Self::Output {
//         Nil
//     }
// }
//
// impl<Args: Tuple + Copy, Head, Tail> FnHList<Args> for Cons<Head, Tail>
// where
//     Head: Fn<Args>,
//     Tail: FnHList<Args>,
// {
//     type Output = Cons<Head::Output, Tail::Output>;
//     fn call(&self, args: Args) -> Self::Output {
//         let Cons(head, tail) = self;
//         let head = head.call(args);
//         let tail = tail.call(args);
//         Cons(head, tail)
//     }
// }
