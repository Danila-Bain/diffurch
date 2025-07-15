use crate::{EventCall, State, StateFnMut};

use super::*;

/// Detection method marker for detecting that a delayed argument function has crossed a value in
/// the state discontinuity list [State::disco].
pub struct Propagation;

///
pub struct Propagator<const N: usize, Alpha: StateFnMut<N, Output = f64>> {
    /// Delayed argument function
    pub alpha: Alpha,
    /// index into state disco queue
    pub disco_idx: usize,
    /// how order of discontinuity increases after this propagation
    ///
    /// 1 corresponds to retarded delay,
    /// 0 corresponds to neutral delay
    pub smoothing_order: usize,
    /// the time of propagated discontinuity from last detection
    pub propagated_t: f64,
    /// the order of propagated discontinuity from last detection
    pub propagated_order: usize,
}

impl<const N: usize, Alpha: StateFnMut<N, Output = f64>> Propagator<N, Alpha> {
    /// Constructs a new Propagator object with provided values for `alpha` and
    /// `smoothing_order` and sets `disco_idx: 0`, `propagated_t: f64::NAN`,
    /// `propagated_order: usize::MAX`
    pub fn new(alpha: Alpha, smoothing_order: usize) -> Self {
        Self {
            alpha,
            disco_idx: 0,
            smoothing_order,
            propagated_t: f64::NAN,
            propagated_order: usize::MAX,
        }
    }
}

impl<const N: usize, Alpha: StateFnMut<N, Output = f64>> StateFnMut<N> for Propagator<N, Alpha> {
    type Output = f64;

    fn eval(&mut self, state: &impl State<N>) -> Self::Output {
        self.alpha.eval(state) - self.propagated_t
    }

    fn eval_prev(&mut self, state: &impl State<N>) -> Self::Output {
        self.alpha.eval_prev(state) - self.propagated_t
    }

    fn eval_at(&mut self, state: &impl State<N>, t: f64) -> Self::Output {
        self.alpha.eval_at(state, t) - self.propagated_t
    }
}

impl<const N: usize, Alpha: StateFnMut<N, Output = f64>, L> Detect<N>
    for Loc<Propagator<N, Alpha>, Propagation, L>
{
    fn detect(&mut self, state: &impl State<N>) -> bool {
        let alpha_prev = self.0.alpha.eval_prev(state);
        let alpha_curr = self.0.alpha.eval(state);
        // println!("Delay: {}, Time: {} -> {}, Delayed: {alpha_prev} -> {alpha_curr}", state.t() - alpha_curr, state.t_prev(), state.t());

        if alpha_prev < alpha_curr {
            // get first t_disco > alpha_prev
            while let Some((t_disco, _)) = state.disco_seq().get(self.0.disco_idx)
                && *t_disco <= alpha_prev
            {
                self.0.disco_idx += 1;
            }
            while self.0.disco_idx > 0
                && let Some((t_disco, _)) = state.disco_seq().get(self.0.disco_idx - 1)
                && *t_disco > alpha_prev
            {
                self.0.disco_idx -= 1;
            }

            // check for t_disco < alpha_curr
            if let Some((t_disco, order_disco)) = state.disco_seq().get(self.0.disco_idx)
                && *t_disco <= alpha_curr
            {
                // println!("\tCrossing: {t_disco}");
                self.0.propagated_t = *t_disco;
                self.0.propagated_order = *order_disco;
                return true;
            }
        } else {
            todo!();
        }

        // while let Some((t_disco, _)) = state.disco().get(self.0.disco_idx + 1)
        //     && *t_disco < alpha_prev
        // {
        //     self.0.disco_idx += 1;
        // }
        // if let Some((t_disco, _)) = state.disco().get(self.0.disco_idx + 1)
        //     && alpha_curr > *t_disco
        // {
        //     self.0.propagated_t = *t_disco;
        //     return true;
        // }
        //
        // while let Some((t_disco, _)) = state.disco().get(self.0.disco_idx)
        //     && *t_disco > alpha_prev
        // {
        //     self.0.disco_idx -= 1;
        // }
        // if let Some((t_disco, _)) = state.disco().get(self.0.disco_idx)
        //     && alpha_curr < *t_disco
        // {
        //     self.0.propagated_t = *t_disco;
        //     return true;
        // }

        false
    }
}

impl<const N: usize, Alpha: StateFnMut<N, Output = f64>, L> EventCall<N>
    for Loc<Propagator<N, Alpha>, Propagation, L>
{
    fn call(&mut self, state: &mut impl State<N>) {
        let new_order = self.0.propagated_order + self.0.smoothing_order;
        if new_order < state.interpolation_order() {
            let t = state.t();
            state.disco_seq_mut().push_back((t, new_order))
        }
    }
}
