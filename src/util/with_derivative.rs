#[derive(Clone)]
pub struct Differentiable<F, DF>(pub F, pub DF);

impl<Ret, F: FnOnce(f64) -> Ret, DF> FnOnce<(f64,)> for Differentiable<F, DF> {
    type Output = Ret;

    extern "rust-call" fn call_once(self, args: (f64,)) -> Self::Output {
        (self.0)(args.0)
    }
}
impl<Ret, F: FnMut(f64) -> Ret, DF> FnMut<(f64,)> for Differentiable<F, DF> {
    extern "rust-call" fn call_mut(&mut self, args: (f64,)) -> Self::Output {
        (self.0)(args.0)
    }
}
impl<Ret, F: Fn(f64) -> Ret, DF> Fn<(f64,)> for Differentiable<F, DF> {
    extern "rust-call" fn call(&self, args: (f64,)) -> Self::Output {
        (self.0)(args.0)
    }
}

impl<F, DF, Ret> Differentiable<F, DF>
where
    DF: Fn<(f64,), Output = Ret>,
{
    pub fn d(&self, t: f64) -> Ret {
        (self.1)(t)
    }
}

pub trait WithDerivative<DF>
where
    Self: Sized,
{
    fn with_derivative(self, derivative: DF) -> Differentiable<Self, DF> {
        Differentiable(self, derivative)
    }
}

impl<Ret, F: Fn(f64) -> Ret, DF: Fn(f64) -> Ret> WithDerivative<DF> for F {}

