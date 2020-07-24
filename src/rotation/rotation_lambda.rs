use num_traits::Float;
use num_traits::FloatConst;

use crate::Transform;

pub struct RotationLambda<T>
where
  T: Float + FloatConst,
{
  pub delta_lambda: T,
}

// TODO why can't I #[inline] this.
fn forward_rotation_lambda<T>(delta_lambda: T, p: &[T; 2]) -> [T; 2]
where
  T: Float + FloatConst,
{
  let lambda = p[0] + delta_lambda;
  let phi = p[1];
  return match (lambda > T::PI(), lambda < -T::PI()) {
    (false, false) => [lambda, phi],       // -PI <= lambda <= PI
    (true, _) => [lambda - T::TAU(), phi], // lambda >  PI
    (_, true) => [lambda + T::TAU(), phi], // lambda < -PI
  };
}

impl<T> RotationLambda<T>
where
  T: Float + FloatConst,
{
  pub fn new(delta_lambda: T) -> Self {
    return Self { delta_lambda };
  }
}

impl<T> Transform<T> for RotationLambda<T>
where
  T: Float + FloatConst,
{
  fn transform(&self, coordinates: &[T; 2]) -> [T; 2] {
    return forward_rotation_lambda(self.delta_lambda, coordinates);
  }
  fn invert(&self, coordinates: &[T; 2]) -> [T; 2] {
    return forward_rotation_lambda(-self.delta_lambda, coordinates);
  }
}
