use num_traits::Float;

use crate::Transform;

pub struct ScaleTranslateRotate<T>
where
  T: Float,
{
  a: T,
  b: T,
  ai: T,
  bi: T,
  ci: T,
  fi: T,
  dx: T,
  dy: T,
  sx: T,
  sy: T,
}

impl<T> ScaleTranslateRotate<T>
where
  T: Float,
{
  pub fn new(k: T, dx: T, dy: T, sx: T, sy: T, alpha: T) -> Box<Self> {
    let cos_alpha = alpha.cos();
    let sin_alpha = alpha.sin();
    return Box::new(ScaleTranslateRotate {
      a: cos_alpha * k,
      b: sin_alpha * k,
      ai: cos_alpha / k,
      bi: sin_alpha / k,
      ci: (sin_alpha * dy - cos_alpha * dx) / k,
      fi: (sin_alpha * dx + cos_alpha * dy) / k,
      dx,
      dy,
      sx: sx,
      sy: sy,
    });
  }
}

impl<T> Transform<T> for ScaleTranslateRotate<T>
where
  T: Float,
{
  fn transform(&self, p: &[T; 2]) -> [T; 2] {
    let x = p[0] * self.sx;
    let y = p[1] * self.sy;
    return [
      self.a * x - self.b * y + self.dx,
      self.dy - self.b * x - self.a * y,
    ];
  }

  fn invert(&self, p: &[T; 2]) -> [T; 2] {
    return [
      self.sx * (self.ai * p[0] - self.bi * p[1] + self.ci),
      self.sy * (self.fi - self.bi * p[0] - self.ai * p[1]),
    ];
  }
}
