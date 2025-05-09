// #![feature(test)]
// extern crate test;
// use test::Bencher;
//
// macro_rules! polynomial {
//     ($($coef:expr),+ $(,)?) => {
//         |_t| { polynomial!(_t => $($coef),+) }
//     };
//     ($t:ident => $a:expr) => { $a };
//     ($t:ident => $a:expr, $($rest:expr),+) => {
//         $a + $t * (polynomial!($t => $($rest),+))
//     };
// }
//
// macro_rules! polynomial_fold {
//     () => {
//         |_t: f64| { 0. }
//     };
//     ($($coef:expr),+ $(,)?) => {
//         |t: f64| {
//             [$($coef),+].into_iter().rev()
//             .reduce(|acc: f64, c: f64| c + t * acc).expect("coefficient array is non-empty")
//         }
//     };
// }
//
// // 1. + t * (2. + t * (3.))
// // 0. + t * (0. + t * (0.))
//
// #[bench]
// fn base(b: &mut Bencher) {
//     let mut t = 0.;
//     b.iter(|| {
//         t += 0.01;
//     })
// }
//
// #[bench]
// fn macro_recursion_polynomial(b: &mut Bencher) {
//     let p = polynomial![-1., 0., 1.];
//     let mut t = 0.;
//     b.iter(|| {
//         t += 0.01;
//         p(t)
//     })
// }
//
// #[bench]
// fn macro_fold_polynomial(b: &mut Bencher) {
//     let p = polynomial_fold![-1., 0., 1.];
//     let mut t = 0.;
//     b.iter(|| {
//         t += 0.01;
//         p(t)
//     })
// }
//
// #[bench]
// fn inline_polynomial(b: &mut Bencher) {
//     let mut t = 0.;
//     b.iter(|| {
//         t += 0.01;
//         t * t - 1.
//     })
// }
//
// #[bench]
// fn closure_polynomial(b: &mut Bencher) {
//     let p = |t| t * t - 1.;
//     let mut t = 0.;
//     b.iter(|| {
//         t += 0.01;
//         p(t)
//     })
// }
//
// #[bench]
// fn closure_pow_polynomial(b: &mut Bencher) {
//     let p = |t: f64| t.powi(2) - 1.;
//     let mut t = 0.;
//     b.iter(|| {
//         t += 0.01;
//         p(t)
//     })
// }
//
// struct PolynomialArray<const N: usize> {
//     coefficients: [f64; N],
// }
// impl<const N: usize> PolynomialArray<N> {
//     fn new(coefficients: [f64; N]) -> Self {
//         Self { coefficients }
//     }
//
//     fn eval_fold(&self, t: f64) -> f64 {
//         self.coefficients
//             .iter()
//             .rev()
//             .fold(0., |acc, c| c + t * acc)
//     }
//
//     fn eval_loop_index(&self, t: f64) -> f64 {
//         let mut result = 0.;
//         for i in (0..self.coefficients.len()).rev() {
//             result = self.coefficients[i] + result * t;
//         }
//         result
//     }
//
//     fn eval_loop_iter(&self, t: f64) -> f64 {
//         let mut result = 0.;
//         for c in self.coefficients.iter().rev() {
//             result = c + result * t;
//         }
//         result
//     }
// }
//
// #[bench]
// fn struct_polynomial_assertions(b: &mut Bencher) {
//     let p0 = polynomial![-1., 0., 1.];
//     let p1 = PolynomialArray::new([-1., 0., 1.]);
//     let p2 = polynomial_fold![-1., 0., 1.];
//     let mut t = 0.;
//     b.iter(|| {
//         t += 0.01;
//         let val = p0(t);
//         assert_eq!(val, p1.eval_fold(t));
//         assert_eq!(val, p1.eval_loop_index(t));
//         assert_eq!(val, p1.eval_loop_iter(t));
//         assert_eq!(val, p2(t));
//     })
// }
//
// #[bench]
// fn struct_polynomial_fold(b: &mut Bencher) {
//     let p = PolynomialArray::new([-1., 0., 1.]);
//     let mut t = 0.;
//     b.iter(|| {
//         t += 0.01;
//         p.eval_fold(t)
//     })
// }
//
// #[bench]
// fn struct_polynomial_fold_zero_bloat(b: &mut Bencher) {
//     let p = PolynomialArray::new([
//         -1., 0., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
//         0., 0.,
//     ]);
//     let mut t = 0.;
//     b.iter(|| {
//         t += 0.01;
//         p.eval_fold(t)
//     })
// }
//
// #[bench]
// fn struct_polynomial_fold_zero_bloat_minuses(b: &mut Bencher) {
//     let p = PolynomialArray::new([
//         -1., 0., 1., -0., -0., -0., -0., -0., -0., -0., -0., -0., -0., -0., -0., -0., -0., -0.,
//         -0., -0., -0., -0., -0., -0.,
//     ]);
//     let mut t = 0.;
//     b.iter(|| {
//         t += 0.01;
//         p.eval_fold(t)
//     })
// }
//
// #[bench]
// fn struct_polynomial_loop_index(b: &mut Bencher) {
//     let p = PolynomialArray::new([-1., 0., 1.]);
//     let mut t = 0.;
//     b.iter(|| {
//         t += 0.01;
//         p.eval_loop_index(t)
//     })
// }
//
// #[bench]
// fn struct_polynomial_loop_iter(b: &mut Bencher) {
//     let p = PolynomialArray::new([-1., 0., 1.]);
//     let mut t = 0.;
//     b.iter(|| {
//         t += 0.01;
//         p.eval_loop_iter(t)
//     })
// }
//
// #[bench]
// fn macro_full_polynomial(b: &mut Bencher) {
//     let p = polynomial![-1., -2., 1.];
//     let mut t = 0.;
//     b.iter(|| {
//         t += 0.01;
//         p(t)
//     })
// }
//
// #[bench]
// fn macro_polynomial_zero_bloat(b: &mut Bencher) {
//     let p = polynomial![
//         -1., 0., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
//         0., 0.
//     ];
//     let mut t = 0.;
//     b.iter(|| {
//         t += 0.01;
//         p(t)
//     })
// }
//
// #[bench]
// fn macro_polynomial_zero_bloat_minuses(b: &mut Bencher) {
//     let p = polynomial![
//         -1., 0., 1., -0., -0., -0., -0., -0., -0., -0., -0., -0., -0., -0., -0., -0., -0., -0.,
//         -0., -0., -0., -0., -0., -0.
//     ];
//     let mut t = 0.;
//     b.iter(|| {
//         t += 0.01;
//         p(t)
//     })
// }
//
// struct PolynomialWrappwer(fn(f64) -> f64);
//
// #[bench]
// fn wrapper_macro_polynomial(b: &mut Bencher) {
//     let p = PolynomialWrappwer(polynomial![-1., -2., 1.]);
//     let mut t = 0.;
//     b.iter(|| {
//         t += 0.01;
//         p.0(t)
//     })
// }
//
// /* Take-aways:
//  *
//  * closures are faster than loops, can be staked together with different sizes of polynomials
//  * without additional overhead of trailing zeros. Also, there is a possibility in the future, to
//  * remove trailing zeros by modifying a macro.
//  *
//  * -0. addition is optimized by compiler, but +0. is not,
//  * multiplication by 0. or -0. is not optimized by compiler.
//  * but I found the impact of using -0. instead of 0. or removing 0s in macro inconsistent.
//  *
//  * wrapper around closure is ok
//  *
//  *
//  */
