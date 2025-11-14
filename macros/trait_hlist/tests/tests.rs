use hlist2::hlist;
use hlist2_trait_macro::TraitHList;

#[test]
fn simple_trait() {
    #[allow(dead_code)]
    trait MyTrait {
        fn to_u32(&self) -> u32;
        fn to_bool(&self) -> bool;
    }

    impl MyTrait for bool {
        fn to_u32(&self) -> u32 {
            *self as u32
        }
        fn to_bool(&self) -> bool {
            *self
        }
    }

    impl MyTrait for i32 {
        fn to_u32(&self) -> u32 {
            *self as u32
        }
        fn to_bool(&self) -> bool {
            *self != 0
        }
    }

    TraitHList!(
        // Comment is ignored
        MyTraitHlist for trait MyTrait {
            fn to_u32(&self) -> u32;
            fn to_bool(&self) -> bool;
        }
    );

    let l = hlist![false, true, 0, 10];
    assert_eq!(hlist![0, 1, 0, 10], l.to_u32());
    assert_eq!(hlist![false, true, false, true], l.to_bool());
    assert!(!l.all_to_bool());
    assert!(l.any_to_bool());
    assert!(!hlist![false, 0, false].any_to_bool());
    assert!(hlist![true, 1, true].all_to_bool());
}

#[test]
fn generic_into() {
    TraitHList! {
        IntoHlist for trait Into<T> {
            fn into(self) -> T;
        }
    }

    let list = hlist![true, 1u8, 1u16, 1u32];
    assert_eq!(hlist![1u64, 1u64, 1u64, 1u64], IntoHlist::into(list),);
    assert_eq!(hlist![1f64, 1f64, 1f64, 1f64], IntoHlist::into(list),);
}

#[test]
fn generic_into_renamed() {
    TraitHList! {
        IntoHlist for trait Into<T> {
            #[name = hlist_into]
            fn into(self) -> T;
        }
    }

    let list = hlist![true, 1u8, 1u16, 1u32];
    assert_eq!(hlist![1u64, 1u64, 1u64, 1u64], list.hlist_into(),);
    assert_eq!(hlist![1u32, 1u32, 1u32, 1u32], list.hlist_into(),);
    assert_eq!(hlist![1f64, 1f64, 1f64, 1f64], list.hlist_into(),);
}

#[test]
fn generic_trait_0() {
    trait MyTrait<const N: usize, T: Into<i64>> {
        fn a<U: Into<i64>>(&self, x: i64, y: U, z: T) -> bool;
        fn b(self, x: i64, y: &i64, z: T) -> bool;
    }

    impl<const N: usize, T: Into<i64>> MyTrait<N, T> for [i64; N] {
        fn a<U: Into<i64>>(&self, x: i64, y: U, z: T) -> bool {
            (self.into_iter().sum::<i64>() + x + y.into() + z.into()) == 0
        }

        fn b(self, x: i64, y: &i64, z: T) -> bool {
            (self.into_iter().sum::<i64>() + x + y + z.into()) == 0
        }
    }

    TraitHList! {
        MyTraitHlist for trait MyTrait<const N: usize, T: Into<i64>> {
            fn a<U: Into<i64>>(&self, x: i64, y: U, z: T) -> bool where T: Copy, U: Copy;
            fn b(self, x: i64, y: &i64, z: T) -> bool where T: Clone;
        }
    }

    // Note that size must be the same, because N is the parameter of the trait, not methods
    let h0 = hlist![[0; 4], [1; 4], [2; 4], [3; 4], [4; 4],];

    assert_eq!(
        hlist![false, true, false, false, false],
        h0.a(0i64, 4u32, -8i16)
    );
    assert_eq!(
        hlist![false, true, false, false, false],
        h0.b(0i64, &4i64, -8i16)
    );
}

#[test]
fn parameters_in_methods() {
    #[allow(dead_code)]
    trait MyTrait {
        fn a(&self, x: &f64, y: &mut u32, z: u8) -> u32;
        fn b(&mut self, x: &f64, y: &mut u32, z: u8) -> bool;
        fn c(self, x: &f64, y: &mut u32, z: u8) -> bool;
        fn extra_one_ignored(self) -> Self;
    }

    TraitHList!(
        MyTraitHlist for trait MyTrait {
            fn a(&self, x: &f64, y: &mut u32, z: u8) -> u32;
            fn b(&mut self, x: &f64, y: &mut u32, z: u8) -> bool;
            fn c(self, x: &f64, y: &mut u32, z: u8) -> bool;
        }
    );
}

#[test]
fn generic_and_references() {
    #[allow(dead_code)]
    trait MyTrait<T> {
        fn owned(self, x: T);
        fn borrowed(&self, x: &T);
        fn mut_borrowed<'a>(&'a mut self, x: &'a mut T);
    }
    TraitHList! {
        SwapHList for trait MyTrait<T> {
            fn owned(self, x: T) where T: Clone;
            fn borrowed(& self, x: & T);
            fn mut_borrowed<'a>(&'a mut self, x: &'a mut T) where Self: 'a, T: 'a;
        }
    }
}

#[test]
fn generic_parameters_in_methods() {
    #[allow(dead_code)]
    trait MyTrait {
        fn a(&self, x: impl Copy) -> u32;
        fn b<T>(&self, x: T) -> bool;
        fn c<T>(&self, x: T) -> bool
        where
            T: std::fmt::Display;
        fn extra_one_ignored(self) -> Self;
    }

    TraitHList!(
        MyTraitHlist for trait MyTrait {
            fn a(&self, x: impl Copy) -> u32;
            fn b<T: Copy>(&self, x: T) -> bool;
            fn c<T>(&self, x: T) -> bool
                where T: std::fmt::Display + Copy;
        }
    );
}

#[test]
fn generic_trait_1() {
    #[allow(dead_code)]
    trait MyTrait<'a, T> {
        fn a(&self, x: T) -> bool;
        fn b(&self, x: T) -> bool;
        fn extra_one_ignored(self) -> Self;
    }

    TraitHList!(
        MyTraitHlist for trait MyTrait<'a, T: std::fmt::Display> where T: Copy {
            fn a(&self, x: T) -> bool;
            fn b(&self, x: T) -> bool;
        }
    );
}

#[test]
fn generic_trait_2() {
    #[allow(dead_code)]
    trait MyTrait<'a, const N: usize, T> {
        fn a<'aa: 'a>(&'a self, x: &'aa T, y: [T; N]) -> bool;
        fn b(&self, x: T, y: [T; N]) -> bool;
        fn extra_one_ignored(self) -> Self;
    }

    TraitHList!(
        MyTraitHlist for trait MyTrait<'a, const N: usize, T: std::fmt::Display> where {
            fn a<'aa: 'a>(&'a self, x: &'aa T, y: [T; N]) -> bool where T: Copy, Self: 'aa, T: 'aa;
            fn b(&self, x: T, y: [T; N]) -> bool where T: Copy;
        }
    );
}

#[allow(dead_code)]
#[test]
fn generic_return_types() {

    trait MyTrait {
        type MyType<T> where T: Copy;
    }

    impl MyTrait for u64 {
        type MyType<T> = T where T: Copy;
    }

}
