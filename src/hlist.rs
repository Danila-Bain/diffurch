pub struct Nil;
pub struct Cons<H, T>(pub H, pub T);

pub trait HList {
    const LEN: usize;

    fn len(&self) -> usize {
        Self::LEN
    }

    fn append<H>(self, h: H) -> Cons<H, Self>
    where
        Self: Sized,
    {
        Cons(h, self)
    }

    fn is_empty(&self) -> bool {
        Self::LEN == 0
    }
}

impl HList for Nil {
    const LEN: usize = 0;
}

impl<H, T> HList for Cons<H, T>
where
    T: HList,
{
    const LEN: usize = 1 + T::LEN;
}

// impl Iterator for Cons<H, T>

// pub trait Homogeneous<Item>: HList {
//     fn to_array(self) -> [Item; Self::LEN];
// }
// impl<Item> Homogeneous<Item> for Nil {
//     fn to_array(self) -> [Item; 0] {
//         []
//     }
// }
// impl<Item, T> Homogeneous<Item> for Cons<Item, T>
// where
//     T: Homogeneous<Item>,
// {
//     fn to_array(self) -> [T; Self::LEN] {
//         let Cons(h, t) = self;
//         concat([h], t.to_array())
//     }
// }


// trait TraitA {
//     fn a(&mut self, other: f64) -> usize;
// }
//
// trait TraitAHList {
//     fn a(&mut self, other: f64) -> impl HList;
// }
//
// impl TraitAHList for Nil {
//     fn a(&mut self, _: f64) -> impl HList {
//         Nil
//     }
// }
//
// impl<H, T> TraitAHList for Cons<H, T>
// where
//     H: TraitA,
//     T: TraitAHList,
// {
//     fn a(&mut self, other: f64) -> impl HList {
//         let Cons(h, t) = self;
//         Cons(h.a(other), t.a(other))
//     }
// }
