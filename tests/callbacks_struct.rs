#![feature(unboxed_closures, fn_traits, tuple_trait)]

trait CallTower<Args>
{
    fn call_tower(&self, args: Args);
}

impl<Args> CallTower<Args> for ()
{
    #[inline]
    fn call_tower(&self, _arg: Args) {}
}

impl<Args, F, Tail> CallTower<Args> for (F, Tail)
where
    F: Fn<Args>,
    Tail: CallTower<Args>,
    Args: std::marker::Tuple + Copy
{
    fn call_tower(&self, args: Args) {
        let (f, tail) = self;
        f.call(args);
        tail.call_tower(args);
    }
}


#[test]
fn main() {

    let a = |t: &i32| { println!("{}", t) };
    let b = |t: &i32| { println!("{}", t+1) };
    let c = |t: &i32| { println!("{}", t+2) };

    let tower = (a, (b, (c, ())));

    let zero = 0;
    tower.call_tower((&zero,));
}

#[test]
fn generic_call() {
    fn call<F, Args>(f: F, args: Args) where F: Fn<Args>, Args: std::marker::Tuple + Copy {
       f.call(args);
       f.call(args);
    }

    let f = |t: &str| println!("{t}");
    call(f, ("Hello, world!",));


    let f = |a: i32, b: i32| println!("{a} and {b}");
    call(f, (1, 2));


    let f = |a: &i32, b: &i32| println!("{a} and {b}");
    call(f, (&1, &2));
    let (a,b) = (3,4);
    call(f, (&a, &b));


}

