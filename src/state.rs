use crate::initial_condition::InitialCondition;
use std::collections::VecDeque;

pub struct StateHistory<'rk, const N: usize, const S: usize, const S2: usize, T, IC> {
    pub t_span: T,

    pub t_init: T,
    pub x_init: IC,

    pub t_deque: VecDeque<T>,
    pub x_deque: VecDeque<[T; N]>,
    pub k_deque: VecDeque<[[T; N]; S]>,
    pub disco_deque: VecDeque<(T, usize)>,

    pub rk: &'rk crate::rk::ExplicitRungeKuttaTable<S, S2, T>,
}

pub struct State<'rk, const N: usize, const S: usize, const S2: usize, T, IC> {
    pub history: StateHistory<'rk, N, S, S2, T, IC>,

    pub t_curr: T,
    pub t_prev: T,

    pub x_curr: [T; N],
    pub x_prev: [T; N],

    pub dx_curr: [T; N],
    pub dx_prev: [T; N],

    pub rk: &'rk crate::rk::ExplicitRungeKuttaTable<S, S2, T>,
    pub k_curr: [[T; N]; S],
}

impl<
    'rk,
    T: num::Float + std::fmt::Debug,
    const N: usize,
    const S: usize,
    const S2: usize,
    IC: InitialCondition<N, T>,
> State<'rk, N, S, S2, T, IC>
{
    pub fn new(
        t_init: T,
        x_init: IC,
        rk: &'rk crate::rk::ExplicitRungeKuttaTable<S, S2, T>,
    ) -> Self {
        let x = x_init.eval::<0>(t_init);
        Self {
            t_curr: t_init,
            t_prev: t_init,
            x_curr: x,
            x_prev: x,
            dx_curr: [T::nan(); N],
            dx_prev: [T::nan(); N],
            rk,
            k_curr: [[T::zero(); N]; S],
            history: StateHistory {
                rk,
                t_init,
                t_span: T::zero(),
                x_init,
                t_deque: VecDeque::new(),
                x_deque: VecDeque::new(),
                k_deque: VecDeque::new(),
                disco_deque: VecDeque::new(),
            },
        }
    }

    pub fn eval<const D: usize>(&self, t: T) -> [T; N] {
        self.history.eval::<D>(t)
    }

    pub fn make_step(&mut self, rhs: &mut impl EvalStateFn<N, T, [T; N]>, t_step: T) {
        if self.t_prev != self.t_curr {
            self.k_curr[0] = self.dx_curr;
        } else {
            self.k_curr[0] = rhs.eval_curr(self);
        }

        self.t_prev = self.t_curr;
        self.x_prev = self.x_curr;
        self.dx_prev = self.dx_curr;

        let mut a_i = 0;
        for i in 1..S {
            self.t_curr = self.t_prev + self.rk.c[i] * t_step;

            self.x_curr = std::array::from_fn(|k| {
                self.x_prev[k]
                    + t_step
                        * (0..i).fold(T::zero(), |acc, j| {
                            acc + self.rk.a[a_i + j] * self.k_curr[j][k]
                        })
            });
            a_i += i;
            self.k_curr[i] = rhs.eval_curr(self);
        }

        self.x_curr = std::array::from_fn(|k| {
            self.x_prev[k]
                + t_step * (0..S).fold(T::zero(), |acc, j| acc + self.rk.b[j] * self.k_curr[j][k])
        });
        self.t_curr = self.t_prev + t_step;
        self.dx_curr = rhs.eval_curr(self);
    }

    pub fn commit_step(&mut self) {
        self.history.t_deque.push_back(self.t_curr);
        self.history.x_deque.push_back(self.x_curr);
        self.history.k_deque.push_back(self.k_curr);
        let t_tail = self.t_prev - self.history.t_span;
        while let Some(second_t) = self.history.t_deque.get(1)
            && *second_t < t_tail
        {
            self.history.t_deque.pop_front();
            self.history.x_deque.pop_front();
            self.history.k_deque.pop_front();
        }
        while let Some((t, _order)) = self.history.disco_deque.front()
            && t < &t_tail
        {
            self.history.disco_deque.pop_front();
        }
    }

    pub fn make_zero_step(&mut self) {
        self.t_prev = self.t_curr;
        self.x_prev = self.x_curr;
        self.k_curr = [[T::zero(); N]; S];
        self.dx_curr = [T::zero(); N];
    }

    pub fn undo_step(&mut self) {
        self.t_curr = self.t_prev;
        self.x_curr = self.x_prev;
        self.dx_curr = self.dx_prev;
    }
}

impl<
    'rk,
    T: num::Float + std::fmt::Debug,
    const N: usize,
    const S: usize,
    const S2: usize,
    IC: InitialCondition<N, T>,
> StateHistory<'rk, N, S, S2, T, IC>
{
    fn dense_output_formula<const D: usize>(
        &self,
        x_prev: &[T; N],
        t_step: T,
        theta: T,
        k: &[[T; N]; S],
    ) -> [T; N] {
        match D {
            0 => {
                return std::array::from_fn(|i| {
                    x_prev[i]
                        + t_step
                            * (0..S)
                                .fold(T::zero(), |acc, j| acc + (self.rk.bi[j].0)(theta) * k[j][i])
                });
            }
            1 => {
                return std::array::from_fn(|i| {
                    (0..S).fold(T::zero(), |acc, j| acc + (self.rk.bi[j].1)(theta) * k[j][i])
                });
            }
            _ => unimplemented!(),
        }
    }
    pub fn eval<const D: usize>(&self, t: T) -> [T; N] {
        if t <= self.t_init {
            self.x_init.eval::<D>(t)
        } else {
            let i = self.t_deque.partition_point(|t_i| t_i <= &t); // first i : t_seq[i] > t
            if i == 0 {
                panic!(
                    "Evaluation of state at {t:?} in deleted time range (before {:?})",
                    self.t_deque.front(),
                );
            } else if i == self.t_deque.len() {
                panic!(
                    "Evaluation of state in a not yet computed time range at {t:?} while most recent time in history is {:?}.",
                    self.t_deque.front()
                );
            }
            let x_prev = &self.x_deque[i - 1];
            let k = &self.k_deque[i - 1];
            let t_prev = self.t_deque[i - 1];
            let t_next = self.t_deque[i];
            let t_step = t_next - t_prev;
            let theta = (t - t_prev) / t_step;
            return self.dense_output_formula::<D>(&x_prev, t_step, theta, &k);
        }
    }
}

pub struct StateRef<'s, T, const N: usize> {
    /// Time of a state
    pub t: T,
    /// Position of a state
    pub x: &'s [T; N],

    /// Derivative of a state
    pub dx: &'s [T; N],

    pub h: &'s dyn Fn(T) -> [T; N],
}

pub struct StateRefMut<'s, T, const N: usize> {
    /// Reference to time of a state
    pub t: &'s mut T,
    /// Reference to position of a state
    pub x: &'s mut [T; N],

    pub dx: &'s [T; N],

    pub h: &'s dyn Fn(T) -> [T; N],
}

#[allow(unused)]
pub struct StateFn<const N: usize, T, Output, F, const MUT: bool = false> {
    f: F,
    _phantom_f: std::marker::PhantomData<fn(&StateRef<T, N>) -> Output>,
}

impl<T, const N: usize, Output, F: FnMut(&StateRef<T, N>) -> Output>
    StateFn<N, T, Output, F, false>
{
    pub fn new(f: F) -> Self {
        Self {
            f,
            _phantom_f: std::marker::PhantomData,
        }
    }
}

impl<T, const N: usize, Output, F: FnMut(&mut StateRefMut<T, N>) -> Output>
    StateFn<N, T, Output, F, true>
{
    pub fn new_mut(f: F) -> Self {
        Self {
            f,
            _phantom_f: std::marker::PhantomData,
        }
    }
}

// abstract F parameter away
pub trait EvalStateFn<const N: usize, T, Output> {
    fn eval_curr<'s, const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &'s State<N, S, S2, T, IC>,
    ) -> Output;

    fn eval_prev<'s, const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &'s State<N, S, S2, T, IC>,
    ) -> Output;

    fn eval_at<'s, const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &'s State<N, S, S2, T, IC>,
        t: T,
    ) -> Output;
}

impl<T: num::Float + std::fmt::Debug, const N: usize, Output, F: FnMut(&StateRef<T, N>) -> Output>
    EvalStateFn<N, T, Output> for StateFn<N, T, Output, F, false>
{
    fn eval_curr<'s, const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &'s State<N, S, S2, T, IC>,
    ) -> Output {
        (self.f)(&StateRef {
            t: state.t_curr,
            x: &state.x_curr,
            dx: &state.dx_curr,
            h: &|t: T| state.history.eval::<0>(t),
        })
    }
    fn eval_prev<'s, const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &'s State<N, S, S2, T, IC>,
    ) -> Output {
        (self.f)(&StateRef {
            t: state.t_prev,
            x: &state.x_prev,
            dx: &state.dx_prev,
            h: &|t: T| state.eval::<0>(t),
        })
    }
    fn eval_at<'s, const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &'s State<N, S, S2, T, IC>,
        t: T,
    ) -> Output {
        (self.f)(&StateRef {
            t: t,
            x: &state.history.eval::<0>(t),
            dx: &state.history.eval::<1>(t),
            h: &|t: T| state.history.eval::<0>(t),
        })
    }
}
pub trait EvalMutStateFn<const N: usize, T, Output> {
    fn eval_mut<'s, const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &'s mut State<N, S, S2, T, IC>,
    ) -> Output;
}

impl<
    T: num::Float + std::fmt::Debug,
    const N: usize,
    Output,
    F: FnMut(&mut StateRefMut<T, N>) -> Output,
> EvalMutStateFn<N, T, Output> for StateFn<N, T, Output, F, true>
{
    fn eval_mut<'s, const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &'s mut State<N, S, S2, T, IC>,
    ) -> Output {
        (self.f)(&mut StateRefMut {
            t: &mut state.t_curr,
            x: &mut state.x_curr,
            dx: &state.dx_curr,
            h: &|t: T| state.history.eval::<0>(t),
        })
    }
}

impl<T: num::Float + std::fmt::Debug, const N: usize, Output, F: FnMut(&StateRef<T, N>) -> Output>
    EvalMutStateFn<N, T, Output> for StateFn<N, T, Output, F, false>
{
    fn eval_mut<'s, const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
        &mut self,
        state: &'s mut State<N, S, S2, T, IC>,
    ) -> Output {
        self.eval_curr(state)
    }
}

hlist2_trait_macro::TraitHList! {
    pub EvalStateFnHList for trait EvalStateFn<const N: usize, T, Output> {
        fn eval_curr<'s, const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
            &mut self,
            state: &'s State<N, S, S2, T, IC>,
        ) -> Output where T: 's, IC: 's;
    }
}

hlist2_trait_macro::TraitHList! {
    pub EvalMutStateFnHList for trait EvalMutStateFn<const N: usize, T, Output> {
        fn eval_mut<'s, const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
            &mut self,
            state: &'s mut State<N, S, S2, T, IC>,
        ) -> Output where T: 's, IC: 's;
    }
}

pub trait IntoStateFn<const N: usize, T, Output>: Sized {
    type Output: EvalStateFn<N, T, Output>;
    fn into(self) -> Self::Output;
}
pub trait IntoMutStateFn<const N: usize, T, Output>: Sized {
    type Output: EvalMutStateFn<N, T, Output>;
    fn into(self) -> Self::Output;
}

// pub trait IntoStateFn<const N: usize, T, Output>: Sized {
//     fn into(self) -> StateFn<N, T, Output, Self, false>;
// }
// pub trait IntoMutStateFn<const N: usize, T, Output>: Sized {
//     fn into(self) -> StateFn<N, T, Output, Self, true>;
// }

impl<const N: usize, T: num::Float + std::fmt::Debug, Output, F: FnMut(&StateRef<T, N>) -> Output>
    IntoStateFn<N, T, Output> for F
{
    type Output = StateFn<N, T, Output, Self, false>;
    fn into(self) -> Self::Output {
        StateFn::new(self)
    }
}
impl<const N: usize, T: num::Float + std::fmt::Debug, Output, F: FnMut(&mut StateRefMut<T, N>) -> Output>
    IntoMutStateFn<N, T, Output> for F
{
    type Output = StateFn<N, T, Output, Self, true> ;
    fn into(self) -> Self::Output {
        StateFn::new_mut(self)
    }
}

impl<const N: usize, T: num::Float + std::fmt::Debug, Output, F: FnMut(&StateRef<T, N>) -> Output>
    IntoStateFn<N, T, Output> for StateFn<N, T, Output, F, false>
{
    type Output = StateFn<N, T, Output, F, false>;
    fn into(self) -> Self::Output {
        self
    }
}
impl<const N: usize, T: num::Float + std::fmt::Debug, Output, F: FnMut(&mut StateRefMut<T, N>) -> Output>
    IntoMutStateFn<N, T, Output> for StateFn<N, T, Output, F, true>
{
    type Output = StateFn<N, T, Output, F, true>;
    fn into(self) -> Self::Output {
        self
    }
}


// impl<T, const N: usize, Output, F: FnMut(&StateRef<T, N>) -> Output>
//     StateFn<N, T, Output, F, false>
// {
//     pub fn new(f: F) -> Self {
//         Self {
//             f,
//             _phantom_f: std::marker::PhantomData,
//         }
//     }
// }
//
// impl<T, const N: usize, Output, F: FnMut(&mut StateRefMut<T, N>) -> Output>
//     StateFn<N, T, Output, F, true>
// {
//     pub fn new_mut(f: F) -> Self {
//         Self {
//             f,
//             _phantom_f: std::marker::PhantomData,
//         }
//     }
// }

// impl<T: num::Float + std::fmt::Debug,
//     const N: usize,
//     F: Fn(&StateRef<T, N>) -> (),
//     Filter,
// > StateFn<N, T, (), F, Filter> {
//     pub fn eval_filtered<'s, const S: usize, const S2: usize, IC: InitialCondition<N, T>>(
//         &self,
//         state: &'s State<N, S, S2, T, IC>,
//     ) {
//         // if self.filter.all() {
//            self.eval_curr(&state)
//         // }
//     }
//
// }

#[macro_export]
macro_rules! state_fn(
    (
        |
            $(
                $var:ident
                $(:
                    $($bind:ident)?
                    $(&$ref_bind:ident)?
                    $([$($coords:pat),*])?
                    $(&[$($ref_coords:pat),*])?
                )?
            ),*
        | $body:expr
    ) => {
        $crate::StateFn::new(|&$crate::StateRef{
            $(
                $var
                $(: $($bind)?
                    $(&$ref_bind)?
                    $([$($coords),*])?
                    $(&[$($ref_coords),*])?
                )?
                ,
            )*
            ..
        }| $body)
    }
);
#[macro_export]
macro_rules! mut_state_fn(
    (
        |
            $(
                $var:ident
                $(:
                    $($bind:ident)?
                    $(&mut$ref_bind:ident)?
                    $([$($coords:pat),*])?
                    $(&mut[$($ref_coords:pat),*])?
                )?
            ),*
        | $body:expr
    ) => {
        $crate::StateFn::new_mut(|&mut $crate::StateRefMut{
            $(
                $var
                $(: $($bind)?
                    $(&mut $ref_bind)?
                    $([$($coords),*])?
                    $(&mut [$($ref_coords),*])?
                )?
                ,
            )*
            ..
        }| $body)
    }
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn macro_state_fn() {
        let rk = crate::rk::euler();
        let state = State::new(0., [1., 2., 3.], &rk);
        let mut f = state_fn!(|t, x: &[x, y, z]| [t, x, y, z]);
        assert_eq!(f.eval_curr(&state), [0., 1., 2., 3.]);
        let mut f = crate::StateFn::new(|&crate::StateRef { t, x, .. }| [t, x[0], x[1], x[2]]);
        assert_eq!(f.eval_curr(&state), [0., 1., 2., 3.]);
        let mut f = crate::StateFn::new(|&crate::StateRef { t, x: &x, .. }| [t, x[0], x[1], x[2]]);
        assert_eq!(f.eval_curr(&state), [0., 1., 2., 3.]);
    }
    #[test]
    fn macro_mut_state_fn() {
        let rk = crate::rk::euler();
        let mut state = State::new(0., [1., 2., 3.], &rk);
        let mut f = mut_state_fn!(|t: &mut t, x: [x,y,z]| {
            *y += 10.;
            *z += 10.;
            [t, *x, *y, *z]
        });
        assert_eq!(f.eval_mut(&mut state), [0., 1., 12., 13.]);
    }

    #[test]
    fn lorenz() {
        let sigma = 10.;
        let rho = 28.;
        let beta = 8. / 3.;

        let rk = crate::rk::euler();

        let mut state = State::new(0., [0., 0., 0.], &rk);

        let mut lorenz_rhs = StateFn::new(|&StateRef { x: [x, y, z], .. }| {
            [sigma * (y - x), x * (rho - z) - y, x * y - beta * z]
        });
        lorenz_rhs.eval_curr(&state);

        let mut event = StateFn::new_mut(|state| {
            let ref mut t = *state.t;
            *t += 4.2;
        });
        event.eval_mut(&mut state);

        let mut event = StateFn::new_mut(
            |&mut StateRefMut {
                 ref mut t,
                 x: [x, _, _],
                 ..
             }| {
                let ref mut t = **t;
                *x = 15.;
                *t = 5.;
            },
        );
        event.eval_mut(&mut state);
        assert_eq!(state.t_curr, 5.);
        assert_eq!(state.x_curr[0], 15.);

        // panic!("{:?}, {:?}", state.t_curr, state.x_curr)
    }
}
