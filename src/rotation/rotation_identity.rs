use num_traits::Float;
use num_traits::FloatConst;

use crate::Transform;

pub struct RotationIdentity {}

// By design a stateless function.
// TODO maybe add attributes to suggest inlining this where possible.
fn normalise<T>(p: &[T; 2]) -> [T; 2]
where
  T: Float + FloatConst,
{
  let lambda = p[0];
  let phi = p[1];

  return match lambda.abs() > T::PI() {
    true => [lambda + (-lambda / T::TAU()).round() * T::TAU(), phi],
    false => [lambda, phi],
  };
}

impl<T> Transform<T> for RotationIdentity
where
  T: Float + FloatConst,
{
  fn transform(&self, p: &[T; 2]) -> [T; 2] {
    return normalise(p);
  }

  fn invert(&self, p: &[T; 2]) -> [T; 2] {
    return normalise(p);
  }
}
