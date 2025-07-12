use super::{Loc, Locate};

use crate::event::*;
use crate::{
    StateFnMutComposition,
    state::{State, StateFnMut},
};

pub struct Propagated<Alpha> {
    /// at which order we shall stop propagating
    pub order: usize,
    /// index into state disco queue, it is assumed,
    /// that previous evaluation is on the half-open interval
    /// [ state.disco()[disco_idx - 1], state.disco[disco_idx] )
    /// where state.disco()[-1] is f64::NEG_INFINITY
    pub disco_idx: usize,
    pub last_t: f64,
    pub order_increase: usize,
    /// Deviated argument function
    pub alpha: Alpha,
}

impl<Alpha> Propagated<Alpha> {
    pub fn new(alpha: Alpha) -> Self {
        Propagated {
            order: 0,
            alpha,
            disco_idx: 0,
            last_t: f64::NEG_INFINITY,
            order_increase: 1,
        }
    }
}

impl<const N: usize, Alpha: StateFnMut<N, Output = f64>> Locate<N> for Propagated<Alpha> {
    fn locate(&mut self, state: &impl State<N>) -> Option<f64> {
        // we assume that delay function is continuous, because otherwise
        // additional events need to be introduced externally anyway

        let alpha_value = self.alpha.eval(state);

        // dbg!(&self);
        // dbg!(&state.disco());
        // dbg!(alpha_value);
        //

        if let Some((t_disco, _order)) = state.disco().get(self.disco_idx) {
            dbg!(t_disco);
            dbg!(alpha_value);

            if &alpha_value > t_disco {
                let t_loc = Loc(
                    StateFnMutComposition(&mut |alpha_| alpha_ - *t_disco, &mut self.alpha),
                    super::detection::Sign,
                    super::location::Bisection,
                )
                .locate(state);
                // let t_loc = crate::loc!(StateFnMutComposition(&mut |alpha_| alpha_ - *t_disco, &mut self.alpha)).locate(state);

                // dbg!(t_loc);
                return t_loc;
            }
        }

        None

        // for (idx, dir) in [(self.disco_idx - 1, -1isize), (self.disco_idx, 1)].iter() {
        //     if let Some((t, order)) = state.disco().get(*idx as usize)
        //         && alpha_value < *t
        //     {
        //         let t_loc = Loc(
        //             Sign(StateFnMutComposition(
        //                 &mut |alpha_| alpha_ - *t,
        //                 &mut self.alpha,
        //             )),
        //             Bisection,
        //         )
        //         .locate(state);
        //
        //
        //         if let Some(t_loc) = t_loc {
        //             self.order = *order;
        //             self.last_t = t_loc;
        //             self.disco_idx += dir;
        //             return Some(t_loc);
        //         } else {
        //             return None;
        //         }
        //     }
        // }
        // None
    }
}

impl<const N: usize, Alpha: StateFnMut<N, Output = f64>> EventCall<N> for Propagated<Alpha> {
    fn call(&mut self, state: &mut impl State<N>) {
        let t = state.t();
        self.disco_idx += 1;

        state
            .disco_mut()
            .push_back((t, self.order + self.order_increase))
    }
}

impl<Alpha> std::fmt::Debug for Propagated<Alpha> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Propagated")
            .field("disco_idx", &self.disco_idx)
            .field("last_t", &self.last_t)
            .field("order", &self.order)
            .field("order_increase", &self.order_increase)
            .finish()
    }
}
