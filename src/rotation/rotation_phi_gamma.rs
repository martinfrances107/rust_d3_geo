use num_traits::Float;
use num_traits::FloatConst;

use crate::Transform;

pub struct RotationPhiGamma<F>
where
  F: Float + FloatConst,
{
  cos_delta_phi: F,
  sin_delta_phi: F,
  cos_delta_gamma: F,
  sin_delta_gamma: F,
}

impl<F> RotationPhiGamma<F>
where
  F: Float + FloatConst,
{
  pub fn new(delta_phi: &F, delta_gamma: &F) -> Self {
    return Self {
      cos_delta_phi: delta_phi.cos(),
      sin_delta_phi: delta_phi.sin(),
      cos_delta_gamma: delta_gamma.cos(),
      sin_delta_gamma: delta_gamma.sin(),
    };
  }
}

impl<F> Transform<F> for RotationPhiGamma<F>
where
  F: Float + FloatConst,
{
  fn transform(&self, p: &[F; 2]) -> [F; 2] {
    let lambda = p[0];
    let phi = p[1];

    let cos_phi = phi.cos();
    let x = lambda.cos() * cos_phi;
    let y = lambda.sin() * cos_phi;
    let z = phi.sin();
    let k = z * self.cos_delta_phi + x * self.sin_delta_phi;

    return [
      (y * self.cos_delta_gamma - k * self.sin_delta_gamma)
        .atan2(x * self.cos_delta_phi - z * self.sin_delta_phi),
      (k * self.cos_delta_gamma + y * self.sin_delta_gamma).asin(),
    ];
  }

  fn invert(&self, p: &[F; 2]) -> [F; 2] {
    let lambda = p[0];
    let phi = p[1];

    let cos_phi = phi.cos();
    let x = lambda.cos() * cos_phi;
    let y = lambda.sin() * cos_phi;
    let z = phi.sin();
    let k = z * self.cos_delta_gamma - y * self.sin_delta_gamma;

    return [
      (y * self.cos_delta_gamma + z * self.sin_delta_gamma)
        .atan2(x * self.cos_delta_phi + k * self.sin_delta_phi),
      (k * self.cos_delta_phi - x * self.sin_delta_phi).asin(),
    ];
  }
}
