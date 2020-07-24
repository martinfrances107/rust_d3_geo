use num_traits::Float;
use num_traits::FloatConst;

use crate::Transform;

pub struct RotationPhiGamma<T>
where
  T: Float + FloatConst,
{
  cos_delta_phi: T,
  sin_delta_phi: T,
  cos_delta_gamma: T,
  sin_delta_gamma: T,
}

impl<T> RotationPhiGamma<T>
where
  T: Float + FloatConst,
{
  pub fn new(delta_phi: T, delta_gamma: T) -> Self {
    return Self {
      cos_delta_phi: delta_phi.cos(),
      sin_delta_phi: delta_phi.sin(),
      cos_delta_gamma: delta_gamma.cos(),
      sin_delta_gamma: delta_gamma.sin(),
    };
  }
}

impl<T> Transform<T> for RotationPhiGamma<T>
where
  T: Float + FloatConst,
{
  fn transform(&self, p: &[T; 2]) -> [T; 2] {
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

  fn invert(&self, p: &[T; 2]) -> [T; 2] {
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
