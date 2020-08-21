use num_traits::Float;

use crate::Transform;

use super::scale_translate::ScaleTranslate;

#[derive(Debug)]
pub struct ScaleTranslateRotate<F>
where
  F: Float,
{
  a: F,
  b: F,
  ai: F,
  bi: F,
  ci: F,
  fi: F,
  dx: F,
  dy: F,
  sx: F,
  sy: F,
}

impl<F> ScaleTranslateRotate<F>
where
  F: Float + 'static,
{
  pub fn new(k: F, dx: F, dy: F, sx: F, sy: F, alpha: F) -> Box<dyn Transform<F>> {
    if alpha.is_zero() {
      return ScaleTranslate::new(k, dx, dy, sx, sy);
    } else {
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
}

impl<F> Transform<F> for ScaleTranslateRotate<F>
where
  F: Float,
{
  fn transform(&self, p: &[F; 2]) -> [F; 2] {
    let x = p[0] * self.sx;
    let y = p[1] * self.sy;
    return [
      self.a * x - self.b * y + self.dx,
      self.dy - self.b * x - self.a * y,
    ];
  }

  fn invert(&self, p: &[F; 2]) -> [F; 2] {
    return [
      self.sx * (self.ai * p[0] - self.bi * p[1] + self.ci),
      self.sy * (self.fi - self.bi * p[0] - self.ai * p[1]),
    ];
  }
}
