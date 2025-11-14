#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(test)]
#![allow(non_camel_case_types)]
#![feature(generic_const_exprs)]

extern crate test;

use std::hint::black_box;

use test::Bencher;

const N: usize = 5;
const A1: [&[f64]; N] = [
    &[],
    &[0.5],
    &[0., 0.5],
    &[0.0, 0.0, 1.0],
    &[5. / 32., 7. / 32., 13. / 32., -1. / 32.],
];

const A2: [[f64; N]; N] = [
    [0., 0., 0., 0., 0.],
    [0.5, 0., 0., 0., 0.],
    [0., 0.5, 0., 0., 0.],
    [0.0, 0.0, 1.0, 0., 0.],
    [5. / 32., 7. / 32., 13. / 32., -1. / 32., 0.],
];

const A3: [f64; N * (N - 1) / 2] = [
    0.5,
    //
    0.,
    0.5,
    //
    0.0,
    0.0,
    1.0,
    //
    5. / 32.,
    7. / 32.,
    13. / 32.,
    -1. / 32.,
    //
];

#[bench]
fn array_of_slices(b: &mut Bencher) {
    b.iter(|| {
        let a = black_box(A1);
        let mut sum = 0.;
        for i in 0..N {
            for j in 0..i {
                sum += a[i][j];
            }
        }
        black_box(sum);
    })
}

#[bench]
fn array_of_arrays(b: &mut Bencher) {
    b.iter(|| {
        let a = black_box(A2);
        let mut sum = 0.;
        for i in 0..N {
            for j in 0..i {
                sum += a[i][j];
            }
        }
        black_box(sum);
    })
}

#[bench]
fn array_flatten(b: &mut Bencher) {
    b.iter(|| {
        let a = black_box(A3);
        let mut sum = 0.;
        let mut ai = 0;
        for i in 0..N {
            for j in 0..i {
                sum += a[ai + j];
            }
            ai += i;
        }
        black_box(sum);
    })
}

#[bench]
fn baseline(b: &mut Bencher) {
    b.iter(|| {
        let a = black_box(A3);
        let mut sum = 0.;
        let mut ai = 0;
        for i in 0..N {
            for j in 0..i {
                sum += 1.;
            }
        }
        black_box(sum);
    })
}

#[bench]
fn array_flatten_single_loop(b: &mut Bencher) {
    b.iter(|| {
        let a = black_box(A3);
        let mut sum = 0.;
        for i in 0..(N*(N-1)/2) {
            sum += a[i];
        }
        black_box(sum);
    })
}
