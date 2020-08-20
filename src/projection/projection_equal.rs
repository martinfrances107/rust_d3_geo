use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use super::projection_mutator::ProjectionMutator;
use crate::Transform;

pub fn projection_equal<F>(
  projection: &ProjectionMutator<F>,
  location: &[F; 2],
  point: &[F; 2],
  delta_p: Option<F>,
) -> bool
where
  F: Float + FloatConst + FromPrimitive,
{
  let delta = match delta_p {
    Some(d) => d,
    None => F::from(1e-6).unwrap(),
  };
  return planar_equal(projection.transform(location), point, delta)
    && spherical_equal(projection.invert(point), location, delta);
}

fn planar_equal<F>(actual: [F; 2], expected: &[F; 2], delta: F) -> bool
where
  F: Float,
{
  return in_delta(actual[0], expected[0], delta) & in_delta(actual[1], expected[1], delta);
}

fn spherical_equal<F>(actual: [F; 2], expected: &[F; 2], delta: F) -> bool
where
  F: Float,
{
  return logitude_equal(actual[0], expected[0], delta) & in_delta(actual[1], expected[1], delta);
}

fn logitude_equal<F>(actual: F, expected: F, delta: F) -> bool
where
  F: Float,
{
  println!("in le");
  let f360 = F::from(360u16).unwrap();
  let actual = (actual - expected).abs() % f360;
  return actual <= delta || actual >= f360 - delta;
}

fn in_delta<F>(actual: F, expected: F, delta: F) -> bool
where
  F: Float,
{
  return (actual - expected).abs() <= delta;
}
