use num_traits::cast::AsPrimitive;
use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use super::projection_mutator::ProjectionMutator;
use crate::Transform;

pub fn projection_equal<F>(
  projection: &ProjectionMutator<F>,
  expected_location: &[F; 2],
  expected_point: &[F; 2],
  delta_p: Option<F>,
) -> bool
where
  F: Float + FloatConst + FromPrimitive + AsPrimitive<f64>,
{
  let delta = match delta_p {
    Some(d) => d,
    None => F::from(1e-6).unwrap(),
  };
  println!("project_equal");
  println!(
    "expected [{:?}, {:?}], [{:?}, {:?}]",
    expected_location[0].as_(),
    expected_location[1].as_(),
    expected_point[0].as_(),
    expected_point[1].as_(),
  );
  let actual_location = projection.invert(expected_point);
  let actual_point = projection.transform(expected_location);
  println!(
    "actual [{:?}, {:?}], [{:?}, {:?}]",
    actual_location[0].as_(),
    actual_location[1].as_(),
    actual_point[0].as_(),
    actual_point[1].as_(),
  );
  return planar_equal(actual_point, expected_point, delta)
    && spherical_equal(actual_location, expected_location, delta);
}

fn planar_equal<F>(actual: [F; 2], expected: &[F; 2], delta: F) -> bool
where
  F: Float + FromPrimitive + AsPrimitive<f64>,
{
  let e0 = in_delta(actual[0], expected[0], delta);
  let e1 = in_delta(actual[1], expected[1], delta);
  return e0 && e1;
}

fn spherical_equal<F>(actual: [F; 2], expected: &[F; 2], delta: F) -> bool
where
  F: Float + FromPrimitive + AsPrimitive<f64>,
{
  let e0 = logitude_equal(actual[0], expected[0], delta);
  let e1 = in_delta(actual[1], expected[1], delta);
  return e0 & e1;
}

fn logitude_equal<F>(actual: F, expected: F, delta: F) -> bool
where
  F: Float + FromPrimitive + AsPrimitive<f64>,
{
  println!("in le");
  let f360 = F::from(360u16).unwrap();
  let actual = (actual - expected).abs() % f360;
  return actual <= delta || actual >= f360 - delta;
}

fn in_delta<F>(actual: F, expected: F, delta: F) -> bool
where
  F: Float + FromPrimitive + AsPrimitive<f64>,
{
  return (actual - expected).abs() <= delta;
}
