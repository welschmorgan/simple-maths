use std::fmt::Display;

use crate::{vec4, Vec4f, Vector, Zero};

#[cfg(not(feature = "double-precision"))]
pub type DefaultFracType = f32;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Quat<T: Fract = f32>(Vector<T, 4>);

impl<T> Quat<T> {
  pub fn new(x: T, y: T, z: T, w: T) -> Self {
    Self(Vector::<T, 4>::new([x, y, z, w]))
  }

  pub fn identity() -> Self {
    Self::new(0.0, 0.0, 0.0)
  }
}

impl Zero for Quat {
  fn zero() -> Self {
    Quat::identity()
  }
}
