#[test]
fn const_parameter_inference() {
    fn make_array<const N: usize>() -> [u8; N] {
        [0; N]
    }

    let arr: [u8; 7] = make_array(); // the size is inferred
    // println!("{:?}", arr);
    // panic!();
}
