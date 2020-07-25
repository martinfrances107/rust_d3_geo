use num_traits::Float;

use crate::Transform;

pub struct ScaleTranslate<T> {
  k: T,
  dx: T,
  dy: T,
  sx: T,
  sy: T,
}

impl<T> ScaleTranslate<T> {
  pub fn new(k: T, dx: T, dy: T, sx: T, sy: T) -> Box<Self> {
    return Box::new(ScaleTranslate { k, dx, dy, sx, sy });
  }
}

impl<T> Transform<T> for ScaleTranslate<T>
where
  T: Float,
{
  fn transform(&self, p: &[T; 2]) -> [T; 2] {
    let x = p[0] * self.sx;
    let y = p[1] * self.sy;
    return [self.dx + self.k * x, self.dy + self.k * y];
  }

  fn invert(&self, p: &[T; 2]) -> [T; 2] {
    return [
      (p[0] - self.dx) / self.k * self.sx,
      (self.dy - p[1]) / self.k * self.sy,
    ];
  }
}
