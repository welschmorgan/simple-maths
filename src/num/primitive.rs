pub trait ToPrimitive {
  fn to_primitive(&self) -> f64;
}

macro_rules! impl_to_primitive {
  ($($ty: ty),+) => {
    $(
      impl ToPrimitive for $ty {
        fn to_primitive(&self) -> f64 {
          *self as f64
        }
      }
    )+
  };
}

impl_to_primitive!(f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

pub trait FromPrimitive {
  fn from_primitive(v: f64) -> Self;
}

macro_rules! impl_from_primitive {
  ($($ty: ty),+) => {
    $(
      impl FromPrimitive for $ty {
        fn from_primitive(v: f64) -> Self {
          v as Self
        }
      }
    )+
  };
}

impl_from_primitive!(f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
