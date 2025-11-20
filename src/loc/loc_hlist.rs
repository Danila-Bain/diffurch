use hlist2_trait_macro::TraitHList;
use num::Float;

use crate::{initial_condition::InitialCondition, loc::locate::Locate, state::State};

// pub trait LocateFirst<const N: usize, T>: Locate<N, T> {
//     fn locate_first<const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
//         &mut self,
//         state: &State<N, S, S2, T, IC>,
//         other_t: &mut Option<T>
//     );
// }
//
// impl<const N: usize, T: Float, L: Locate<N, T>> LocateFirst<N, T> for L {
//     fn locate_first<const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
//         &mut self,
//         state: &State<N, S, S2, T, IC>,
//         other_option_t: &mut Option<T>
//     ) {
//         if let Some(self_t) = self.locate(state) {
//             if let Some(other_t) = other_option_t {
//                 *other_option_t = Some(self_t.min(*other_t));
//             } else {
//                 *other_option_t = Some(self_t);
//             }
//         }
//     }
// }
//

pub trait LocateEarliestImpl<const N: usize, T> {
    fn locate_earliest_impl<const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &State<N, S, S2, T, IC>,
        self_index: &mut usize,
        earliest_index: &mut usize,
        earliest_time: &mut Option<T>,
    );
}

impl<const N: usize, T: Float, L: Locate<N, T>> LocateEarliestImpl<N, T> for L {
    fn locate_earliest_impl<const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &State<N, S, S2, T, IC>,
        self_index: &mut usize,
        earliest_index: &mut usize,
        earliest_time: &mut Option<T>,
    ) {
        if let Some(self_time) = self.locate(state)
            && earliest_time.is_none_or(|t| self_time < t)
        {
            *earliest_index = *self_index;
            *earliest_time = Some(self_time);
        }
        *self_index += 1;
    }
}


TraitHList! {
    pub HListLocateEarliestImpl for trait LocateEarliestImpl<const N: usize, T> {
        fn locate_earliest_impl<const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
            &mut self,
            state: &State<N, S, S2, T, IC>,
            self_index: &mut usize,
            earliest_index: &mut usize,
            earliest_time: &mut Option<T>,
        );
    }
}

pub trait HListLocateEarliest<const N: usize, T> {
    fn locate_earliest<const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &State<N, S, S2, T, IC>,
    ) -> Option<(usize, T)>;
}

impl<const N: usize, T, U: HListLocateEarliestImpl<N, T>> HListLocateEarliest<N, T> for U {
    fn locate_earliest<const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &State<N, S, S2, T, IC>,
    ) -> Option<(usize, T)> {
        let mut index = 0;
        let mut earliest_time = None;
        self.locate_earliest_impl(state, &mut 0, &mut index, &mut earliest_time);
        let Some(earliest_time) = earliest_time else {return None};
        return Some((index, earliest_time));
    }
}
