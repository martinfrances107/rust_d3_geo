use delaunator::Point;

use crate::Transform;

use super::scale_translate::ScaleTranslate;

#[derive(Debug)]
pub struct ScaleTranslateRotate
{
  a: f64,
  b: f64,
  ai: f64,
  bi: f64,
  ci: f64,
  fi: f64,
  dx: f64,
  dy: f64,
  sx: f64,
  sy: f64,
}

impl ScaleTranslateRotate
{
  pub fn new(k: f64, dx: f64, dy: f64, sx: f64, sy: f64, alpha: f64) -> Box<dyn Transform> {
    if alpha == 0f64 {
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
        sx,
        sy,
      });
    }
  }
}

impl Transform for ScaleTranslateRotate
{
  fn transform(&self, p: &Point) -> Point {
    let x = p.x * self.sx;
    let y = p.y * self.sy;
    return Point{
      x:self.a * x - self.b * y + self.dx,
      y:self.dy - self.b * x - self.a * y,
    };
  }

  fn invert(&self, p: &Point) -> Point {
    return Point{
      x:self.sx * (self.ai * p.x - self.bi * p.y + self.ci),
      y:self.sy * (self.fi - self.bi * p.x - self.ai * p.y),
    };
  }
}
