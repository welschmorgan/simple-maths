pub trait Unit {
  fn unit() -> Self;
}

macro_rules! impl_unit {
  ($($ty: ty),+) => {
    $(
      impl Unit for $ty {
        fn unit() -> Self {
          return 1 as $ty
        }
      }
    )+
  };
}

impl_unit!(f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

impl Unit for bool {
  fn unit() -> Self {
    true
  }
}
