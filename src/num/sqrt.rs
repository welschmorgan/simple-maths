use super::{FromPrimitive, ToPrimitive};

pub trait Sqrt {
  fn sqrt(&self) -> Self;
}

impl<T: ToPrimitive + FromPrimitive> Sqrt for T {
  fn sqrt(&self) -> Self {
    Self::from_primitive(self.to_primitive().sqrt())
  }
}
