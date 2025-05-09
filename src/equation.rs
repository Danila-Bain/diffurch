use crate::{State, StateFn};

// use crate::state::CoordFn;
//
pub struct Equation<'a, const N: usize = 1> {
    pub rhs: StateFn<'a, N, [f64; N]>,
    pub max_delay: f64,
}

impl<'a, const N: usize> Equation<'a, N> {
    pub fn new(rhs: StateFn<'a, N, [f64; N]>) -> Self {
        Equation {
            rhs,
            max_delay: f64::NAN,
        }
    }

    // ordinary differential equation
    pub fn ode< RHS>(rhs: RHS) -> Self
    where
        RHS: 'a + Fn<([f64; N],), Output = [f64; N]>,
    {
        Equation {
            rhs: StateFn::ODE(Box::new(rhs)),
            max_delay: 0.,
        }
    }


    pub fn ode2< RHS>(rhs: RHS) -> Self
    where
        RHS: 'a + Fn<(f64, [f64; N],), Output = [f64; N]>,
    {
        Equation {
            rhs: StateFn::ODE2(Box::new(rhs)),
            max_delay: 0.,
        }
    }
    //
    //     pub fn dde<const N: usize, RHS, const S: usize, IF: Fn(f64) -> [f64; N]>(
    //         rhs: RHS,
    //     ) -> Equation<N, RHS, ()>
    //     where
    //         RHS: for<'a> Fn(f64, [f64; N], [CoordFn<'a, N, S, IF>; N]) -> [f64; N],
    //     {
    //         Equation::<N, RHS, ()> {
    //             rhs,
    //             events: (),
    //             max_delay: f64::NAN,
    //         }
    //     }
    pub fn with_delay(self, value: f64) -> Self {
        Self {
            rhs: self.rhs,
            max_delay: value,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation() {
        let _eq = Equation {
            rhs: StateFn::ODE2(Box::new(|t, [x, y]| [-y / t, x])),
            max_delay: f64::NAN,
        };

        let _eq = Equation::new(StateFn::Constant(Box::new(|| [42.])));
        let _eq = Equation::ode(|[x, y]| [-y, x]);
        let _eq = Equation::ode2(|t, [x, y, z]| [t-y, z-x, x - z/t]);
    }
}
