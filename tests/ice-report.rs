// #![feature(unboxed_closures)]
//
// #[test]
// fn main() {
//
//     trait Acceptable<Args> {}
//     impl<F> Acceptable<(f64,)> for F where F: Fn<(f64,)> {}
//
//     fn take_and_return_acceptable_closure<F, Args>(closure: F) -> F
//     where F: Acceptable<Args>, {
//         closure
//     }
//
//
//     // OK
//     // let _closure_back = take_and_return_acceptable_closure(|arg: f64| println!("{arg}"));
//
//     // OK
//     // let closure_back = take_and_return_acceptable_closure(|arg| println!("{arg}"));
//     // closure_back(1.);
//     //
//
//     // ICE
//     // let _closure_back = take_and_return_acceptable_closure(|arg| println!("{arg}"));
// }
