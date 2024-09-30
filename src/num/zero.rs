pub trait Zero {
  fn zero() -> Self;
}

impl<T: Default> Zero for T {
  fn zero() -> Self {
    Self::default()
  }
}
