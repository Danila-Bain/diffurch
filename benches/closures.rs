#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(test)]
#![allow(non_camel_case_types)]

/*
Output (sorted manually):

test bench_none                                        ... bench:           0.35 ns/iter (+/- 0.07)

test bench_single_closure                              ... bench:           1.39 ns/iter (+/- 0.22)
test bench_single_expression                           ... bench:           1.39 ns/iter (+/- 0.67)
test bench_single_function                             ... bench:           1.40 ns/iter (+/- 0.22)
test bench_single_struct_fn_trait                      ... bench:           1.39 ns/iter (+/- 0.20)
test bench_single_struct_method                        ... bench:           1.39 ns/iter (+/- 0.39)
test bench_single_boxed_closure                        ... bench:           2.43 ns/iter (+/- 0.07)

test bench_collection_small_hlist_of_closures_map      ... bench:           1.71 ns/iter (+/- 0.19)
test bench_collection_small_array_of_closure1          ... bench:           1.74 ns/iter (+/- 0.09)
test bench_collection_small_vec_of_closure1            ... bench:           3.82 ns/iter (+/- 0.09)
test bench_collection_small_hlist_of_closure1_for_loop ... bench:           4.17 ns/iter (+/- 0.55)
test bench_collection_small_array_of_boxed_closures    ... bench:          10.45 ns/iter (+/- 5.56)
test bench_collection_small_vec_of_boxed_closures      ... bench:          11.48 ns/iter (+/- 1.28)

test bench_collection_large_hlist_of_closure1_map      ... bench:          34.77 ns/iter (+/- 5.30)
test bench_collection_large_array_of_closure1          ... bench:          41.76 ns/iter (+/- 3.44)
test bench_collection_large_vec_of_closure1            ... bench:          77.72 ns/iter (+/- 13.41)
test bench_collection_large_vec_of_boxed_closures      ... bench:         215.16 ns/iter (+/- 80.84)
test bench_collection_large_array_of_boxed_closures    ... bench:         250.33 ns/iter (+/- 120.53)
test bench_collection_large_hlist_of_closure1_for_loop ... bench:       2,897.13 ns/iter (+/- 1,457.15)
 */

/*
Conclusion:
Single calls:
    In those simple cases, the function calles of regular functions,
    struct methods, closures, and Fn implementations are expectedly the same.

    Boxed closures, however, have an overhead around 2.43 - 1.39 = 1.04 ns/call,
    which is like 3 times the overhead of the empty loop iteration (0.35 ns) or about the
    same that the regular function call (1.39 - 0.35 = 1.04 ns/call). It is not much, but for small
    functions boxed version takes about 2 times longer than the regular function.

Collections of functions:
    Funny enough, hlist show both the best and the worst performance in those benchmarks.
*/

extern crate test;

use test::Bencher;

use hlist2::*;

macro_rules! function_body {
    ($t:expr) => {
        $t * $t - $t + 1.
    };
}

const N_S: usize = 5;
const N_L: usize = 100;

fn function(t: f64) -> f64 {
    function_body!(t)
}

struct struct_method;
impl struct_method {
    pub fn eval(&self, t: f64) -> f64 {
        function_body!(t)
    }
}

struct struct_fn_trait;
impl FnOnce<(f64,)> for struct_fn_trait {
    type Output = f64;
    extern "rust-call" fn call_once(self, arg: (f64,)) -> Self::Output {
        function_body!(arg.0)
    }
}

impl FnMut<(f64,)> for struct_fn_trait {
    #[inline]
    extern "rust-call" fn call_mut(&mut self, arg: (f64,)) -> Self::Output {
        function_body!(arg.0)
    }
}

impl Fn<(f64,)> for struct_fn_trait {
    extern "rust-call" fn call(&self, arg: (f64,)) -> Self::Output {
        function_body!(arg.0)
    }
}

fn closure_1() -> impl Copy + Fn(f64) -> f64 {
    |t: f64| function_body!(t)
}
fn closure_2() -> impl Copy + Fn(f64) -> f64 {
    |t: f64| function_body!(t)
}
fn closure_3() -> impl Copy + Fn(f64) -> f64 {
    |t: f64| function_body!(t)
}
fn closure_4() -> impl Copy + Fn(f64) -> f64 {
    |t: f64| function_body!(t)
}
fn closure_5() -> impl Copy + Fn(f64) -> f64 {
    |t: f64| function_body!(t)
}

#[bench]
fn bench_none(b: &mut Bencher) {
    b.iter(|| {})
}

#[bench]
fn bench_single_expression(b: &mut Bencher) {
    b.iter(|| {
        let t = test::black_box(0.);
        function_body!(t)
    })
}
#[bench]
fn bench_single_function(b: &mut Bencher) {
    b.iter(|| {
        let t = test::black_box(0.);
        function(t)
    })
}
#[bench]
fn bench_single_closure(b: &mut Bencher) {
    let f = closure_1();
    b.iter(|| {
        let t = test::black_box(0.);
        f(t)
    })
}
#[bench]
fn bench_single_boxed_closure(b: &mut Bencher) {
    let f: Box<dyn Fn(f64) -> f64> = Box::new(closure_1());
    b.iter(|| {
        let t = test::black_box(0.);
        f(t)
    })
}
#[bench]
fn bench_single_struct_method(b: &mut Bencher) {
    let f = struct_method;
    b.iter(|| {
        let t = test::black_box(0.);
        f.eval(t)
    })
}
#[bench]
fn bench_single_struct_fn_trait(b: &mut Bencher) {
    let f = struct_fn_trait;
    b.iter(|| {
        let t = test::black_box(0.);
        f(t)
    })
}
#[bench]
fn bench_collection_small_array_of_closure1(b: &mut Bencher) {
    let a: [_; N_S] = std::array::from_fn(|_| closure_1());
    b.iter(|| {
        for f in a.iter() {
            let t = test::black_box(0.);
            f(t);
        }
    })
}
#[bench]
fn bench_collection_large_array_of_closure1(b: &mut Bencher) {
    let a: [_; N_L] = std::array::from_fn(|_| closure_1());
    b.iter(|| {
        for f in a.iter() {
            let t = test::black_box(0.);
            f(t);
        }
    })
}
#[bench]
fn bench_collection_small_array_of_boxed_closures(b: &mut Bencher) {
    let a: [Box<dyn Fn(f64) -> f64>; N_S] = std::array::from_fn(|i| {
        let b: Box<dyn Fn(f64) -> f64> = match i % 5 {
            0 => Box::new(closure_1()),
            1 => Box::new(closure_2()),
            2 => Box::new(closure_3()),
            3 => Box::new(closure_4()),
            4 => Box::new(closure_5()),
            _ => unreachable!(),
        };
        b
    });
    b.iter(|| {
        for f in a.iter() {
            let t = test::black_box(0.);
            f(t);
        }
    })
}

#[bench]
fn bench_collection_large_array_of_boxed_closures(b: &mut Bencher) {
    let a: [Box<dyn Fn(f64) -> f64>; N_L] = std::array::from_fn(|i| {
        let b: Box<dyn Fn(f64) -> f64> = match i % 5 {
            0 => Box::new(closure_1()),
            1 => Box::new(closure_2()),
            2 => Box::new(closure_3()),
            3 => Box::new(closure_4()),
            4 => Box::new(closure_5()),
            _ => unreachable!(),
        };
        b
    });
    b.iter(|| {
        for f in a.iter() {
            let t = test::black_box(0.);
            f(t);
        }
    })
}

#[bench]
fn bench_collection_small_vec_of_closure1(b: &mut Bencher) {
    let a: Vec<_> = (0..N_S).map(|_| closure_1()).collect();
    b.iter(|| {
        for f in a.iter() {
            let t = test::black_box(0.);
            f(t);
        }
    })
}
#[bench]
fn bench_collection_large_vec_of_closure1(b: &mut Bencher) {
    let a: Vec<_> = (0..N_L).map(|_| closure_1()).collect();
    b.iter(|| {
        for f in a.iter() {
            let t = test::black_box(0.);
            f(t);
        }
    })
}
#[bench]
fn bench_collection_small_vec_of_boxed_closures(b: &mut Bencher) {
    let a: Vec<Box<dyn Fn(f64) -> f64>> = (0..N_S)
        .map(|i| {
            let b: Box<dyn Fn(f64) -> f64> = match i % 5 {
                0 => Box::new(closure_1()),
                1 => Box::new(closure_2()),
                2 => Box::new(closure_3()),
                3 => Box::new(closure_4()),
                4 => Box::new(closure_5()),
                _ => unreachable!(),
            };
            b
        })
        .collect();
    b.iter(|| {
        for f in a.iter() {
            let t = test::black_box(0.);
            f(t);
        }
    })
}
#[bench]
fn bench_collection_large_vec_of_boxed_closures(b: &mut Bencher) {
    let a: Vec<Box<dyn Fn(f64) -> f64>> = (0..N_L)
        .map(|i| {
            let b: Box<dyn Fn(f64) -> f64> = match i % 5 {
                0 => Box::new(closure_1()),
                1 => Box::new(closure_2()),
                2 => Box::new(closure_3()),
                3 => Box::new(closure_4()),
                4 => Box::new(closure_5()),
                _ => unreachable!(),
            };
            b
        })
        .collect();
    b.iter(|| {
        for f in a.iter() {
            let t = test::black_box(0.);
            f(t);
        }
    })
}
#[bench]
fn bench_collection_small_hlist_of_closure1_for_loop(b: &mut Bencher) {
    let hl = hlist2::hlist![
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
    ];

    assert_eq!(N_S, hl.len());

    b.iter(|| {
        for f in &hl {
            let t = test::black_box(0.);
            f(t);
        }
    })
}

#[bench]
fn bench_collection_small_hlist_of_closures_map(b: &mut Bencher) {
    let hl = hlist2::hlist![
        closure_1(),
        closure_2(),
        closure_3(),
        closure_4(),
        closure_5(),
    ];

    assert_eq!(N_S, hl.len());

    use hlist2::ops::*;
    struct MyMapFn(f64);
    impl<T> MapFn<T> for MyMapFn
    where
        T: Fn(f64) -> f64,
    {
        type Output = f64;
        fn map(&mut self, f: T) -> Self::Output {
            f(self.0)
        }
    }

    b.iter(|| {
        let t = test::black_box(0.);
        hl.map(Mapper(MyMapFn(t)))
    })
}

#[bench]
fn bench_collection_large_hlist_of_closure1_for_loop(b: &mut Bencher) {
    let hl = hlist2::hlist![
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
        closure_1(),
    ];
    assert_eq!(N_L, hl.len());
    b.iter(|| {
        for f in &hl {
            let t = test::black_box(0.);
            f(t);
        }
    })
}

#[bench]
fn bench_collection_large_hlist_of_closure1_map(b: &mut Bencher) {
    let hl = hlist2::hlist![
        closure_1(),
        closure_2(),
        closure_3(),
        closure_4(),
        closure_5(),
        closure_1(),
        closure_2(),
        closure_3(),
        closure_4(),
        closure_5(),
        closure_1(),
        closure_2(),
        closure_3(),
        closure_4(),
        closure_5(),
        closure_1(),
        closure_2(),
        closure_3(),
        closure_4(),
        closure_5(),
        closure_1(),
        closure_2(),
        closure_3(),
        closure_4(),
        closure_5(),
        closure_1(),
        closure_2(),
        closure_3(),
        closure_4(),
        closure_5(),
        closure_1(),
        closure_2(),
        closure_3(),
        closure_4(),
        closure_5(),
        closure_1(),
        closure_2(),
        closure_3(),
        closure_4(),
        closure_5(),
        closure_1(),
        closure_2(),
        closure_3(),
        closure_4(),
        closure_5(),
        closure_1(),
        closure_2(),
        closure_3(),
        closure_4(),
        closure_5(),
        closure_1(),
        closure_2(),
        closure_3(),
        closure_4(),
        closure_5(),
        closure_1(),
        closure_2(),
        closure_3(),
        closure_4(),
        closure_5(),
        closure_1(),
        closure_2(),
        closure_3(),
        closure_4(),
        closure_5(),
        closure_1(),
        closure_2(),
        closure_3(),
        closure_4(),
        closure_5(),
        closure_1(),
        closure_2(),
        closure_3(),
        closure_4(),
        closure_5(),
        closure_1(),
        closure_2(),
        closure_3(),
        closure_4(),
        closure_5(),
        closure_1(),
        closure_2(),
        closure_3(),
        closure_4(),
        closure_5(),
        closure_1(),
        closure_2(),
        closure_3(),
        closure_4(),
        closure_5(),
        closure_1(),
        closure_2(),
        closure_3(),
        closure_4(),
        closure_5(),
        closure_1(),
        closure_2(),
        closure_3(),
        closure_4(),
        closure_5(),
    ];
    assert_eq!(N_L, hl.len());

    use hlist2::ops::*;
    struct MyMapFn(f64);
    impl<T> MapFn<T> for MyMapFn
    where
        T: Fn(f64) -> f64,
    {
        type Output = f64;
        fn map(&mut self, f: T) -> Self::Output {
            f(self.0)
        }
    }

    b.iter(|| {
        let t = test::black_box(0.);
        hl.map(Mapper(MyMapFn(t)))
    })
}
