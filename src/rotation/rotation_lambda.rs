use num_traits::Float;
use num_traits::FloatConst;

use crate::Transform;

#[derive(Debug)]
pub struct RotationLambda<F>
where
  F: Float + FloatConst,
{
  pub delta_lambda: F,
  pub neg_delta_lambda: F,
}

// TODO why can't I #[inline] this.
fn forward_rotation_lambda<'a, F>(delta_lambda: F, p: &'a [F; 2]) -> [F; 2]
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

impl<'a, F> RotationLambda<F>
where
  F: Float + FloatConst,
{
  pub fn new(delta_lambda_p: &'a F) -> Self {
    let delta_lambda = *delta_lambda_p;
    let neg_delta_lambda = -delta_lambda;
    return Self { delta_lambda, neg_delta_lambda };
  }
}

impl<'a, F> Transform<F> for RotationLambda<F>
where
  F: Float + FloatConst,
{
  fn transform(&self, coordinates: &[F; 2]) -> [F; 2] {
    return forward_rotation_lambda(self.delta_lambda, coordinates);
  }
  fn invert(&self, coordinates: &[F; 2]) -> [F; 2] {

    // TODO must come back and optimise this.
    return forward_rotation_lambda(self.neg_delta_lambda, coordinates);

  }
}

