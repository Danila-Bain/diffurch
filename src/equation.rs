pub struct Equation<const N: usize = 1, RHS = (), Events = ()> {
    pub rhs: RHS,
    pub events: Events,
}

impl Equation {
    pub fn new<const N: usize, Args, F>(rhs: F) -> Equation<N, F, ()>
    where
        // for<'a> &'a RKState<N, 1, fn(f64) -> [f64; N]>: StateInto<Args>,
        F: Fn<Args, Output = [f64; N]>,
        Args: std::marker::Tuple ,
    {
        Equation::<N, F, ()> { rhs, events: () }
    }


    // ordinary differential equation
    pub fn ode<const N: usize, F>(rhs: F) -> Equation<N, F, ()>
    where
        F: Fn<([f64; N],), Output = [f64; N]>,
    {
        Equation::<N, F, ()> { rhs, events: () }
    }

    pub fn ode2<const N: usize, F>(rhs: F) -> Equation<N, F, ()>
    where
        F: Fn(f64, [f64; N]) -> [f64; N],
    {
        Equation::<N, F, ()> { rhs, events: () }
    }
}


// pub trait EquationArgsOption<const N: usize> {}
//
// impl<const N: usize> EquationArgsOption<N> for (f64,) {}
// impl<const N: usize> EquationArgsOption<N> for (f64, [f64; N]) {}
// impl<const N: usize> EquationArgsOption<N> for ([f64; N],) {}
