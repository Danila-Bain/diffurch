use crate::{
    initial_condition::InitialCondition, loc::locate::Locate, state::State, traits::RealVectorSpace,
};
use hlist2_trait_macro::TraitHList;
use nalgebra::RealField;

pub trait LocateEarliestImpl<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
>
{
    fn locate_earliest_impl(
        &mut self,
        state: &State<T, Y, S, I, IC>,
        self_index: &mut usize,
        earliest_index: &mut usize,
        earliest_time: &mut Option<T>,
    );
}

impl<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
    L: Locate<T, Y, S, I, IC>,
> LocateEarliestImpl<T, Y, S, I, IC> for L
{
    fn locate_earliest_impl(
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
    pub HListLocateEarliestImpl for
        trait LocateEarliestImpl<
            T: RealField + Copy,
            Y: RealVectorSpace<T>,
            const S: usize,
            const I: usize,
            IC: InitialCondition<T, Y>,
        > {
        fn locate_earliest_impl(
            &mut self,
            state: &State<T, Y, S, I, IC>,
            self_index: &mut usize,
            earliest_index: &mut usize,
            earliest_time: &mut Option<T>,
        );
    }
}

pub trait HListLocateEarliest<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
>
{
    fn locate_earliest(&mut self, state: &State<T, Y, S, I, IC>) -> Option<(usize, T)>;
}

impl<
    T: RealField + Copy,
    Y: RealVectorSpace<T>,
    const S: usize,
    const I: usize,
    IC: InitialCondition<T, Y>,
    U: HListLocateEarliestImpl<T, Y, S, I, IC>,
> HListLocateEarliest<T, Y, S, I, IC> for U
{
    fn locate_earliest(&mut self, state: &State<T, Y, S, I, IC>) -> Option<(usize, T)> {
        let mut index = 0;
        let mut earliest_time = None;
        self.locate_earliest_impl(state, &mut 0, &mut index, &mut earliest_time);
        Some((index, earliest_time?))
    }
}
