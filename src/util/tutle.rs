use std::marker::Tuple;

// tuple tower
#[derive(Debug, PartialEq)]
pub struct Tutle<T = ()>(pub T);

impl<T> Tutle<T> {
    pub fn append<H>(self, head: H) -> Tutle<(H, Tutle<T>)> {
        Tutle((head, self))
    }
}

// Tutle level trait to prevent infinite recursion
pub struct TutleLevel0 {}
pub struct TutleNextLevel<L> {
    _prev: L,
}
pub trait TutleLevel {
    type Level;
}
impl TutleLevel for Tutle<()> {
    type Level = TutleLevel0;
}
impl<H, T> TutleLevel for Tutle<(H, T)>
where
    T: TutleLevel,
{
    type Level = TutleNextLevel<T::Level>;
}

// implement Fn traits: single
impl<Args: Tuple> FnOnce<Args> for Tutle<()> {
    type Output = Tutle<()>;

    extern "rust-call" fn call_once(self, _: Args) -> Self::Output {
        Tutle(())
    }
}
impl<Args: Tuple> FnMut<Args> for Tutle<()> {
    extern "rust-call" fn call_mut(&mut self, _: Args) -> Self::Output {
        Tutle(())
    }
}
impl<Args: Tuple> Fn<Args> for Tutle<()> {
    extern "rust-call" fn call(&self, _: Args) -> Self::Output {
        Tutle(())
    }
}
impl<Args: Tuple + Copy, H, T, HR, TR> FnOnce<Args> for Tutle<(H, T)>
where
    H: FnOnce<Args, Output = HR>,
    T: FnOnce<Args, Output = TR>,
{
    type Output = Tutle<(HR, TR)>;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output {
        let Tutle((head, tail)) = self;
        Tutle((head.call_once(args), tail.call_once(args)))
    }
}
impl<Args: Tuple + Copy, H, T, HR, TR> FnMut<Args> for Tutle<(H, T)>
where
    H: FnMut<Args, Output = HR>,
    T: FnMut<Args, Output = TR>,
{
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output {
        let Tutle((head, tail)) = self;
        Tutle((head.call_mut(args), tail.call_mut(args)))
    }
}
impl<Args: Tuple + Copy, H, T, HR, TR> Fn<Args> for Tutle<(H, T)>
where
    H: Fn<Args, Output = HR>,
    T: Fn<Args, Output = TR>,
{
    extern "rust-call" fn call(&self, args: Args) -> Self::Output {
        let Tutle((head, tail)) = self;
        Tutle((head.call(args), tail.call(args)))
    }
}





pub trait BoolTutle {
    fn any(&self) -> bool; 
    fn all(&self) -> bool; 
}

impl BoolTutle for Tutle<()> {
    fn any(&self) -> bool { false }
    fn all(&self) -> bool { true }
}

impl<Rest> BoolTutle for Tutle<(bool, Tutle<Rest>)>
where Tutle<Rest>: BoolTutle {
    fn any(&self) -> bool {
        let Tutle((value, rest)) = self;
        *value || rest.any() 
    }

    fn all(&self) -> bool {
        let Tutle((value, rest)) = self;
        *value && rest.all()
    }
}


pub trait LazyBoolTutle<Args> {
    fn lazy_all(&mut self, arg: Args) -> bool;
    fn lazy_any(&mut self, arg: Args) -> bool;
}


impl<Args> LazyBoolTutle<Args> for Tutle<()> 
{
    fn lazy_all(&mut self, _args: Args) -> bool {
        true
    }

    fn lazy_any(&mut self, _args: Args) -> bool {
        false
    }
}

impl<Args: Tuple + Copy, F, Rest> LazyBoolTutle<Args> for Tutle<(F, Rest)> 
where F: FnMut<Args, Output=bool>, Rest: LazyBoolTutle<Args>
{
    fn lazy_all(&mut self, args: Args) -> bool {
        let Tutle((f, rest)) = self;
        if !rest.lazy_all(args) {
            false
        } else {
            f.call_mut(args)
        }
    }

    fn lazy_any(&mut self, args: Args) -> bool {
        let Tutle((f, rest)) = self;
        if rest.lazy_any(args) {
            true
        } else {
            f.call_mut(args)
        }
    }
}
