pub struct Equation<const N: usize = 1, RHS = (), Events = ()> {
    pub rhs: RHS,
    pub events: Events,
}

impl Equation {
    pub fn new<const N: usize, Args: std::marker::Tuple, F: Fn<Args, Output = [f64; N]>>(rhs: F) -> Equation<N, F, ()> {
        Equation::<N, F, ()> {
            rhs,
            events: ()
        }
    }
}

