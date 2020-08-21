use num_traits::Float;

use crate::Transform;

#[derive(Debug)]
pub struct ScaleTranslate<F> {
  k: F,
  dx: F,
  dy: F,
  sx: F,
  sy: F,
}

impl<F> ScaleTranslate<F> {
  pub fn new(k: F, dx: F, dy: F, sx: F, sy: F) -> Box<Self> {
    return Box::new(ScaleTranslate { k, dx, dy, sx, sy });
  }
}

impl<F> Transform<F> for ScaleTranslate<F>
where
  F: Float,
{
  fn transform(&self, p: &[F; 2]) -> [F; 2] {
    let x = p[0] * self.sx;
    let y = p[1] * self.sy;
    // TODO the minus sign in the y-output component I think is a inconsistency/bug in the javascript.
    // it should be :-
    // self.dy + self.k * y
    // but that would mean a departure from the copy and would have to be adjusted elsewhere.
    return [self.dx + self.k * x, self.dy - self.k * y];
  }

  fn invert(&self, p: &[F; 2]) -> [F; 2] {
    return [
      (p[0] - self.dx) / self.k * self.sx,
      (self.dy - p[1]) / self.k * self.sy,
    ];
  }
}
