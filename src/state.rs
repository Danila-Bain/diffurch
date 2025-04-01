pub struct State<const N: usize, const S: usize> {
    t: f64,
    t_init: f64,
    t_prev: f64,
    t_step: f64,
    t_span: f64,
    t_seq: Vec<f64>,

    x: [f64; N],
    x_init: [f64; N],
    x_prev: [f64; N],
    x_err: [f64; N],
    x_seq: Vec<[f64; N]>,

    k: [[f64; N]; S],
    k_seq: Vec<[[f64; N]; S]>,
}

impl<const N: usize, const S: usize> State<N, S> {
    fn new(t_init: f64, x_init: [f64; N]) -> Self {
        Self {
            t_init,
            t: t_init,
            t_prev: t_init,
            t_step: 0.,
            t_span: 0.,
            t_seq: vec![t_init],

            x_init,
            x: x_init,
            x_prev: x_init,
            x_err: [0.; N],
            x_seq: vec![x_init],

            k: [[0.; N]; S],
            k_seq: Vec::new(),
        }
    }

    fn push_current(&mut self) {
        self.t_seq.push(self.t);
        self.x_seq.push(self.x);
        self.k_seq.push(self.k);
    }

    fn make_zero_step(&mut self) {
        self.t_prev = self.t;
        self.x_prev = self.x;
        self.k = [[0.; N]; S];
        self.push_current();
    }

    fn eval (&self, t: f64) -> [f64; N] {
        if t < self.t_init {
            return self.x_init;
        } else {
            unimplemented!("Eval function unimplemented. Polynomial is required");
        }
    }
}
