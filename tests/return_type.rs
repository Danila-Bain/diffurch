#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

trait WrapperTrait<'a> {
    type WrapperIterator: Iterator<Item = u32> + 'a;
    fn iter(&'a self) -> Self::WrapperIterator;
}

struct Wrapper {
    data: Vec<u32>,
}

impl<'a> WrapperTrait<'a> for Wrapper {
    type WrapperIterator = impl Iterator<Item = u32> + 'a;
    fn iter(&'a self) -> Self::WrapperIterator {
        self.data.iter().map(|&x| x)
    }
}


trait ToDoubleFunction {
    type DF: Fn();
    fn to_double_function(self) -> Self::DF;
}

impl<F> ToDoubleFunction for F where F: Fn() {
    type DF = impl Fn();

    fn to_double_function(self) -> Self::DF {
        move || { println!("Twice:"); self(); self(); }
    }
}

#[test]
fn test() {
    let print_fn = || println!("Hello! ");

    print_fn();

    let print_double_fn = print_fn.to_double_function();

    print_double_fn();

    // panic!();
}
