// // conflicting implementations
// trait MyTrait {}
// impl<F: Fn(f64)> MyTrait for F {}
// impl<F: Fn(usize)> MyTrait for F {}

// non-conflicting implementations
trait MyTrait<T> {}
impl<F: Fn(f64)> MyTrait<f64> for F {}
impl<F: Fn(usize)> MyTrait<usize> for F {}
