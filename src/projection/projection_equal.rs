use delaunator::Point;

use crate::in_delta::in_delta;
use crate::Transform;

use super::projection_mutator::ProjectionMutator;

pub fn projection_equal(
  projection: &ProjectionMutator,
  expected_location: &Point,
  expected_point: &Point,
  delta_p: Option<f64>,
) -> bool
{
  let delta = match delta_p {
    Some(d) => d,
    None => 1e-6f64,
  };
  println!("project_equal");
  println!(
    "expected [{:?}, {:?}], [{:?}, {:?}]",
    expected_location.x,
    expected_location.y,
    expected_point.x,
    expected_point.y,
  );
  let actual_location = projection.invert(expected_point);
  let actual_point = projection.transform(expected_location);
  println!(
    "actual [{:?}, {:?}], [{:?}, {:?}]",
    actual_location.x,
    actual_location.y,
    actual_point.x,
    actual_point.y,
  );
  return planar_equal(actual_point, expected_point, delta)
    && spherical_equal(actual_location, expected_location, delta);
}

fn planar_equal(actual: Point, expected: &Point, delta: f64) -> bool
{
  let e0 = in_delta(actual.x, expected.x, delta);
  let e1 = in_delta(actual.y, expected.y, delta);
  return e0 && e1;
}

fn spherical_equal(actual: Point, expected: &Point, delta: f64) -> bool
{
  let e0 = logitude_equal(actual.x, expected.x, delta);
  let e1 = in_delta(actual.y, expected.y, delta);
  return e0 & e1;
}

fn logitude_equal(actual: f64, expected: f64, delta: f64) -> bool
{
  let actual = (actual - expected).abs() % 360f64;
  return actual <= delta || actual >= 360f64 - delta;
}
