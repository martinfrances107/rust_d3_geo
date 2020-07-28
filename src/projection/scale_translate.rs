use num_traits::Float;

use crate::Transform;

pub struct ScaleTranslate<F>
where F: Float {
  k: F,
  dx: F,
  dy: F,
  sx: F,
  sy: F,
}

impl <'a, F: 'static >ScaleTranslate<F>
where F: Float {
  pub fn new(k: F, dx: F, dy: F, sx: F, sy: F) -> Box<dyn Transform<F>> {
    return Box::new(ScaleTranslate::<F>{ k, dx, dy, sx, sy });
  }
}

impl<F> Transform<F> for ScaleTranslate<F>
where
  F: Float,
{
  fn transform(&self, p: &[F; 2]) -> [F; 2] {
    let x = p[0] * self.sx;
    let y = p[1] * self.sy;
    return [self.dx + self.k * x, self.dy + self.k * y];
  }

  fn invert(&self, p: &[F; 2]) -> [F; 2] {
    return [
      (p[0] - self.dx) / self.k * self.sx,
      (self.dy - p[1]) / self.k * self.sy,
    ];
  }
}
