use num_traits::Float;
use num_traits::FloatConst;

use crate::Transform;

#[derive(Clone, Debug)]
pub struct RotationIdentity {}

// By design a stateless function.
// TODO maybe add attributes to suggest inlining this where possible.
fn normalise<F>(p: &[F; 2]) -> [F; 2]
where
  F: Float + FloatConst,
{
  let lambda = p[0];
  let phi = p[1];

  return match lambda.abs() > F::PI() {
    true => [lambda + (-lambda / F::TAU()).round() * F::TAU(), phi],
    false => [lambda, phi],
  };
}

impl RotationIdentity {
  pub fn new<F>() -> Box<dyn Transform<F>>
  where
    F: Float + FloatConst + 'static,
  {
    return Box::new(RotationIdentity {});
  }
}

impl<F> Transform<F> for RotationIdentity
where
  F: Float + FloatConst,
{
  fn transform(&self, p: &[F; 2]) -> [F; 2] {
    return normalise(p);
  }

  fn invert(&self, p: &[F; 2]) -> [F; 2] {
    return normalise(p);
  }
}
