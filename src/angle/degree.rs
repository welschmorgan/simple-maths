use crate::{FromPrimitive, ToPrimitive, Zero};
use paste::paste;
use std::{
  fmt::{Debug, Display},
  ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign,
  },
  process::Output,
};

use super::{impl_op, macros::impl_angle, Radian, DEG2RAD, RAD2DEG};

pub struct Degree<T>(T);

impl<
    T: Copy
      + FromPrimitive
      + Sub<T, Output = T>
      + Neg<Output = T>
      + Rem<T, Output = T>
      + Mul<T, Output = T>
      + PartialOrd
      + Zero
      + ToPrimitive
      + FromPrimitive,
  > Degree<T>
{
  pub fn from_radians(v: T) -> Self {
    Self(v * T::from_primitive(RAD2DEG))
  }

  pub fn to_radians(&self) -> Radian<T> {
    Radian::new(self.0 * T::from_primitive(DEG2RAD))
  }
}
impl_angle!(Degree, "∘", 0.0, 360.0);

impl<
    T: Copy
      + FromPrimitive
      + Sub<T, Output = T>
      + Neg<Output = T>
      + Rem<T, Output = T>
      + Mul<T, Output = T>
      + MulAssign<T>
      + PartialOrd
      + Zero
      + ToPrimitive
      + FromPrimitive,
  > Mul<Radian<T>> for Degree<T>
{
  type Output = Degree<T>;

  fn mul(mut self, rhs: Radian<T>) -> Self::Output {
    self.0 *= rhs.to_degrees().0;
    self
  }
}

impl<
    T: Copy
      + FromPrimitive
      + Sub<T, Output = T>
      + Neg<Output = T>
      + Rem<T, Output = T>
      + Mul<T, Output = T>
      + PartialOrd
      + Zero
      + ToPrimitive
      + FromPrimitive,
  > From<Radian<T>> for Degree<T>
{
  fn from(value: Radian<T>) -> Self {
    Degree::from_radians(value.value())
  }
}

impl<
    T: Copy
      + FromPrimitive
      + Sub<T, Output = T>
      + Neg<Output = T>
      + Rem<T, Output = T>
      + Mul<T, Output = T>
      + MulAssign<T>
      + PartialOrd
      + Zero
      + ToPrimitive
      + FromPrimitive,
  > MulAssign<Radian<T>> for Degree<T>
{
  fn mul_assign(&mut self, rhs: Radian<T>) {
    self.0 *= rhs.to_degrees().0;
  }
}

impl<
    T: Copy
      + FromPrimitive
      + Sub<T, Output = T>
      + Neg<Output = T>
      + Rem<T, Output = T>
      + Mul<T, Output = T>
      + Div<T, Output = T>
      + DivAssign<T>
      + PartialOrd
      + Zero
      + ToPrimitive
      + FromPrimitive,
  > Div<Radian<T>> for Degree<T>
{
  type Output = Degree<T>;

  fn div(mut self, rhs: Radian<T>) -> Self::Output {
    self.0 /= rhs.to_degrees().0;
    self
  }
}

impl<
    T: Copy
      + FromPrimitive
      + Sub<T, Output = T>
      + Neg<Output = T>
      + Rem<T, Output = T>
      + Mul<T, Output = T>
      + DivAssign<T>
      + Div<T, Output = T>
      + PartialOrd
      + Zero
      + ToPrimitive
      + FromPrimitive,
  > DivAssign<Radian<T>> for Degree<T>
{
  fn div_assign(&mut self, rhs: Radian<T>) {
    self.0 /= rhs.to_degrees().0;
  }
}

impl<
    T: Copy
      + FromPrimitive
      + Sub<T, Output = T>
      + Neg<Output = T>
      + Rem<T, Output = T>
      + Mul<T, Output = T>
      + Add<T, Output = T>
      + AddAssign<T>
      + PartialOrd
      + Zero
      + ToPrimitive
      + FromPrimitive,
  > Add<Radian<T>> for Degree<T>
{
  type Output = Degree<T>;

  fn add(mut self, rhs: Radian<T>) -> Self::Output {
    self.0 += rhs.to_degrees().0;
    self
  }
}

impl<
    T: Copy
      + FromPrimitive
      + Sub<T, Output = T>
      + Neg<Output = T>
      + Rem<T, Output = T>
      + Mul<T, Output = T>
      + Add<T, Output = T>
      + AddAssign<T>
      + PartialOrd
      + Zero
      + ToPrimitive
      + FromPrimitive,
  > AddAssign<Radian<T>> for Degree<T>
{
  fn add_assign(&mut self, rhs: Radian<T>) {
    self.0 += rhs.to_degrees().0;
  }
}

impl<
    T: Copy
      + FromPrimitive
      + Sub<T, Output = T>
      + Neg<Output = T>
      + Rem<T, Output = T>
      + Mul<T, Output = T>
      + Sub<T, Output = T>
      + SubAssign<T>
      + PartialOrd
      + Zero
      + ToPrimitive
      + FromPrimitive,
  > Sub<Radian<T>> for Degree<T>
{
  type Output = Degree<T>;

  fn sub(mut self, rhs: Radian<T>) -> Self::Output {
    self.0 -= rhs.to_degrees().0;
    self
  }
}

impl<
    T: Copy
      + FromPrimitive
      + Sub<T, Output = T>
      + Neg<Output = T>
      + Rem<T, Output = T>
      + Mul<T, Output = T>
      + Sub<T, Output = T>
      + SubAssign<T>
      + PartialOrd
      + Zero
      + ToPrimitive
      + FromPrimitive,
  > SubAssign<Radian<T>> for Degree<T>
{
  fn sub_assign(&mut self, rhs: Radian<T>) {
    self.0 -= rhs.to_degrees().0;
  }
}

#[cfg(test)]
mod tests {
  use std::f32::consts::PI;

  use crate::{Degree, Radian};

  #[test]
  fn deg2rad() {
    assert_eq!(Degree::new(90f32).to_radians().value(), PI / 2f32)
  }

  #[test]
  fn clamp() {
    assert_eq!(Degree::new(-90f32).value(), 360.0 - 90.0);
    assert_eq!(Degree::new(720f32).value(), 0.0);
  }
}
