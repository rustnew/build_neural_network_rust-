use std::f64::consts::E;

#[derive(Clone)]
pub  struct Activation<'a> {
    pub  fonction : &'a dyn Fn(f64) -> f64,
    pub derivation : &'a dyn  Fn(f64) -> f64,
}

pub const IDENTITY : Activation = Activation {
    fonction : &|x| x,
    derivation : &|_| 1.0,
};

pub const SIGMOID: Activation = Activation {
	fonction: &|x| 1.0 / (1.0 + E.powf(-x)),
	derivation: &|x| x * (1.0 - x),
};

pub const TANH: Activation = Activation {
	fonction: &|x| x.tanh(),
	derivation: &|x| 1.0 - (x.powi(2)),
};

pub const RELU: Activation = Activation {
	fonction: &|x| x.max(0.0),
	derivation: &|x| if x > 0.0 { 1.0 } else { 0.0 },
};

