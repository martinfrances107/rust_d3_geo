use num_traits::Float;
use num_traits::FloatConst;

use crate::Transform;

pub struct RotationLambda<F>
where
  F: Float + FloatConst,
{
  pub delta_lambda: F,
}

// TODO why can't I #[inline] this.
fn forward_rotation_lambda<F>(delta_lambda: F, p: &[F; 2]) -> [F; 2]
where
  F: Float + FloatConst,
{
  let lambda = p[0] + delta_lambda;
  let phi = p[1];
  return match (lambda > F::PI(), lambda < -F::PI()) {
    (false, false) => [lambda, phi],       // -PI <= lambda <= PI
    (true, _) => [lambda - F::TAU(), phi], // lambda >  PI
    (_, true) => [lambda + F::TAU(), phi], // lambda < -PI
  };
}

impl<F> RotationLambda<F>
where
  F: Float + FloatConst,
{
  pub fn new(delta_lambda: F) -> Self {
    return Self { delta_lambda };
  }
}

impl<F> Transform<F> for RotationLambda<F>
where
  F: Float + FloatConst,
{
  fn transform(&self, coordinates: &[F; 2]) -> [F; 2] {
    return forward_rotation_lambda(self.delta_lambda, coordinates);
  }
  fn invert(&self, coordinates: &[F; 2]) -> [F; 2] {
    return forward_rotation_lambda(-self.delta_lambda, coordinates);
  }
}
