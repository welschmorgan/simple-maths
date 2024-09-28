use crate::{FromPrimitive, Sqrt, ToPrimitive, Unit, Zero};
use paste::paste;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::{
  Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Range,
  Sub, SubAssign,
};

pub struct Vector<T, const N: usize>([T; N]);

impl<T, const N: usize> Vector<T, N> {
  pub fn new(data: [T; N]) -> Self {
    Self(data)
  }

  pub fn get(&self, i: usize) -> Option<&T> {
    self.0.get(i)
  }

  pub fn get_mut(&mut self, i: usize) -> Option<&mut T> {
    self.0.get_mut(i)
  }
}

impl<T: PartialOrd + Copy, const N: usize> Vector<T, N> {
  pub fn clamp(&mut self, ranges: [Range<T>; N]) {
    for i in 0..N {
      if self.0[i] < ranges[i].start {
        self.0[i] = ranges[i].start;
      } else if self.0[i] > ranges[i].end {
        self.0[i] = ranges[i].end
      }
    }
  }

  pub fn clamped(&self, ranges: [Range<T>; N]) -> Self {
    let mut ret = self.clone();
    ret.clamp(ranges);
    ret
  }
}

impl<T: Ord + Copy, const N: usize> Vector<T, N> {
  pub fn min(&self, rhs: &Self) -> Self {
    let mut ret = self.clone();
    for i in 0..N {
      ret.0[i] = ret.0[i].min(rhs.0[i]);
    }
    ret
  }

  pub fn max(&self, rhs: &Self) -> Self {
    let mut ret = self.clone();
    for i in 0..N {
      ret.0[i] = ret.0[i].max(rhs.0[i]);
    }
    ret
  }
}

impl<
    T: FromPrimitive
      + ToPrimitive
      + Default
      + AddAssign<T>
      + Add<T, Output = T>
      + Copy
      + Sub<T, Output = T>
      + Unit
      + Mul<T, Output = T>
      + Div<T, Output = T>
      + PartialOrd,
    const N: usize,
  > Vector<T, N>
{
  pub fn dot(&self, rhs: &Self) -> T {
    let mut ret = T::zero();
    for i in 0..N {
      ret += self.0[i] * rhs.0[i];
    }
    ret
  }

  pub fn angle_between(&self, rhs: &Self) -> T {
    let dot_product = self.dot(rhs);
    let magnitudes = self.magnitude() * rhs.magnitude();
    let cos_theta = dot_product / magnitudes;
    // Clamp value to the range [-1, 1] to avoid NaN due to floating-point precision issues
    let cos_theta = cos_theta.to_primitive().clamp(-1.0, 1.0);

    T::from_primitive(cos_theta.acos()) // Returns angle in radians
  }
}

impl<const N: usize> Vector<f32, N> {
  pub fn lerp(&self, rhs: &Self, t: f32) -> Self {
    let mut ret = self.clone();
    for i in 0..N {
      ret.0[i] = (1.0 - t) * self.0[0] + t * rhs.0[0];
    }
    return ret;
  }
}

impl<const N: usize> Vector<f64, N> {
  pub fn lerp(&self, rhs: &Self, t: f64) -> Self {
    let mut ret = self.clone();
    for i in 0..N {
      ret.0[i] = (1.0 - t) * self.0[0] + t * rhs.0[0];
    }
    if N == 4 {
      ret.0[3] = 1.0
    }
    return ret;
  }
}

impl<
    T: Copy
      + FromPrimitive
      + ToPrimitive
      + Default
      + AddAssign<T>
      + Add<T, Output = T>
      + Copy
      + Sub<T, Output = T>
      + SubAssign<T>
      + Unit
      + Mul<T, Output = T>
      + MulAssign<T>
      + Div<T, Output = T>
      + DivAssign<T>
      + PartialOrd,
    const N: usize,
  > Vector<T, N>
{
  pub fn project(&self, onto: &Self) -> Self {
    let dot_product = self.dot(onto);
    let onto_mag = onto.magnitude();
    let magnitude_squared = onto_mag * onto_mag;
    let scalar = dot_product / magnitude_squared;
    *onto * scalar
  }

  // Reflect method
  pub fn reflect(&self, normal: &Self) -> Self {
    let dot_product = self.dot(normal);
    let mut ret = Self::default();
    for i in 0..N {
      ret.0[i] = self.0[i] - T::from_primitive(2.0) * dot_product * normal.0[i]
    }
    ret
  }

  // Distance method
  pub fn distance(&self, other: &Self) -> T {
    let diff = *self - *other;
    let mut ret = T::zero();
    for i in 0..N {
      ret += diff.0[i];
    }
    ret.sqrt()
  }
}

impl<
    T: Copy
      + FromPrimitive
      + ToPrimitive
      + Default
      + AddAssign<T>
      + Add<T, Output = T>
      + Copy
      + Sub<T, Output = T>
      + Unit
      + Mul<T, Output = T>
      + MulAssign<T>
      + Div<T, Output = T>
      + DivAssign<T>
      + PartialOrd,
  > Vector<T, 2>
{
  pub fn rotate(&self, angle: T) -> Self {
    let cos_theta = T::from_primitive(angle.to_primitive().cos());
    let sin_theta = T::from_primitive(angle.to_primitive().sin());

    Self([
      self.0[0] * cos_theta - self.0[1] * sin_theta,
      self.0[0] * sin_theta + self.0[1] * cos_theta,
    ])
  }
}

impl<
    T: Copy
      + FromPrimitive
      + ToPrimitive
      + Default
      + AddAssign<T>
      + Add<T, Output = T>
      + Copy
      + Sub<T, Output = T>
      + Unit
      + Mul<T, Output = T>
      + MulAssign<T>
      + Div<T, Output = T>
      + DivAssign<T>
      + PartialOrd,
  > Vector<T, 3>
{
  pub fn rotate(&self, axis: Self, angle: T) -> Self {
    let axis = axis.normalized();
    let cos_theta = T::from_primitive(angle.to_primitive().cos());
    let sin_theta = T::from_primitive(angle.to_primitive().sin());
    (*self * cos_theta)
      + axis.cross(self) * sin_theta
      + axis * (axis.dot(self) * (T::unit() - cos_theta))
  }
}

impl<
    T: Copy
      + FromPrimitive
      + ToPrimitive
      + Default
      + AddAssign<T>
      + Add<T, Output = T>
      + Copy
      + Sub<T, Output = T>
      + Unit
      + Mul<T, Output = T>
      + MulAssign<T>
      + Div<T, Output = T>
      + DivAssign<T>
      + PartialOrd,
  > Vector<T, 4>
{
  pub fn rotate(&self, axis: Self, angle: T) -> Self {
    let axis = axis.normalized();
    let cos_theta = T::from_primitive(angle.to_primitive().cos());
    let sin_theta = T::from_primitive(angle.to_primitive().sin());
    let mut ret = (*self * cos_theta)
      + axis.cross(self) * sin_theta
      + axis * (axis.dot(self) * (T::unit() - cos_theta));
    ret.0[3] = T::unit();
    ret
  }
}

impl<T: Copy, const N: usize> Vector<T, N> {
  pub fn scalar(v: T) -> Self {
    Self([v; N])
  }
}

impl<T: PartialEq, const N: usize> PartialEq for Vector<T, N> {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl<T: PartialOrd, const N: usize> PartialOrd for Vector<T, N> {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    self.0.partial_cmp(&other.0)
  }
}

impl<T: Hash, const N: usize> Hash for Vector<T, N> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.0.hash(state);
  }
}

impl<T, const N: usize> Index<usize> for Vector<T, N> {
  type Output = T;

  fn index(&self, index: usize) -> &Self::Output {
    self.get(index).unwrap()
  }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N> {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    self.get_mut(index).unwrap()
  }
}

impl<T, const N: usize> Deref for Vector<T, N> {
  type Target = [T; N];

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<T, const N: usize> DerefMut for Vector<T, N> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl<T> From<T> for Vector<T, 1> {
  fn from(value: T) -> Self {
    Vector([value])
  }
}

impl<T> From<(T, T)> for Vector<T, 2> {
  fn from(value: (T, T)) -> Self {
    Vector([value.0, value.1])
  }
}

impl<T> From<(T, T, T)> for Vector<T, 3> {
  fn from(value: (T, T, T)) -> Self {
    Vector([value.0, value.1, value.2])
  }
}

impl<T> From<(T, T, T, T)> for Vector<T, 4> {
  fn from(value: (T, T, T, T)) -> Self {
    Vector([value.0, value.1, value.2, value.3])
  }
}

impl<T: Copy> From<[T; 2]> for Vector<T, 2> {
  fn from(value: [T; 2]) -> Self {
    Vector([value[0], value[1]])
  }
}

impl<T: Copy> From<[T; 3]> for Vector<T, 3> {
  fn from(value: [T; 3]) -> Self {
    Vector([value[0], value[1], value[2]])
  }
}

impl<T: Copy> From<[T; 4]> for Vector<T, 4> {
  fn from(value: [T; 4]) -> Self {
    Vector([value[0], value[1], value[2], value[3]])
  }
}

impl<T: Copy> From<T> for Vector<T, 2> {
  fn from(value: T) -> Self {
    Vector::scalar(value)
  }
}

impl<T: Copy> From<T> for Vector<T, 3> {
  fn from(value: T) -> Self {
    Vector::scalar(value)
  }
}

impl<T: Copy> From<T> for Vector<T, 4> {
  fn from(value: T) -> Self {
    Vector::scalar(value)
  }
}

pub fn vec1<T: Copy>(x: T) -> Vector<T, 1> {
  Vector::from(x)
}

pub fn vec2<T: Copy>(x: T, y: T) -> Vector<T, 2> {
  Vector::from([x, y])
}

pub fn vec3<T: Copy>(x: T, y: T, z: T) -> Vector<T, 3> {
  Vector::from([x, y, z])
}

pub fn vec4<T: Copy>(x: T, y: T, z: T, w: T) -> Vector<T, 4> {
  Vector::from([x, y, z, w])
}

impl<T: Copy + Zero + Mul<T, Output = T> + AddAssign<T>, const N: usize> Vector<T, N> {
  pub fn sq_magnitude(&self) -> T {
    let mut ret = T::zero();
    for i in 0..N {
      ret += self.0[i] * self.0[i];
    }
    ret
  }
}

impl<T: Copy + Zero + Sqrt + Mul<T, Output = T> + AddAssign<T>, const N: usize> Vector<T, N> {
  pub fn magnitude(&self) -> T {
    self.sq_magnitude().sqrt()
  }
}
impl<T: Copy + Zero + Sqrt + Mul<T, Output = T> + AddAssign<T> + DivAssign<T>, const N: usize>
  Vector<T, N>
{
  pub fn normalize(&mut self) -> T {
    let mag = self.magnitude();
    for i in 0..N {
      self.0[i] /= mag;
    }
    return mag;
  }

  pub fn normalized(&self) -> Self {
    let mut ret = self.clone();
    ret.normalize();
    return ret;
  }
}

impl<T: Copy + Add<T, Output = T> + Mul<T, Output = T> + Sub<T, Output = T>> Vector<T, 2> {
  pub fn cross(&self, rhs: &Self) -> T {
    self.0[0] * rhs.0[1] - self.0[1] * rhs.0[0]
  }
}

impl<T: Copy + Default + Mul<T, Output = T> + Sub<T, Output = T> + Add<T, Output = T>>
  Vector<T, 3>
{
  pub fn cross(&self, rhs: &Self) -> Self {
    let mut ret = Self::default();
    ret.0[0] = self.0[1] * rhs.0[2] - self.0[2] * rhs.0[1];
    ret.0[1] = self.0[2] * rhs.0[0] - self.0[0] * rhs.0[2];
    ret.0[2] = self.0[0] * rhs.0[1] - self.0[1] * rhs.0[0];
    ret
  }
}

impl<T: Copy + Default + Unit + Add<T, Output = T> + Mul<T, Output = T> + Sub<T, Output = T>>
  Vector<T, 4>
{
  pub fn cross(&self, rhs: &Self) -> Self {
    let mut ret = Self::default();
    ret.0[0] = self.0[1] * rhs.0[2] - self.0[2] * rhs.0[1];
    ret.0[1] = self.0[2] * rhs.0[0] - self.0[0] * rhs.0[2];
    ret.0[2] = self.0[0] * rhs.0[1] - self.0[1] * rhs.0[0];
    ret.0[3] = T::unit();
    ret
  }
}

impl<T: Copy, const N: usize> Vector<T, N> {}

impl<T: Copy, const N: usize> Copy for Vector<T, N> {}

impl<T: Clone, const N: usize> Clone for Vector<T, N> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<T: Default + Copy, const N: usize> Default for Vector<T, N> {
  fn default() -> Self {
    Self([Default::default(); N])
  }
}

impl<T: Unit + Copy, const N: usize> Unit for Vector<T, N> {
  fn unit() -> Self {
    Self([T::unit(); N])
  }
}

impl<T: Debug, const N: usize> Debug for Vector<T, N> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("Vector").field(&self.0).finish()
  }
}

impl<T: Display, const N: usize> Display for Vector<T, N> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "(")?;
    let mut first = true;
    for v in &self.0 {
      if !first {
        write!(f, ", ")?;
      }
      write!(f, "{}", *v)?;
      first = false;
    }
    write!(f, ")")
  }
}

macro_rules! impl_unit {
  ($(($ty: ty, $ident: ident)),+) => {
    $(
      impl <T: Default + Copy + Unit + Neg<Output = T>> $ty {
        paste! {
          pub fn [<unit_ $ident>]() -> Self {
            let mut v = Self::default();
            v.[<set_ $ident>](T::unit());
            v
          }

          pub fn [<neg_unit_ $ident>]() -> Self {
            let mut v = Self::default();
            v.[<set_ $ident>](-T::unit());
            v
          }
        }
      }
    )+
  };
}

macro_rules! impl_alias {
  ($(($ty:ty, $comps: expr, $suffix:ident)),+) => {
    $(
      paste! {
        pub type [<Vec $comps $suffix>] = Vector<$ty, $comps>;
      }
    )+
  };
}

macro_rules! impl_aliases {
  ($(($ty:ty, $suffix:ident)),+) => {
    $(
      impl_alias!(
        ($ty, 1, $suffix),
        ($ty, 2, $suffix),
        ($ty, 3, $suffix),
        ($ty, 4, $suffix)
      );
    )+
  };
}

macro_rules! add_component {
  ($( ($ty: ty, $ident: ident, $i: expr) ),+) => {
    $(
      impl<T: Copy> $ty {
        pub fn $ident(&self) -> T {
          self.0[$i]
        }

        paste! {
          pub fn [<$ident _mut>](&mut self) -> &mut T {
            &mut self.0[$i]
          }

          pub fn [<set_ $ident>](&mut self, v: T) {
            self.0[$i] = v;
          }
        }
      }
    )+
  };
}

///////////////////////
// Vector / Vector ops
//

macro_rules! impl_op {
  ($trait:ty, $meth:ident, $op:tt) => {
    paste! {
      impl<T: Copy + [<$trait Assign>]<T>, const N: usize> [<$trait Assign>]<Self> for Vector<T, N> {
        fn [<$meth _assign>](&mut self, rhs: Self) {
          for i in 0..N {
            self.0[i] $op rhs.0[i];
          }
        }
      }

      impl<T: Copy + [<$trait Assign>]<T>, const N: usize> $trait<Self> for Vector<T, N> {
        type Output = Self;

        fn $meth(mut self, rhs: Self) -> Self::Output {
          self.[<$meth _assign>](rhs);
          self
        }
      }


      impl<T: Copy + [<$trait Assign>]<T>, const N: usize> $trait<T> for Vector<T, N> {
        type Output = Self;

        fn $meth(mut self, rhs: T) -> Self::Output {
          self.[<$meth _assign>](rhs);
          self
        }
      }

      impl<T: Copy + [<$trait Assign>]<T>, const N: usize> [<$trait Assign>]<T> for Vector<T, N> {
        fn [<$meth _assign>](&mut self, rhs: T) {
          for i in 0..N {
            self.0[i] $op rhs;
          }
        }
      }
    }
  };
}

impl<T: Copy + Neg<Output = T>, const N: usize> Neg for Vector<T, N> {
  type Output = Self;

  fn neg(mut self) -> Self::Output {
    for i in 0..N {
      self.0[i] = -self.0[i];
    }
    self
  }
}

impl<T: Copy> Vector<T, 1> {
  pub fn extend(&self, by: T) -> Vector<T, 2> {
    return Vector([self.0[0], by]);
  }
}

impl<T: Copy> Vector<T, 2> {
  pub fn extend(&self, by: T) -> Vector<T, 3> {
    return Vector([self.0[0], self.0[1], by]);
  }

  pub fn shrink(&self) -> Vector<T, 1> {
    return Vector([self.0[0]]);
  }
}

impl<T: Copy> Vector<T, 3> {
  pub fn extend(&self, by: T) -> Vector<T, 4> {
    return Vector([self.0[0], self.0[1], self.0[2], by]);
  }

  pub fn shrink(&self) -> Vector<T, 2> {
    return Vector([self.0[0], self.0[1]]);
  }
}

impl_op!(Add, add, +=);
impl_op!(Sub, sub, -=);
impl_op!(Mul, mul, *=);
impl_op!(Div, div, /=);

add_component!(
  (Vector<T, 1>, x, 0),
  (Vector<T, 2>, x, 0),
  (Vector<T, 3>, x, 0),
  (Vector<T, 4>, x, 0),

  (Vector<T, 2>, y, 1),
  (Vector<T, 3>, y, 1),
  (Vector<T, 4>, y, 1),

  (Vector<T, 3>, z, 2),
  (Vector<T, 4>, z, 2),

  (Vector<T, 4>, w, 3)
);

impl_unit!(
  // unit_x
  (Vector<T, 1>, x),
  (Vector<T, 2>, x),
  (Vector<T, 3>, x),
  (Vector<T, 4>, x),
  // unit_y
  (Vector<T, 2>, y),
  (Vector<T, 3>, y),
  (Vector<T, 4>, y),
  // unit_z
  (Vector<T, 3>, z),
  (Vector<T, 4>, z),
  // unit_w
  (Vector<T, 4>, w)
);

impl_aliases!(
  // floats
  (f32, f),
  (f64, d),
  // signed
  (i8, i8),
  (i16, i16),
  (i32, i32),
  (i64, i64),
  (i128, i128),
  (isize, is),
  // unsigned
  (u8, u8),
  (u16, u16),
  (u32, u32),
  (u64, u64),
  (u128, u128),
  (usize, us),
  // others
  (bool, b)
);

#[cfg(test)]
mod tests {
  use crate::{vec2, vec3, Vec2i8, Vec3d, Vec3i8};

  use super::Vector;

  #[test]
  fn from_tuple() {
    let v4 = Vec2i8::from((2i8, 3i8));
    assert_eq!(v4, Vector::new([2i8, 3i8]));
  }

  #[test]
  fn from_scalar() {
    let v4 = Vec2i8::from(2i8);
    assert_eq!(v4, Vector::new([2i8, 2i8]));
  }

  #[test]
  fn sq_magnitude() {
    let v4 = vec2(2i8, 3i8);
    assert_eq!(v4.sq_magnitude(), 13i8);
  }

  #[test]
  fn magnitude() {
    let v4 = vec2(2i8, 3i8);
    assert_eq!(v4.magnitude(), 13f64.sqrt() as i8);
  }

  #[test]
  fn normalize() {
    let mut v4 = vec2(2f32, 3f32);
    let mag = v4.normalize();
    assert_eq!(mag, 13f32.sqrt() as f32);
    assert_eq!(v4, vec2(0.5547002f32, 0.8320503f32));
  }

  #[test]
  fn clamp() {
    let v = vec2(4f32, 5f32).clamped([1f32..2f32, 3f32..4f32]);
    assert_eq!(v, vec2(2f32, 4f32))
  }

  #[test]
  fn dot_2d() {
    let a = vec2(3f32, 2f32);
    let b = vec2(4f32, 5f32);
    assert_eq!(a.dot(&b), 22.0)
  }

  #[test]
  fn dot_3d() {
    let a = vec3(3f32, 2f32, 2f32);
    let b = vec3(4f32, 5f32, 2f32);
    assert_eq!(a.dot(&b), 26.0)
  }

  #[test]
  fn cross_2d() {
    let a = vec2(3f32, 2f32);
    let b = vec2(4f32, 5f32);
    assert_eq!(a.cross(&b), 7.0)
  }

  #[test]
  fn cross_3d() {
    let a = vec3(3f32, 2f32, 2f32);
    let b = vec3(4f32, 5f32, 2f32);
    assert_eq!(a.cross(&b), vec3(-6f32, 2f32, 7f32))
  }
}
