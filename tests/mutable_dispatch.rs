fn take([x, y]: &mut [f64; 2]) {
    *x *= 2.;
    *y /= 2.;
}

#[test]
fn main() {
   let mut p = [1.,1.];
   take(&mut p);
   assert_eq!(p, [2., 0.5])

}
