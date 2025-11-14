# `TraitHList` 

`TraitHList` is a macro for `hlist2` crate (compile-time heterogeneous lists).

This crate defines a procedural macro `TraitHList!` that automatically generates trait implementations
for [heterogeneous lists](https://docs.rs/hlist2/latest/hlist2/macro.hlist.html)
from the [`hlist2`](https://docs.rs/hlist2) crate.

It allows you to apply any trait’s methods *element-wise* across all items in an `hlist2::hlist!` object,
with support of generic parameters and lifetimes.

##  Features

- 🟢 Automatically implements a new `*_HList` trait corresponding to any existing trait.  
- 🟢 Supports traits with generic parameters, lifetimes, and `where` clauses.  
- 🟢 Handles methods with all receiver types: `self`, `&self`, and `&mut self`.  
- 🟢 Works with arbitrary argument and return types.  
- ⚠️ Methods returning `bool` automatically gain `.all_*()` and `.any_*()` variants.  
- 🟡 Method renaming supported with `#[name = "custom_name"]`.  

## Example

```rust
use hlist2::hlist;
use hlist2_trait_macro::TraitHList;

trait Check {
    fn is_integer(&self) -> bool;
    fn is_positive(&self) -> bool;
}

impl Check for i32 {
    fn is_integer(&self) -> bool{
        true
    }
    fn is_positive(&self) -> bool {
        *self > 0
    }
}
impl Check for f64 {
    fn is_integer(&self) -> bool{
        false
    }
    fn is_positive(&self) -> bool {
        *self > 0.0
    }
}

TraitHList!{
    CheckHList for trait Check {
        fn is_integer(&self) -> bool;

        #[name = are_positive]
        fn is_positive(&self) -> bool;
        
    }
}

fn main() {
    let xs = hlist![1, -2, 3, -5.0, 6.0, -7.0];

    assert_eq!(xs.is_integer(), hlist![true, true, true, false, false, false]);
    assert!(xs.all_is_integer() == false);
    assert!(xs.any_is_integer() == true);
    
    assert_eq!(xs.are_positive(), hlist![true, false, true, false, true, false]);
    assert!(xs.all_are_positive() == false);
    assert!(xs.any_are_positive() == true);
}
```

## Why?

Current implementation for iteration of heterogeneous collections like implemented in [`hlist2`] 
is slow and limited. The alternative approach with `Vec<Box<dyn Trait>>` objects has overhead at runtime.
Implementation of [`TraitHList!`] acts like an unrolled version of loops over static [`hlist2::hlist!`].
Custom traits with methods accepting mutable references can achieve many things that are also possible 
with regular loops, but do not require objects to have the same type. This is especially useful
for types, that rely on anonymous functions without dynamic dispatch with `dyn` traits.

```rust
use hlist2::hlist;
use hlist2_trait_macro::TraitHList;

trait AddOne {
    fn add_one(&self, x: &mut usize);
}

impl AddOne for () {
    fn add_one(&self, x: &mut usize) {
       *x += 1;
    }
}

TraitHList!{
    AddOneHList for trait AddOne {
        fn add_one(&self, x: &mut usize);
    }
}

let mut x = 0;
let list = hlist![(), (), (), (), ()];
list.add_one(&mut x);
assert!(x == 5);
```



Macro, that generates trait implementations for heterogeneous lists
whose elements share provided trait.

The `TraitHList!` macro automatically generates trait implementations 
for heterogeneous lists (`hlist2::hlist!`), allowing 
trait methods to be applied element-wise across all list elements. 

It supports traits with arbitrary generics, lifetimes, const parameters,
and `where` clauses, as well as methods with any receiver form 
(`self`, `&self`, `&mut self`) and arbitrary parameter types.

The macro defines a new trait (e.g. `MyTraitHlist`) mirroring 
the methods of the original one (e.g. `MyTrait`). Implemented
of types that implement the source trait. Each listed method produces
an `hlist2::hlist!` of results, preserving element order. 
Methods returning `bool` automatically gain two aggregators: 
`.all_<method>()` and `.any_<method>()`. 

Individual methods can be renamed with `#[name = ...]`.

In essence, `TraitHList!` extends any trait to operate 
seamlessly over heterogeneous lists, as a replacement for lacking 
iteration capabilities.


## Basic Usage

```rust,ignore
use hlist2_trait_macro::TraitHList;

TraitHList!{
    HListTraitName for trait TraitName<...> where ... { 
        // methods...
    }
};
```

```rust
use hlist2::hlist;
use hlist2_trait_macro::TraitHList;

trait MyTrait {
    fn to_u32(&self) -> u32;
    fn to_bool(&self) -> bool;
}

impl MyTrait for bool {
    fn to_u32(&self) -> u32 { *self as u32 }
    fn to_bool(&self) -> bool { *self }
}

impl MyTrait for i8 {
    fn to_u32(&self) -> u32 { *self as u32 }
    fn to_bool(&self) -> bool { *self != 0 }
}

TraitHList!(
    MyTraitHList for trait MyTrait {
        fn to_u32(&self) -> u32;
        fn to_bool(&self) -> bool;
    }
);

let l = hlist![false, true, 0, 10];
assert_eq!(hlist![0, 1, 0, 10], l.to_u32());
assert_eq!(hlist![false, true, false, true], l.to_bool());
assert!(!l.all_to_bool());
assert!(l.any_to_bool());
```

- The macro defines a trait `MyTraitHList` and implements it 
  for all `hlist!` combinations of types that implement `MyTrait`.
- Each method in the `MyTraitHList` acts **elementwise** on the list:
  - `l.to_u32()` calls `to_u32()` on each element.
  - `l.to_bool()` does the same.
- For methods that return bool, macro also provides:
  - `.all_<method>()` — returns `true` if all results are `true`.
  - `.any_<method>()` — returns `true` if any result is `true`.

  `.all_*` and `.any_*` methods are lazily evaluated from head to tail.

## Renaming Methods

Each method can be renamed in the HList version 
using attribute `#[name = ...]`, which can be
usefull to avoid naming collisions.
```rust
use hlist2::hlist;
use hlist2_trait_macro::TraitHList;

TraitHList! {
    IntoHlist for trait Into<T> {
        #[name = hlist_into]
        fn into(self) -> T;
    }
}

let list = hlist![true, 8u8, 16u16, 32u32];
assert_eq!(hlist![1u64, 8u64, 16u64, 32u64], list.hlist_into());
```


This generates a method `hlist_into` instead of the default `into`.

## Generic Traits
raitHList
```rust
use hlist2::hlist;
use hlist2_trait_macro::TraitHList;
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
```

Generated methods will operate on `hlist!`s of arrays `[i64; N]` with consistent `N`.


Also note, that parameters passed by value must implement either `Copy` or `Clone`, 
because they are passed to each element of the list.

## Comments and Unused Methods

Any methods omitted in the macro definition are ignored.  
Comments are safely ignored as well.

---

## Summary of Features

| Feature                            | Supported | Description |
|------------------------------------|------------|--------------|
| Elementwise trait method calls     | ✅ | Applies trait methods to each list element |
| Arbitrary trait-level generics and bounds | ✅ | Generic, const, lifetime parameters |
| Trait-level `where` clauses        | ✅ | Fully supported |
| Arbitrary method-level generics and bounds | ⚠️ | Generic lifetimes introduce additional explicit lifetime bounds |
| Method-level `where` clauses             | ✅ | Fully supported  |
| Different receiver forms           | ✅ | `self`, `&self`, `&mut self` |
| Method renaming                    | ✅ | `#[name = ...]` attribute |
| Additional convenience methods     | ✅ | `any_*`, `all_*` for `bool`-returning methods |
| Comments in macro body             | ✅ | Ignored |
| Associated types in traits | ⛔ | Not planned until usecase if found |
