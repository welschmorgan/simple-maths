use super::Vector;

impl<T: Copy> Vector<T, 2> {
  pub fn xx(&self) -> Self {
    return Self::from([(*self)[0], (*self)[0]]);
  }
  pub fn xy(&self) -> Self {
    return Self::from([(*self)[0], (*self)[1]]);
  }
  pub fn yx(&self) -> Self {
    return Self::from([(*self)[1], (*self)[0]]);
  }
  pub fn yy(&self) -> Self {
    return Self::from([(*self)[1], (*self)[1]]);
  }
}

impl<T: Copy> Vector<T, 3> {
  pub fn xx(&self) -> Vector<T, 2> {
    return Vector::<T, 2>::from([(*self)[0], (*self)[0]]);
  }
  pub fn xy(&self) -> Vector<T, 2> {
    return Vector::<T, 2>::from([(*self)[0], (*self)[1]]);
  }
  pub fn yx(&self) -> Vector<T, 2> {
    return Vector::<T, 2>::from([(*self)[1], (*self)[0]]);
  }
  pub fn yy(&self) -> Vector<T, 2> {
    return Vector::<T, 2>::from([(*self)[1], (*self)[1]]);
  }

  pub fn xxx(&self) -> Self {
    return Self::from([(*self)[0], (*self)[0], (*self)[0]]);
  }
  pub fn xxy(&self) -> Self {
    return Self::from([(*self)[0], (*self)[0], (*self)[1]]);
  }
  pub fn xxz(&self) -> Self {
    return Self::from([(*self)[0], (*self)[0], (*self)[2]]);
  }
  pub fn xyx(&self) -> Self {
    return Self::from([(*self)[0], (*self)[1], (*self)[0]]);
  }
  pub fn xyy(&self) -> Self {
    return Self::from([(*self)[0], (*self)[1], (*self)[1]]);
  }
  pub fn xyz(&self) -> Self {
    return Self::from([(*self)[0], (*self)[1], (*self)[2]]);
  }
  pub fn xzx(&self) -> Self {
    return Self::from([(*self)[0], (*self)[2], (*self)[0]]);
  }
  pub fn xzy(&self) -> Self {
    return Self::from([(*self)[0], (*self)[2], (*self)[1]]);
  }
  pub fn xzz(&self) -> Self {
    return Self::from([(*self)[0], (*self)[2], (*self)[2]]);
  }
  pub fn yxx(&self) -> Self {
    return Self::from([(*self)[1], (*self)[0], (*self)[0]]);
  }
  pub fn yxy(&self) -> Self {
    return Self::from([(*self)[1], (*self)[0], (*self)[1]]);
  }
  pub fn yxz(&self) -> Self {
    return Self::from([(*self)[1], (*self)[0], (*self)[2]]);
  }
  pub fn yyx(&self) -> Self {
    return Self::from([(*self)[1], (*self)[1], (*self)[0]]);
  }
  pub fn yyy(&self) -> Self {
    return Self::from([(*self)[1], (*self)[1], (*self)[1]]);
  }
  pub fn yyz(&self) -> Self {
    return Self::from([(*self)[1], (*self)[1], (*self)[2]]);
  }
  pub fn yzx(&self) -> Self {
    return Self::from([(*self)[1], (*self)[2], (*self)[0]]);
  }
  pub fn yzy(&self) -> Self {
    return Self::from([(*self)[1], (*self)[2], (*self)[1]]);
  }
  pub fn yzz(&self) -> Self {
    return Self::from([(*self)[1], (*self)[2], (*self)[2]]);
  }
  pub fn zxx(&self) -> Self {
    return Self::from([(*self)[2], (*self)[0], (*self)[0]]);
  }
  pub fn zxy(&self) -> Self {
    return Self::from([(*self)[2], (*self)[0], (*self)[1]]);
  }
  pub fn zxz(&self) -> Self {
    return Self::from([(*self)[2], (*self)[0], (*self)[2]]);
  }
  pub fn zyx(&self) -> Self {
    return Self::from([(*self)[2], (*self)[1], (*self)[0]]);
  }
  pub fn zyy(&self) -> Self {
    return Self::from([(*self)[2], (*self)[1], (*self)[1]]);
  }
  pub fn zyz(&self) -> Self {
    return Self::from([(*self)[2], (*self)[1], (*self)[2]]);
  }
  pub fn zzx(&self) -> Self {
    return Self::from([(*self)[2], (*self)[2], (*self)[0]]);
  }
  pub fn zzy(&self) -> Self {
    return Self::from([(*self)[2], (*self)[2], (*self)[1]]);
  }
  pub fn zzz(&self) -> Self {
    return Self::from([(*self)[2], (*self)[2], (*self)[2]]);
  }
}
impl<T: Copy> Vector<T, 4> {
  pub fn xx(&self) -> Vector<T, 2> {
    return Vector::<T, 2>::from([(*self)[0], (*self)[0]]);
  }
  pub fn xy(&self) -> Vector<T, 2> {
    return Vector::<T, 2>::from([(*self)[0], (*self)[1]]);
  }
  pub fn yx(&self) -> Vector<T, 2> {
    return Vector::<T, 2>::from([(*self)[1], (*self)[0]]);
  }
  pub fn yy(&self) -> Vector<T, 2> {
    return Vector::<T, 2>::from([(*self)[1], (*self)[1]]);
  }

  pub fn xxx(&self) -> Vector<T, 3> {
    return Vector::<T, 3>::from([(*self)[0], (*self)[0], (*self)[0]]);
  }
  pub fn xxy(&self) -> Vector<T, 3> {
    return Vector::<T, 3>::from([(*self)[0], (*self)[0], (*self)[1]]);
  }
  pub fn xxz(&self) -> Vector<T, 3> {
    return Vector::<T, 3>::from([(*self)[0], (*self)[0], (*self)[2]]);
  }
  pub fn xyx(&self) -> Vector<T, 3> {
    return Vector::<T, 3>::from([(*self)[0], (*self)[1], (*self)[0]]);
  }
  pub fn xyy(&self) -> Vector<T, 3> {
    return Vector::<T, 3>::from([(*self)[0], (*self)[1], (*self)[1]]);
  }
  pub fn xyz(&self) -> Vector<T, 3> {
    return Vector::<T, 3>::from([(*self)[0], (*self)[1], (*self)[2]]);
  }
  pub fn xzx(&self) -> Vector<T, 3> {
    return Vector::<T, 3>::from([(*self)[0], (*self)[2], (*self)[0]]);
  }
  pub fn xzy(&self) -> Vector<T, 3> {
    return Vector::<T, 3>::from([(*self)[0], (*self)[2], (*self)[1]]);
  }
  pub fn xzz(&self) -> Vector<T, 3> {
    return Vector::<T, 3>::from([(*self)[0], (*self)[2], (*self)[2]]);
  }
  pub fn yxx(&self) -> Vector<T, 3> {
    return Vector::<T, 3>::from([(*self)[1], (*self)[0], (*self)[0]]);
  }
  pub fn yxy(&self) -> Vector<T, 3> {
    return Vector::<T, 3>::from([(*self)[1], (*self)[0], (*self)[1]]);
  }
  pub fn yxz(&self) -> Vector<T, 3> {
    return Vector::<T, 3>::from([(*self)[1], (*self)[0], (*self)[2]]);
  }
  pub fn yyx(&self) -> Vector<T, 3> {
    return Vector::<T, 3>::from([(*self)[1], (*self)[1], (*self)[0]]);
  }
  pub fn yyy(&self) -> Vector<T, 3> {
    return Vector::<T, 3>::from([(*self)[1], (*self)[1], (*self)[1]]);
  }
  pub fn yyz(&self) -> Vector<T, 3> {
    return Vector::<T, 3>::from([(*self)[1], (*self)[1], (*self)[2]]);
  }
  pub fn yzx(&self) -> Vector<T, 3> {
    return Vector::<T, 3>::from([(*self)[1], (*self)[2], (*self)[0]]);
  }
  pub fn yzy(&self) -> Vector<T, 3> {
    return Vector::<T, 3>::from([(*self)[1], (*self)[2], (*self)[1]]);
  }
  pub fn yzz(&self) -> Vector<T, 3> {
    return Vector::<T, 3>::from([(*self)[1], (*self)[2], (*self)[2]]);
  }
  pub fn zxx(&self) -> Vector<T, 3> {
    return Vector::<T, 3>::from([(*self)[2], (*self)[0], (*self)[0]]);
  }
  pub fn zxy(&self) -> Vector<T, 3> {
    return Vector::<T, 3>::from([(*self)[2], (*self)[0], (*self)[1]]);
  }
  pub fn zxz(&self) -> Vector<T, 3> {
    return Vector::<T, 3>::from([(*self)[2], (*self)[0], (*self)[2]]);
  }
  pub fn zyx(&self) -> Vector<T, 3> {
    return Vector::<T, 3>::from([(*self)[2], (*self)[1], (*self)[0]]);
  }
  pub fn zyy(&self) -> Vector<T, 3> {
    return Vector::<T, 3>::from([(*self)[2], (*self)[1], (*self)[1]]);
  }
  pub fn zyz(&self) -> Vector<T, 3> {
    return Vector::<T, 3>::from([(*self)[2], (*self)[1], (*self)[2]]);
  }
  pub fn zzx(&self) -> Vector<T, 3> {
    return Vector::<T, 3>::from([(*self)[2], (*self)[2], (*self)[0]]);
  }
  pub fn zzy(&self) -> Vector<T, 3> {
    return Vector::<T, 3>::from([(*self)[2], (*self)[2], (*self)[1]]);
  }
  pub fn zzz(&self) -> Vector<T, 3> {
    return Vector::<T, 3>::from([(*self)[2], (*self)[2], (*self)[2]]);
  }

  pub fn xxxx(&self) -> Self {
    return Self::from([(*self)[0], (*self)[0], (*self)[0], (*self)[0]]);
  }
  pub fn xxxy(&self) -> Self {
    return Self::from([(*self)[0], (*self)[0], (*self)[0], (*self)[1]]);
  }
  pub fn xxxz(&self) -> Self {
    return Self::from([(*self)[0], (*self)[0], (*self)[0], (*self)[2]]);
  }
  pub fn xxxw(&self) -> Self {
    return Self::from([(*self)[0], (*self)[0], (*self)[0], (*self)[3]]);
  }
  pub fn xxyx(&self) -> Self {
    return Self::from([(*self)[0], (*self)[0], (*self)[1], (*self)[0]]);
  }
  pub fn xxyy(&self) -> Self {
    return Self::from([(*self)[0], (*self)[0], (*self)[1], (*self)[1]]);
  }
  pub fn xxyz(&self) -> Self {
    return Self::from([(*self)[0], (*self)[0], (*self)[1], (*self)[2]]);
  }
  pub fn xxyw(&self) -> Self {
    return Self::from([(*self)[0], (*self)[0], (*self)[1], (*self)[3]]);
  }
  pub fn xxzx(&self) -> Self {
    return Self::from([(*self)[0], (*self)[0], (*self)[2], (*self)[0]]);
  }
  pub fn xxzy(&self) -> Self {
    return Self::from([(*self)[0], (*self)[0], (*self)[2], (*self)[1]]);
  }
  pub fn xxzz(&self) -> Self {
    return Self::from([(*self)[0], (*self)[0], (*self)[2], (*self)[2]]);
  }
  pub fn xxzw(&self) -> Self {
    return Self::from([(*self)[0], (*self)[0], (*self)[2], (*self)[3]]);
  }
  pub fn xxwx(&self) -> Self {
    return Self::from([(*self)[0], (*self)[0], (*self)[3], (*self)[0]]);
  }
  pub fn xxwy(&self) -> Self {
    return Self::from([(*self)[0], (*self)[0], (*self)[3], (*self)[1]]);
  }
  pub fn xxwz(&self) -> Self {
    return Self::from([(*self)[0], (*self)[0], (*self)[3], (*self)[2]]);
  }
  pub fn xxww(&self) -> Self {
    return Self::from([(*self)[0], (*self)[0], (*self)[3], (*self)[3]]);
  }
  pub fn xyxx(&self) -> Self {
    return Self::from([(*self)[0], (*self)[1], (*self)[0], (*self)[0]]);
  }
  pub fn xyxy(&self) -> Self {
    return Self::from([(*self)[0], (*self)[1], (*self)[0], (*self)[1]]);
  }
  pub fn xyxz(&self) -> Self {
    return Self::from([(*self)[0], (*self)[1], (*self)[0], (*self)[2]]);
  }
  pub fn xyxw(&self) -> Self {
    return Self::from([(*self)[0], (*self)[1], (*self)[0], (*self)[3]]);
  }
  pub fn xyyx(&self) -> Self {
    return Self::from([(*self)[0], (*self)[1], (*self)[1], (*self)[0]]);
  }
  pub fn xyyy(&self) -> Self {
    return Self::from([(*self)[0], (*self)[1], (*self)[1], (*self)[1]]);
  }
  pub fn xyyz(&self) -> Self {
    return Self::from([(*self)[0], (*self)[1], (*self)[1], (*self)[2]]);
  }
  pub fn xyyw(&self) -> Self {
    return Self::from([(*self)[0], (*self)[1], (*self)[1], (*self)[3]]);
  }
  pub fn xyzx(&self) -> Self {
    return Self::from([(*self)[0], (*self)[1], (*self)[2], (*self)[0]]);
  }
  pub fn xyzy(&self) -> Self {
    return Self::from([(*self)[0], (*self)[1], (*self)[2], (*self)[1]]);
  }
  pub fn xyzz(&self) -> Self {
    return Self::from([(*self)[0], (*self)[1], (*self)[2], (*self)[2]]);
  }
  pub fn xyzw(&self) -> Self {
    return Self::from([(*self)[0], (*self)[1], (*self)[2], (*self)[3]]);
  }
  pub fn xywx(&self) -> Self {
    return Self::from([(*self)[0], (*self)[1], (*self)[3], (*self)[0]]);
  }
  pub fn xywy(&self) -> Self {
    return Self::from([(*self)[0], (*self)[1], (*self)[3], (*self)[1]]);
  }
  pub fn xywz(&self) -> Self {
    return Self::from([(*self)[0], (*self)[1], (*self)[3], (*self)[2]]);
  }
  pub fn xyww(&self) -> Self {
    return Self::from([(*self)[0], (*self)[1], (*self)[3], (*self)[3]]);
  }
  pub fn xzxx(&self) -> Self {
    return Self::from([(*self)[0], (*self)[2], (*self)[0], (*self)[0]]);
  }
  pub fn xzxy(&self) -> Self {
    return Self::from([(*self)[0], (*self)[2], (*self)[0], (*self)[1]]);
  }
  pub fn xzxz(&self) -> Self {
    return Self::from([(*self)[0], (*self)[2], (*self)[0], (*self)[2]]);
  }
  pub fn xzxw(&self) -> Self {
    return Self::from([(*self)[0], (*self)[2], (*self)[0], (*self)[3]]);
  }
  pub fn xzyx(&self) -> Self {
    return Self::from([(*self)[0], (*self)[2], (*self)[1], (*self)[0]]);
  }
  pub fn xzyy(&self) -> Self {
    return Self::from([(*self)[0], (*self)[2], (*self)[1], (*self)[1]]);
  }
  pub fn xzyz(&self) -> Self {
    return Self::from([(*self)[0], (*self)[2], (*self)[1], (*self)[2]]);
  }
  pub fn xzyw(&self) -> Self {
    return Self::from([(*self)[0], (*self)[2], (*self)[1], (*self)[3]]);
  }
  pub fn xzzx(&self) -> Self {
    return Self::from([(*self)[0], (*self)[2], (*self)[2], (*self)[0]]);
  }
  pub fn xzzy(&self) -> Self {
    return Self::from([(*self)[0], (*self)[2], (*self)[2], (*self)[1]]);
  }
  pub fn xzzz(&self) -> Self {
    return Self::from([(*self)[0], (*self)[2], (*self)[2], (*self)[2]]);
  }
  pub fn xzzw(&self) -> Self {
    return Self::from([(*self)[0], (*self)[2], (*self)[2], (*self)[3]]);
  }
  pub fn xzwx(&self) -> Self {
    return Self::from([(*self)[0], (*self)[2], (*self)[3], (*self)[0]]);
  }
  pub fn xzwy(&self) -> Self {
    return Self::from([(*self)[0], (*self)[2], (*self)[3], (*self)[1]]);
  }
  pub fn xzwz(&self) -> Self {
    return Self::from([(*self)[0], (*self)[2], (*self)[3], (*self)[2]]);
  }
  pub fn xzww(&self) -> Self {
    return Self::from([(*self)[0], (*self)[2], (*self)[3], (*self)[3]]);
  }
  pub fn xwxx(&self) -> Self {
    return Self::from([(*self)[0], (*self)[3], (*self)[0], (*self)[0]]);
  }
  pub fn xwxy(&self) -> Self {
    return Self::from([(*self)[0], (*self)[3], (*self)[0], (*self)[1]]);
  }
  pub fn xwxz(&self) -> Self {
    return Self::from([(*self)[0], (*self)[3], (*self)[0], (*self)[2]]);
  }
  pub fn xwxw(&self) -> Self {
    return Self::from([(*self)[0], (*self)[3], (*self)[0], (*self)[3]]);
  }
  pub fn xwyx(&self) -> Self {
    return Self::from([(*self)[0], (*self)[3], (*self)[1], (*self)[0]]);
  }
  pub fn xwyy(&self) -> Self {
    return Self::from([(*self)[0], (*self)[3], (*self)[1], (*self)[1]]);
  }
  pub fn xwyz(&self) -> Self {
    return Self::from([(*self)[0], (*self)[3], (*self)[1], (*self)[2]]);
  }
  pub fn xwyw(&self) -> Self {
    return Self::from([(*self)[0], (*self)[3], (*self)[1], (*self)[3]]);
  }
  pub fn xwzx(&self) -> Self {
    return Self::from([(*self)[0], (*self)[3], (*self)[2], (*self)[0]]);
  }
  pub fn xwzy(&self) -> Self {
    return Self::from([(*self)[0], (*self)[3], (*self)[2], (*self)[1]]);
  }
  pub fn xwzz(&self) -> Self {
    return Self::from([(*self)[0], (*self)[3], (*self)[2], (*self)[2]]);
  }
  pub fn xwzw(&self) -> Self {
    return Self::from([(*self)[0], (*self)[3], (*self)[2], (*self)[3]]);
  }
  pub fn xwwx(&self) -> Self {
    return Self::from([(*self)[0], (*self)[3], (*self)[3], (*self)[0]]);
  }
  pub fn xwwy(&self) -> Self {
    return Self::from([(*self)[0], (*self)[3], (*self)[3], (*self)[1]]);
  }
  pub fn xwwz(&self) -> Self {
    return Self::from([(*self)[0], (*self)[3], (*self)[3], (*self)[2]]);
  }
  pub fn xwww(&self) -> Self {
    return Self::from([(*self)[0], (*self)[3], (*self)[3], (*self)[3]]);
  }
  pub fn yxxx(&self) -> Self {
    return Self::from([(*self)[1], (*self)[0], (*self)[0], (*self)[0]]);
  }
  pub fn yxxy(&self) -> Self {
    return Self::from([(*self)[1], (*self)[0], (*self)[0], (*self)[1]]);
  }
  pub fn yxxz(&self) -> Self {
    return Self::from([(*self)[1], (*self)[0], (*self)[0], (*self)[2]]);
  }
  pub fn yxxw(&self) -> Self {
    return Self::from([(*self)[1], (*self)[0], (*self)[0], (*self)[3]]);
  }
  pub fn yxyx(&self) -> Self {
    return Self::from([(*self)[1], (*self)[0], (*self)[1], (*self)[0]]);
  }
  pub fn yxyy(&self) -> Self {
    return Self::from([(*self)[1], (*self)[0], (*self)[1], (*self)[1]]);
  }
  pub fn yxyz(&self) -> Self {
    return Self::from([(*self)[1], (*self)[0], (*self)[1], (*self)[2]]);
  }
  pub fn yxyw(&self) -> Self {
    return Self::from([(*self)[1], (*self)[0], (*self)[1], (*self)[3]]);
  }
  pub fn yxzx(&self) -> Self {
    return Self::from([(*self)[1], (*self)[0], (*self)[2], (*self)[0]]);
  }
  pub fn yxzy(&self) -> Self {
    return Self::from([(*self)[1], (*self)[0], (*self)[2], (*self)[1]]);
  }
  pub fn yxzz(&self) -> Self {
    return Self::from([(*self)[1], (*self)[0], (*self)[2], (*self)[2]]);
  }
  pub fn yxzw(&self) -> Self {
    return Self::from([(*self)[1], (*self)[0], (*self)[2], (*self)[3]]);
  }
  pub fn yxwx(&self) -> Self {
    return Self::from([(*self)[1], (*self)[0], (*self)[3], (*self)[0]]);
  }
  pub fn yxwy(&self) -> Self {
    return Self::from([(*self)[1], (*self)[0], (*self)[3], (*self)[1]]);
  }
  pub fn yxwz(&self) -> Self {
    return Self::from([(*self)[1], (*self)[0], (*self)[3], (*self)[2]]);
  }
  pub fn yxww(&self) -> Self {
    return Self::from([(*self)[1], (*self)[0], (*self)[3], (*self)[3]]);
  }
  pub fn yyxx(&self) -> Self {
    return Self::from([(*self)[1], (*self)[1], (*self)[0], (*self)[0]]);
  }
  pub fn yyxy(&self) -> Self {
    return Self::from([(*self)[1], (*self)[1], (*self)[0], (*self)[1]]);
  }
  pub fn yyxz(&self) -> Self {
    return Self::from([(*self)[1], (*self)[1], (*self)[0], (*self)[2]]);
  }
  pub fn yyxw(&self) -> Self {
    return Self::from([(*self)[1], (*self)[1], (*self)[0], (*self)[3]]);
  }
  pub fn yyyx(&self) -> Self {
    return Self::from([(*self)[1], (*self)[1], (*self)[1], (*self)[0]]);
  }
  pub fn yyyy(&self) -> Self {
    return Self::from([(*self)[1], (*self)[1], (*self)[1], (*self)[1]]);
  }
  pub fn yyyz(&self) -> Self {
    return Self::from([(*self)[1], (*self)[1], (*self)[1], (*self)[2]]);
  }
  pub fn yyyw(&self) -> Self {
    return Self::from([(*self)[1], (*self)[1], (*self)[1], (*self)[3]]);
  }
  pub fn yyzx(&self) -> Self {
    return Self::from([(*self)[1], (*self)[1], (*self)[2], (*self)[0]]);
  }
  pub fn yyzy(&self) -> Self {
    return Self::from([(*self)[1], (*self)[1], (*self)[2], (*self)[1]]);
  }
  pub fn yyzz(&self) -> Self {
    return Self::from([(*self)[1], (*self)[1], (*self)[2], (*self)[2]]);
  }
  pub fn yyzw(&self) -> Self {
    return Self::from([(*self)[1], (*self)[1], (*self)[2], (*self)[3]]);
  }
  pub fn yywx(&self) -> Self {
    return Self::from([(*self)[1], (*self)[1], (*self)[3], (*self)[0]]);
  }
  pub fn yywy(&self) -> Self {
    return Self::from([(*self)[1], (*self)[1], (*self)[3], (*self)[1]]);
  }
  pub fn yywz(&self) -> Self {
    return Self::from([(*self)[1], (*self)[1], (*self)[3], (*self)[2]]);
  }
  pub fn yyww(&self) -> Self {
    return Self::from([(*self)[1], (*self)[1], (*self)[3], (*self)[3]]);
  }
  pub fn yzxx(&self) -> Self {
    return Self::from([(*self)[1], (*self)[2], (*self)[0], (*self)[0]]);
  }
  pub fn yzxy(&self) -> Self {
    return Self::from([(*self)[1], (*self)[2], (*self)[0], (*self)[1]]);
  }
  pub fn yzxz(&self) -> Self {
    return Self::from([(*self)[1], (*self)[2], (*self)[0], (*self)[2]]);
  }
  pub fn yzxw(&self) -> Self {
    return Self::from([(*self)[1], (*self)[2], (*self)[0], (*self)[3]]);
  }
  pub fn yzyx(&self) -> Self {
    return Self::from([(*self)[1], (*self)[2], (*self)[1], (*self)[0]]);
  }
  pub fn yzyy(&self) -> Self {
    return Self::from([(*self)[1], (*self)[2], (*self)[1], (*self)[1]]);
  }
  pub fn yzyz(&self) -> Self {
    return Self::from([(*self)[1], (*self)[2], (*self)[1], (*self)[2]]);
  }
  pub fn yzyw(&self) -> Self {
    return Self::from([(*self)[1], (*self)[2], (*self)[1], (*self)[3]]);
  }
  pub fn yzzx(&self) -> Self {
    return Self::from([(*self)[1], (*self)[2], (*self)[2], (*self)[0]]);
  }
  pub fn yzzy(&self) -> Self {
    return Self::from([(*self)[1], (*self)[2], (*self)[2], (*self)[1]]);
  }
  pub fn yzzz(&self) -> Self {
    return Self::from([(*self)[1], (*self)[2], (*self)[2], (*self)[2]]);
  }
  pub fn yzzw(&self) -> Self {
    return Self::from([(*self)[1], (*self)[2], (*self)[2], (*self)[3]]);
  }
  pub fn yzwx(&self) -> Self {
    return Self::from([(*self)[1], (*self)[2], (*self)[3], (*self)[0]]);
  }
  pub fn yzwy(&self) -> Self {
    return Self::from([(*self)[1], (*self)[2], (*self)[3], (*self)[1]]);
  }
  pub fn yzwz(&self) -> Self {
    return Self::from([(*self)[1], (*self)[2], (*self)[3], (*self)[2]]);
  }
  pub fn yzww(&self) -> Self {
    return Self::from([(*self)[1], (*self)[2], (*self)[3], (*self)[3]]);
  }
  pub fn ywxx(&self) -> Self {
    return Self::from([(*self)[1], (*self)[3], (*self)[0], (*self)[0]]);
  }
  pub fn ywxy(&self) -> Self {
    return Self::from([(*self)[1], (*self)[3], (*self)[0], (*self)[1]]);
  }
  pub fn ywxz(&self) -> Self {
    return Self::from([(*self)[1], (*self)[3], (*self)[0], (*self)[2]]);
  }
  pub fn ywxw(&self) -> Self {
    return Self::from([(*self)[1], (*self)[3], (*self)[0], (*self)[3]]);
  }
  pub fn ywyx(&self) -> Self {
    return Self::from([(*self)[1], (*self)[3], (*self)[1], (*self)[0]]);
  }
  pub fn ywyy(&self) -> Self {
    return Self::from([(*self)[1], (*self)[3], (*self)[1], (*self)[1]]);
  }
  pub fn ywyz(&self) -> Self {
    return Self::from([(*self)[1], (*self)[3], (*self)[1], (*self)[2]]);
  }
  pub fn ywyw(&self) -> Self {
    return Self::from([(*self)[1], (*self)[3], (*self)[1], (*self)[3]]);
  }
  pub fn ywzx(&self) -> Self {
    return Self::from([(*self)[1], (*self)[3], (*self)[2], (*self)[0]]);
  }
  pub fn ywzy(&self) -> Self {
    return Self::from([(*self)[1], (*self)[3], (*self)[2], (*self)[1]]);
  }
  pub fn ywzz(&self) -> Self {
    return Self::from([(*self)[1], (*self)[3], (*self)[2], (*self)[2]]);
  }
  pub fn ywzw(&self) -> Self {
    return Self::from([(*self)[1], (*self)[3], (*self)[2], (*self)[3]]);
  }
  pub fn ywwx(&self) -> Self {
    return Self::from([(*self)[1], (*self)[3], (*self)[3], (*self)[0]]);
  }
  pub fn ywwy(&self) -> Self {
    return Self::from([(*self)[1], (*self)[3], (*self)[3], (*self)[1]]);
  }
  pub fn ywwz(&self) -> Self {
    return Self::from([(*self)[1], (*self)[3], (*self)[3], (*self)[2]]);
  }
  pub fn ywww(&self) -> Self {
    return Self::from([(*self)[1], (*self)[3], (*self)[3], (*self)[3]]);
  }
  pub fn zxxx(&self) -> Self {
    return Self::from([(*self)[2], (*self)[0], (*self)[0], (*self)[0]]);
  }
  pub fn zxxy(&self) -> Self {
    return Self::from([(*self)[2], (*self)[0], (*self)[0], (*self)[1]]);
  }
  pub fn zxxz(&self) -> Self {
    return Self::from([(*self)[2], (*self)[0], (*self)[0], (*self)[2]]);
  }
  pub fn zxxw(&self) -> Self {
    return Self::from([(*self)[2], (*self)[0], (*self)[0], (*self)[3]]);
  }
  pub fn zxyx(&self) -> Self {
    return Self::from([(*self)[2], (*self)[0], (*self)[1], (*self)[0]]);
  }
  pub fn zxyy(&self) -> Self {
    return Self::from([(*self)[2], (*self)[0], (*self)[1], (*self)[1]]);
  }
  pub fn zxyz(&self) -> Self {
    return Self::from([(*self)[2], (*self)[0], (*self)[1], (*self)[2]]);
  }
  pub fn zxyw(&self) -> Self {
    return Self::from([(*self)[2], (*self)[0], (*self)[1], (*self)[3]]);
  }
  pub fn zxzx(&self) -> Self {
    return Self::from([(*self)[2], (*self)[0], (*self)[2], (*self)[0]]);
  }
  pub fn zxzy(&self) -> Self {
    return Self::from([(*self)[2], (*self)[0], (*self)[2], (*self)[1]]);
  }
  pub fn zxzz(&self) -> Self {
    return Self::from([(*self)[2], (*self)[0], (*self)[2], (*self)[2]]);
  }
  pub fn zxzw(&self) -> Self {
    return Self::from([(*self)[2], (*self)[0], (*self)[2], (*self)[3]]);
  }
  pub fn zxwx(&self) -> Self {
    return Self::from([(*self)[2], (*self)[0], (*self)[3], (*self)[0]]);
  }
  pub fn zxwy(&self) -> Self {
    return Self::from([(*self)[2], (*self)[0], (*self)[3], (*self)[1]]);
  }
  pub fn zxwz(&self) -> Self {
    return Self::from([(*self)[2], (*self)[0], (*self)[3], (*self)[2]]);
  }
  pub fn zxww(&self) -> Self {
    return Self::from([(*self)[2], (*self)[0], (*self)[3], (*self)[3]]);
  }
  pub fn zyxx(&self) -> Self {
    return Self::from([(*self)[2], (*self)[1], (*self)[0], (*self)[0]]);
  }
  pub fn zyxy(&self) -> Self {
    return Self::from([(*self)[2], (*self)[1], (*self)[0], (*self)[1]]);
  }
  pub fn zyxz(&self) -> Self {
    return Self::from([(*self)[2], (*self)[1], (*self)[0], (*self)[2]]);
  }
  pub fn zyxw(&self) -> Self {
    return Self::from([(*self)[2], (*self)[1], (*self)[0], (*self)[3]]);
  }
  pub fn zyyx(&self) -> Self {
    return Self::from([(*self)[2], (*self)[1], (*self)[1], (*self)[0]]);
  }
  pub fn zyyy(&self) -> Self {
    return Self::from([(*self)[2], (*self)[1], (*self)[1], (*self)[1]]);
  }
  pub fn zyyz(&self) -> Self {
    return Self::from([(*self)[2], (*self)[1], (*self)[1], (*self)[2]]);
  }
  pub fn zyyw(&self) -> Self {
    return Self::from([(*self)[2], (*self)[1], (*self)[1], (*self)[3]]);
  }
  pub fn zyzx(&self) -> Self {
    return Self::from([(*self)[2], (*self)[1], (*self)[2], (*self)[0]]);
  }
  pub fn zyzy(&self) -> Self {
    return Self::from([(*self)[2], (*self)[1], (*self)[2], (*self)[1]]);
  }
  pub fn zyzz(&self) -> Self {
    return Self::from([(*self)[2], (*self)[1], (*self)[2], (*self)[2]]);
  }
  pub fn zyzw(&self) -> Self {
    return Self::from([(*self)[2], (*self)[1], (*self)[2], (*self)[3]]);
  }
  pub fn zywx(&self) -> Self {
    return Self::from([(*self)[2], (*self)[1], (*self)[3], (*self)[0]]);
  }
  pub fn zywy(&self) -> Self {
    return Self::from([(*self)[2], (*self)[1], (*self)[3], (*self)[1]]);
  }
  pub fn zywz(&self) -> Self {
    return Self::from([(*self)[2], (*self)[1], (*self)[3], (*self)[2]]);
  }
  pub fn zyww(&self) -> Self {
    return Self::from([(*self)[2], (*self)[1], (*self)[3], (*self)[3]]);
  }
  pub fn zzxx(&self) -> Self {
    return Self::from([(*self)[2], (*self)[2], (*self)[0], (*self)[0]]);
  }
  pub fn zzxy(&self) -> Self {
    return Self::from([(*self)[2], (*self)[2], (*self)[0], (*self)[1]]);
  }
  pub fn zzxz(&self) -> Self {
    return Self::from([(*self)[2], (*self)[2], (*self)[0], (*self)[2]]);
  }
  pub fn zzxw(&self) -> Self {
    return Self::from([(*self)[2], (*self)[2], (*self)[0], (*self)[3]]);
  }
  pub fn zzyx(&self) -> Self {
    return Self::from([(*self)[2], (*self)[2], (*self)[1], (*self)[0]]);
  }
  pub fn zzyy(&self) -> Self {
    return Self::from([(*self)[2], (*self)[2], (*self)[1], (*self)[1]]);
  }
  pub fn zzyz(&self) -> Self {
    return Self::from([(*self)[2], (*self)[2], (*self)[1], (*self)[2]]);
  }
  pub fn zzyw(&self) -> Self {
    return Self::from([(*self)[2], (*self)[2], (*self)[1], (*self)[3]]);
  }
  pub fn zzzx(&self) -> Self {
    return Self::from([(*self)[2], (*self)[2], (*self)[2], (*self)[0]]);
  }
  pub fn zzzy(&self) -> Self {
    return Self::from([(*self)[2], (*self)[2], (*self)[2], (*self)[1]]);
  }
  pub fn zzzz(&self) -> Self {
    return Self::from([(*self)[2], (*self)[2], (*self)[2], (*self)[2]]);
  }
  pub fn zzzw(&self) -> Self {
    return Self::from([(*self)[2], (*self)[2], (*self)[2], (*self)[3]]);
  }
  pub fn zzwx(&self) -> Self {
    return Self::from([(*self)[2], (*self)[2], (*self)[3], (*self)[0]]);
  }
  pub fn zzwy(&self) -> Self {
    return Self::from([(*self)[2], (*self)[2], (*self)[3], (*self)[1]]);
  }
  pub fn zzwz(&self) -> Self {
    return Self::from([(*self)[2], (*self)[2], (*self)[3], (*self)[2]]);
  }
  pub fn zzww(&self) -> Self {
    return Self::from([(*self)[2], (*self)[2], (*self)[3], (*self)[3]]);
  }
  pub fn zwxx(&self) -> Self {
    return Self::from([(*self)[2], (*self)[3], (*self)[0], (*self)[0]]);
  }
  pub fn zwxy(&self) -> Self {
    return Self::from([(*self)[2], (*self)[3], (*self)[0], (*self)[1]]);
  }
  pub fn zwxz(&self) -> Self {
    return Self::from([(*self)[2], (*self)[3], (*self)[0], (*self)[2]]);
  }
  pub fn zwxw(&self) -> Self {
    return Self::from([(*self)[2], (*self)[3], (*self)[0], (*self)[3]]);
  }
  pub fn zwyx(&self) -> Self {
    return Self::from([(*self)[2], (*self)[3], (*self)[1], (*self)[0]]);
  }
  pub fn zwyy(&self) -> Self {
    return Self::from([(*self)[2], (*self)[3], (*self)[1], (*self)[1]]);
  }
  pub fn zwyz(&self) -> Self {
    return Self::from([(*self)[2], (*self)[3], (*self)[1], (*self)[2]]);
  }
  pub fn zwyw(&self) -> Self {
    return Self::from([(*self)[2], (*self)[3], (*self)[1], (*self)[3]]);
  }
  pub fn zwzx(&self) -> Self {
    return Self::from([(*self)[2], (*self)[3], (*self)[2], (*self)[0]]);
  }
  pub fn zwzy(&self) -> Self {
    return Self::from([(*self)[2], (*self)[3], (*self)[2], (*self)[1]]);
  }
  pub fn zwzz(&self) -> Self {
    return Self::from([(*self)[2], (*self)[3], (*self)[2], (*self)[2]]);
  }
  pub fn zwzw(&self) -> Self {
    return Self::from([(*self)[2], (*self)[3], (*self)[2], (*self)[3]]);
  }
  pub fn zwwx(&self) -> Self {
    return Self::from([(*self)[2], (*self)[3], (*self)[3], (*self)[0]]);
  }
  pub fn zwwy(&self) -> Self {
    return Self::from([(*self)[2], (*self)[3], (*self)[3], (*self)[1]]);
  }
  pub fn zwwz(&self) -> Self {
    return Self::from([(*self)[2], (*self)[3], (*self)[3], (*self)[2]]);
  }
  pub fn zwww(&self) -> Self {
    return Self::from([(*self)[2], (*self)[3], (*self)[3], (*self)[3]]);
  }
  pub fn wxxx(&self) -> Self {
    return Self::from([(*self)[3], (*self)[0], (*self)[0], (*self)[0]]);
  }
  pub fn wxxy(&self) -> Self {
    return Self::from([(*self)[3], (*self)[0], (*self)[0], (*self)[1]]);
  }
  pub fn wxxz(&self) -> Self {
    return Self::from([(*self)[3], (*self)[0], (*self)[0], (*self)[2]]);
  }
  pub fn wxxw(&self) -> Self {
    return Self::from([(*self)[3], (*self)[0], (*self)[0], (*self)[3]]);
  }
  pub fn wxyx(&self) -> Self {
    return Self::from([(*self)[3], (*self)[0], (*self)[1], (*self)[0]]);
  }
  pub fn wxyy(&self) -> Self {
    return Self::from([(*self)[3], (*self)[0], (*self)[1], (*self)[1]]);
  }
  pub fn wxyz(&self) -> Self {
    return Self::from([(*self)[3], (*self)[0], (*self)[1], (*self)[2]]);
  }
  pub fn wxyw(&self) -> Self {
    return Self::from([(*self)[3], (*self)[0], (*self)[1], (*self)[3]]);
  }
  pub fn wxzx(&self) -> Self {
    return Self::from([(*self)[3], (*self)[0], (*self)[2], (*self)[0]]);
  }
  pub fn wxzy(&self) -> Self {
    return Self::from([(*self)[3], (*self)[0], (*self)[2], (*self)[1]]);
  }
  pub fn wxzz(&self) -> Self {
    return Self::from([(*self)[3], (*self)[0], (*self)[2], (*self)[2]]);
  }
  pub fn wxzw(&self) -> Self {
    return Self::from([(*self)[3], (*self)[0], (*self)[2], (*self)[3]]);
  }
  pub fn wxwx(&self) -> Self {
    return Self::from([(*self)[3], (*self)[0], (*self)[3], (*self)[0]]);
  }
  pub fn wxwy(&self) -> Self {
    return Self::from([(*self)[3], (*self)[0], (*self)[3], (*self)[1]]);
  }
  pub fn wxwz(&self) -> Self {
    return Self::from([(*self)[3], (*self)[0], (*self)[3], (*self)[2]]);
  }
  pub fn wxww(&self) -> Self {
    return Self::from([(*self)[3], (*self)[0], (*self)[3], (*self)[3]]);
  }
  pub fn wyxx(&self) -> Self {
    return Self::from([(*self)[3], (*self)[1], (*self)[0], (*self)[0]]);
  }
  pub fn wyxy(&self) -> Self {
    return Self::from([(*self)[3], (*self)[1], (*self)[0], (*self)[1]]);
  }
  pub fn wyxz(&self) -> Self {
    return Self::from([(*self)[3], (*self)[1], (*self)[0], (*self)[2]]);
  }
  pub fn wyxw(&self) -> Self {
    return Self::from([(*self)[3], (*self)[1], (*self)[0], (*self)[3]]);
  }
  pub fn wyyx(&self) -> Self {
    return Self::from([(*self)[3], (*self)[1], (*self)[1], (*self)[0]]);
  }
  pub fn wyyy(&self) -> Self {
    return Self::from([(*self)[3], (*self)[1], (*self)[1], (*self)[1]]);
  }
  pub fn wyyz(&self) -> Self {
    return Self::from([(*self)[3], (*self)[1], (*self)[1], (*self)[2]]);
  }
  pub fn wyyw(&self) -> Self {
    return Self::from([(*self)[3], (*self)[1], (*self)[1], (*self)[3]]);
  }
  pub fn wyzx(&self) -> Self {
    return Self::from([(*self)[3], (*self)[1], (*self)[2], (*self)[0]]);
  }
  pub fn wyzy(&self) -> Self {
    return Self::from([(*self)[3], (*self)[1], (*self)[2], (*self)[1]]);
  }
  pub fn wyzz(&self) -> Self {
    return Self::from([(*self)[3], (*self)[1], (*self)[2], (*self)[2]]);
  }
  pub fn wyzw(&self) -> Self {
    return Self::from([(*self)[3], (*self)[1], (*self)[2], (*self)[3]]);
  }
  pub fn wywx(&self) -> Self {
    return Self::from([(*self)[3], (*self)[1], (*self)[3], (*self)[0]]);
  }
  pub fn wywy(&self) -> Self {
    return Self::from([(*self)[3], (*self)[1], (*self)[3], (*self)[1]]);
  }
  pub fn wywz(&self) -> Self {
    return Self::from([(*self)[3], (*self)[1], (*self)[3], (*self)[2]]);
  }
  pub fn wyww(&self) -> Self {
    return Self::from([(*self)[3], (*self)[1], (*self)[3], (*self)[3]]);
  }
  pub fn wzxx(&self) -> Self {
    return Self::from([(*self)[3], (*self)[2], (*self)[0], (*self)[0]]);
  }
  pub fn wzxy(&self) -> Self {
    return Self::from([(*self)[3], (*self)[2], (*self)[0], (*self)[1]]);
  }
  pub fn wzxz(&self) -> Self {
    return Self::from([(*self)[3], (*self)[2], (*self)[0], (*self)[2]]);
  }
  pub fn wzxw(&self) -> Self {
    return Self::from([(*self)[3], (*self)[2], (*self)[0], (*self)[3]]);
  }
  pub fn wzyx(&self) -> Self {
    return Self::from([(*self)[3], (*self)[2], (*self)[1], (*self)[0]]);
  }
  pub fn wzyy(&self) -> Self {
    return Self::from([(*self)[3], (*self)[2], (*self)[1], (*self)[1]]);
  }
  pub fn wzyz(&self) -> Self {
    return Self::from([(*self)[3], (*self)[2], (*self)[1], (*self)[2]]);
  }
  pub fn wzyw(&self) -> Self {
    return Self::from([(*self)[3], (*self)[2], (*self)[1], (*self)[3]]);
  }
  pub fn wzzx(&self) -> Self {
    return Self::from([(*self)[3], (*self)[2], (*self)[2], (*self)[0]]);
  }
  pub fn wzzy(&self) -> Self {
    return Self::from([(*self)[3], (*self)[2], (*self)[2], (*self)[1]]);
  }
  pub fn wzzz(&self) -> Self {
    return Self::from([(*self)[3], (*self)[2], (*self)[2], (*self)[2]]);
  }
  pub fn wzzw(&self) -> Self {
    return Self::from([(*self)[3], (*self)[2], (*self)[2], (*self)[3]]);
  }
  pub fn wzwx(&self) -> Self {
    return Self::from([(*self)[3], (*self)[2], (*self)[3], (*self)[0]]);
  }
  pub fn wzwy(&self) -> Self {
    return Self::from([(*self)[3], (*self)[2], (*self)[3], (*self)[1]]);
  }
  pub fn wzwz(&self) -> Self {
    return Self::from([(*self)[3], (*self)[2], (*self)[3], (*self)[2]]);
  }
  pub fn wzww(&self) -> Self {
    return Self::from([(*self)[3], (*self)[2], (*self)[3], (*self)[3]]);
  }
  pub fn wwxx(&self) -> Self {
    return Self::from([(*self)[3], (*self)[3], (*self)[0], (*self)[0]]);
  }
  pub fn wwxy(&self) -> Self {
    return Self::from([(*self)[3], (*self)[3], (*self)[0], (*self)[1]]);
  }
  pub fn wwxz(&self) -> Self {
    return Self::from([(*self)[3], (*self)[3], (*self)[0], (*self)[2]]);
  }
  pub fn wwxw(&self) -> Self {
    return Self::from([(*self)[3], (*self)[3], (*self)[0], (*self)[3]]);
  }
  pub fn wwyx(&self) -> Self {
    return Self::from([(*self)[3], (*self)[3], (*self)[1], (*self)[0]]);
  }
  pub fn wwyy(&self) -> Self {
    return Self::from([(*self)[3], (*self)[3], (*self)[1], (*self)[1]]);
  }
  pub fn wwyz(&self) -> Self {
    return Self::from([(*self)[3], (*self)[3], (*self)[1], (*self)[2]]);
  }
  pub fn wwyw(&self) -> Self {
    return Self::from([(*self)[3], (*self)[3], (*self)[1], (*self)[3]]);
  }
  pub fn wwzx(&self) -> Self {
    return Self::from([(*self)[3], (*self)[3], (*self)[2], (*self)[0]]);
  }
  pub fn wwzy(&self) -> Self {
    return Self::from([(*self)[3], (*self)[3], (*self)[2], (*self)[1]]);
  }
  pub fn wwzz(&self) -> Self {
    return Self::from([(*self)[3], (*self)[3], (*self)[2], (*self)[2]]);
  }
  pub fn wwzw(&self) -> Self {
    return Self::from([(*self)[3], (*self)[3], (*self)[2], (*self)[3]]);
  }
  pub fn wwwx(&self) -> Self {
    return Self::from([(*self)[3], (*self)[3], (*self)[3], (*self)[0]]);
  }
  pub fn wwwy(&self) -> Self {
    return Self::from([(*self)[3], (*self)[3], (*self)[3], (*self)[1]]);
  }
  pub fn wwwz(&self) -> Self {
    return Self::from([(*self)[3], (*self)[3], (*self)[3], (*self)[2]]);
  }
  pub fn wwww(&self) -> Self {
    return Self::from([(*self)[3], (*self)[3], (*self)[3], (*self)[3]]);
  }
}
