use num_traits::Float;
use num_traits::FloatConst;
use std::f64::consts::PI;

use crate::Transform;
use crate::TransformIdentity;

pub struct RotationIdentity {}

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
