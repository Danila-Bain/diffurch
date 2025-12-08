use crate::{
    initial_condition::InitialCondition, loc::locate::Locate, state::State, traits::RealVectorSpace,
};
use hlist2_trait_macro::TraitHList;
use nalgebra::RealField;

pub trait LocateEarliestImpl<T: RealField + Copy, Y: RealVectorSpace<T>> {
    fn locate_earliest_impl<const S: usize, const I: usize, IC: InitialCondition<T, Y>>(
        &mut self,
        state: &State<T, Y, S, I, IC>,
        self_index: &mut usize,
        earliest_index: &mut usize,
        earliest_time: &mut Option<T>,
    );
}

impl<T: RealField + Copy, Y: RealVectorSpace<T>, L: Locate<T, Y>> LocateEarliestImpl<T, Y> for L {
    fn locate_earliest_impl<const S: usize, const I: usize, IC: InitialCondition<T, Y>>(
        &mut self,
        state: &State<T, Y, S, I, IC>,
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
    pub HListLocateEarliestImpl for trait LocateEarliestImpl<T: RealField + Copy, Y: RealVectorSpace<T>> {
        fn locate_earliest_impl<const S: usize, const I: usize, IC: InitialCondition<T, Y>>(
            &mut self,
            state: &State<T, Y, S, I, IC>,
            self_index: &mut usize,
            earliest_index: &mut usize,
            earliest_time: &mut Option<T>,
        );
    }
}

pub trait HListLocateEarliest<T: RealField + Copy, Y: RealVectorSpace<T>> {
    fn locate_earliest<const S: usize, const I: usize, IC: InitialCondition<T, Y>>(
        &mut self,
        state: &State<T, Y, S, I, IC>,
    ) -> Option<(usize, T)>;
}

impl<T: RealField + Copy, Y: RealVectorSpace<T>, U: HListLocateEarliestImpl<T, Y>>
    HListLocateEarliest<T, Y> for U
{
    fn locate_earliest<const S: usize, const I: usize, IC: InitialCondition<T, Y>>(
        &mut self,
        state: &State<T, Y, S, I, IC>,
    ) -> Option<(usize, T)> {
        let mut index = 0;
        let mut earliest_time = None;
        self.locate_earliest_impl(state, &mut 0, &mut index, &mut earliest_time);
        let Some(earliest_time) = earliest_time else {
            return None;
        };
        return Some((index, earliest_time));
    }
}
