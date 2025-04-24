pub trait CallTower<Args>
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
    #[inline]
    fn call_tower(&self, args: Args) {
        let (f, tail) = self;
        f.call(args);
        tail.call_tower(args);
    }
}




// trait TransformTower
// {
//     fn transform_tower<F>(&self, f: F);
// }
//
// impl TransformTower for () {
//     #[inline]
//     fn transform_tower<F>(&self, f: F) {}
// }
//
// impl<Head, Tail> TransformTower for (F, Tail) {
//     fn transform_tower<F>(&self, f: F) 
//     where
//         F: Fn(Head),
//         Tail: TransformTower,
//     {
//         let (head, tail) = self;
//         (f(head), tail.transform_tower(f))
//     }
// }
