use crate::{FromPrimitive, ToPrimitive, Zero};
use paste::paste;
use std::{
  fmt::{Debug, Display},
  ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign,
  },
};

use super::{impl_op, macros::impl_angle, Degree, DEG2RAD, RAD2DEG};

pub struct Radian<T>(T);

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
  > From<Degree<T>> for Radian<T>
{
  fn from(value: Degree<T>) -> Self {
    Radian::from_degrees(value.value())
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
  > Radian<T>
{
  pub fn from_degrees(v: T) -> Self {
    Self(v * T::from_primitive(DEG2RAD))
  }

  pub fn to_degrees(&self) -> Degree<T> {
    Degree::new(self.0 * T::from_primitive(RAD2DEG))
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
  > Mul<Degree<T>> for Radian<T>
{
  type Output = Radian<T>;

  fn mul(mut self, rhs: Degree<T>) -> Self::Output {
    self.0 *= rhs.to_radians().0;
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
      + MulAssign<T>
      + PartialOrd
      + Zero
      + ToPrimitive
      + FromPrimitive,
  > MulAssign<Degree<T>> for Radian<T>
{
  fn mul_assign(&mut self, rhs: Degree<T>) {
    self.0 *= rhs.to_radians().0;
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
      + Div<T, Output = T>
      + DivAssign<T>
      + PartialOrd
      + Zero
      + ToPrimitive
      + FromPrimitive,
  > Div<Degree<T>> for Radian<T>
{
  type Output = Radian<T>;

  fn div(mut self, rhs: Degree<T>) -> Self::Output {
    self.0 /= rhs.to_radians().0;
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
      + MulAssign<T>
      + Div<T, Output = T>
      + DivAssign<T>
      + PartialOrd
      + Zero
      + ToPrimitive
      + FromPrimitive,
  > DivAssign<Degree<T>> for Radian<T>
{
  fn div_assign(&mut self, rhs: Degree<T>) {
    self.0 /= rhs.to_radians().0;
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
      + Add<T, Output = T>
      + AddAssign<T>
      + PartialOrd
      + Zero
      + ToPrimitive
      + FromPrimitive,
  > Add<Degree<T>> for Radian<T>
{
  type Output = Radian<T>;

  fn add(mut self, rhs: Degree<T>) -> Self::Output {
    self.0 += rhs.to_radians().0;
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
      + MulAssign<T>
      + Add<T, Output = T>
      + AddAssign<T>
      + PartialOrd
      + Zero
      + ToPrimitive
      + FromPrimitive,
  > AddAssign<Degree<T>> for Radian<T>
{
  fn add_assign(&mut self, rhs: Degree<T>) {
    self.0 += rhs.to_radians().0;
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
      + Sub<T, Output = T>
      + SubAssign<T>
      + PartialOrd
      + Zero
      + ToPrimitive
      + FromPrimitive,
  > Sub<Degree<T>> for Radian<T>
{
  type Output = Radian<T>;

  fn sub(mut self, rhs: Degree<T>) -> Self::Output {
    self.0 -= rhs.to_radians().0;
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
      + MulAssign<T>
      + Sub<T, Output = T>
      + SubAssign<T>
      + PartialOrd
      + Zero
      + ToPrimitive
      + FromPrimitive,
  > SubAssign<Degree<T>> for Radian<T>
{
  fn sub_assign(&mut self, rhs: Degree<T>) {
    self.0 -= rhs.to_radians().0;
  }
}

impl_angle!(Radian, "rad", 0.0, 2.0 * std::f64::consts::PI);

#[cfg(test)]
mod tests {
  use std::f32::consts::PI;

  use crate::{Degree, Radian};

  #[test]
  fn rad2deg() {
    assert_eq!(Radian::new(PI / 2f32).to_degrees().value(), 90f32)
  }
}
