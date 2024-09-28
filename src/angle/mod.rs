pub mod degree;
pub mod radian;

mod macros;

pub use degree::*;
pub use radian::*;

pub const DEG2RAD: f64 = std::f64::consts::PI / 180.0;
pub const RAD2DEG: f64 = 180.0 / std::f64::consts::PI;

macro_rules! impl_op {
  ($ty:ty, $trait:ty, $meth:ident, $op:tt) => {
    paste! {
      impl<T: Copy + [<$trait Assign>]<T>> [<$trait Assign>]<Self> for $ty<T> {
        fn [<$meth _assign>](&mut self, rhs: Self) {
          self.0 $op rhs.0;
        }
      }

      impl<T: Copy + [<$trait Assign>]<T>> $trait<Self> for $ty<T> {
        type Output = Self;

        fn $meth(mut self, rhs: Self) -> Self::Output {
          self.[<$meth _assign>](rhs);
          self
        }
      }


      impl<T: Copy + [<$trait Assign>]<T>> $trait<T> for $ty<T> {
        type Output = Self;

        fn $meth(mut self, rhs: T) -> Self::Output {
          self.[<$meth _assign>](rhs);
          self
        }
      }

      impl<T: Copy + [<$trait Assign>]<T>> [<$trait Assign>]<T> for $ty<T> {
        fn [<$meth _assign>](&mut self, rhs: T) {
          self.0 $op rhs;
        }
      }
    }
  };
}

pub(crate) use impl_op;
