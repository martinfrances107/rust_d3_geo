use delaunator::Point;

use crate::Transform;

#[derive(Debug)]
pub struct RotationPhiGamma
{
  cos_delta_phi: f64,
  sin_delta_phi: f64,
  cos_delta_gamma: f64,
  sin_delta_gamma: f64,
}

impl RotationPhiGamma
{
  pub fn new(delta_phi: f64, delta_gamma: f64) -> Box<dyn Transform>
  {
    return Box::new(Self {
      cos_delta_phi: delta_phi.cos(),
      sin_delta_phi: delta_phi.sin(),
      cos_delta_gamma: delta_gamma.cos(),
      sin_delta_gamma: delta_gamma.sin(),
    });
  }
}

impl Transform for RotationPhiGamma
{
  #[allow(clippy::many_single_char_names)]
  fn transform(&self, p: &Point) -> Point {
    let lambda = p.x;
    let phi = p.y;

    let cos_phi = phi.cos();
    let x = lambda.cos() * cos_phi;
    let y = lambda.sin() * cos_phi;
    let z = phi.sin();
    let k = z * self.cos_delta_phi + x * self.sin_delta_phi;

    return Point{
      x:(y * self.cos_delta_gamma - k * self.sin_delta_gamma)
        .atan2(x * self.cos_delta_phi - z * self.sin_delta_phi),
      y:(k * self.cos_delta_gamma + y * self.sin_delta_gamma).asin(),
    };
  }

  #[allow(clippy::many_single_char_names)]
  fn invert(&self, p: &Point) -> Point {
    let lambda = p.x;
    let phi = p.y;

    let cos_phi = phi.cos();
    let x = lambda.cos() * cos_phi;
    let y = lambda.sin() * cos_phi;
    let z = phi.sin();
    let k = z * self.cos_delta_gamma - y * self.sin_delta_gamma;

    return Point{
      x:(y * self.cos_delta_gamma + z * self.sin_delta_gamma)
        .atan2(x * self.cos_delta_phi + k * self.sin_delta_phi),
      y:(k * self.cos_delta_phi - x * self.sin_delta_phi).asin(),
    };
  }
}
