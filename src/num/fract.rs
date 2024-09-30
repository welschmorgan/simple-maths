use crate::Vector;

pub trait Fract {}

impl Fract for f32 {}
impl Fract for f64 {}

impl<T: Fract, const N: usize> Fract for Vector<T, N> {}
