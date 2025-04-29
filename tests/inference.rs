#![feature(unboxed_closures)]


#[test]
fn const_parameter_inference() {
    fn make_array<const N: usize>() -> [u8; N] {
        [0; N]
    }
    let arr: [u8; 7] = make_array(); // the size is inferred
    assert_eq!(arr, [0; 7]);
}

#[test]
fn closure_tuple_dispatch() {
    let f = |t, (x, y, z)| (t, x, y, z);

    assert_eq!(f(0, (1, 2, 3)), (0, 1, 2, 3));
}

#[test]
fn closure_array_dispatch() {
    let f = |t, [x, y, z]: [i32; 3]| (t, x, y, z); // cannot infer type without annotation
    assert_eq!(f(0, [1, 2, 3]), (0, 1, 2, 3));
}

#[test]
fn closure_array_dispatch_type_inferred() {
    fn expects_array_closure<const N: usize, Ret>(f: &impl Fn(f64, [i32; N]) -> Ret) -> Ret {
        f(42., [0; N])
    }

    assert_eq!(
        expects_array_closure(&|t, [x, y, z]| { (t, x, y, z) }),
        (42., 0, 0, 0)
    );
}

#[test]
fn closure_mut_array_dispatch() {
    let f = |t: &mut f64, [_, _, z]: &mut [i32; 3]| {
        *t += 1.;
        *z += 2;
    };
    let mut t = 0.;
    let mut x = [1, 2, 3];

    f(&mut t, &mut x);
    f(&mut t, &mut x);

    assert_eq!(t, 2.);
    assert_eq!(x, [1, 2, 7]);
}

#[test]
fn closure_mut_array_dispatch_type_inference() {
    fn expects_mut_array_closure<const N: usize>(f: &impl Fn(&mut [i32; N])) -> [i32; N] {
        let mut x = [0; N];
        f(&mut x);
        f(&mut x);
        x
    }

    assert_eq!(
        expects_mut_array_closure(&|[_, _, z]| {
            *z += 2;
        }),
        [0, 0, 4]
    );
}

#[test]
fn multiple_closure_inference() {
    // /* Conflicting implementation error */
    // trait Acceptable {}
    // impl<F> Acceptable for F where F: Fn(f64) {}
    // impl<F> Acceptable for F where F: Fn(f64, (u8, i32)) {}

    trait Acceptable<Args> {}
    impl<F> Acceptable<(f64,)> for F where F: Fn<(f64,)> {}
    impl<F> Acceptable<(f64, (u8, i32))> for F where F: Fn<(f64, (u8, i32))> {}


    struct Callable<F>{f: F}
    impl<F> Callable<F> {
        fn new<Arg>(f: F) -> Self where F: Acceptable<Arg> {
            Self {f}
        }
    }

    let c = Callable::new(|t| println!("{t}!"));
    (c.f)(42.);
    // (c.f)(42); // error: unsatisfied trait bound (as should)

    // let c = Callable::new(|(a,b,c)| println!("{a} and {b} and {c}")); // error (as should)
    
    // let c = Callable::new(|t, (a, b)| println!("{t} ==> {a}_{b}"));
    
}
