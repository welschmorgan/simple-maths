use crate::{FromPrimitive, ToPrimitive, Zero};
use paste::paste;

macro_rules! impl_angle {
  ($ty:ty, $repr:expr, $min:expr, $max:expr) => {
    paste! {
      impl<T> $ty<T> {
        pub const REPR: &'static str = $repr;
      }


      impl<T: Copy + PartialOrd + std::ops::Rem<Output = T> + Sub<T, Output = T> + Neg<Output = T> + $crate::ToPrimitive + FromPrimitive> $ty<T>
      {
        pub fn new(mut v: T) -> Self {
          let min = T::from_primitive($min);
          let max = T::from_primitive($max);
          if v < min {
            v = max - (-v % max);
          }
          if v >= max {
            v = v % max
          }
          Self(v)
        }
      }

      impl<T: Copy> $ty<T> {
        pub fn value(&self) -> T {
          self.0
        }
      }
      impl<T: Copy> Deref for $ty<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
          &self.0
        }
      }


      impl<T: Copy> DerefMut for $ty<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
          &mut self.0
        }
      }


      impl<T: Copy
      + FromPrimitive
      + Sub<T, Output = T>
      + Neg<Output = T>
      + Rem<T, Output = T>
      + Mul<T, Output = T>
      + MulAssign<T>
      + PartialOrd
      + Zero
      + ToPrimitive
      + FromPrimitive> Neg for $ty<T> {
        type Output = Self;

        fn neg(self) -> Self::Output {
          Self::new(-self.0)
        }
      }
    }
    impl_op!($ty, Add, add, +=);
    impl_op!($ty, Sub, sub, -=);
    impl_op!($ty, Mul, mul, *=);
    impl_op!($ty, Div, div, /=);


    paste! {
      impl<T: Copy
              + FromPrimitive
              + Sub<T, Output = T>
              + Neg<Output = T>
              + Rem<T, Output = T>
              + Mul<T, Output = T>
              + MulAssign<T>
              + PartialOrd
              + Zero
              + ToPrimitive
              + FromPrimitive
      > From<T> for $ty<T> {
        fn from(value: T) -> Self {
          Self::new(value)
        }
      }

      impl<T: Display> std::fmt::Display for $ty<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
          write!(f, "{}{}", self.0, Self::REPR)
        }
      }

      impl<T: Debug> std::fmt::Debug for $ty<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
          f.debug_tuple(stringify!($ty)).field(&self.0).finish()
        }
      }

      impl<T: $crate::ToPrimitive> $crate::ToPrimitive for $ty<T> {
        fn to_primitive(&self) -> f64 {
          self.0.to_primitive()
        }
      }

      impl<T: $crate::FromPrimitive> $crate::FromPrimitive for $ty<T> {
        fn from_primitive(v: f64) -> $ty<T> {
          Self(T::from_primitive(v))
        }
      }
    }
  };
}

pub(crate) use impl_angle;
