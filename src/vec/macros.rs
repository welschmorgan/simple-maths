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
      crate::vec::macros::impl_alias!(
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

pub(crate) use add_component;
pub(crate) use impl_alias;
pub(crate) use impl_aliases;
pub(crate) use impl_op;
pub(crate) use impl_unit;
