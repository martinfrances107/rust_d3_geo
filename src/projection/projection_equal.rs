use num_traits::cast::AsPrimitive;
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
  F: Float + FloatConst + FromPrimitive + AsPrimitive<f64>,
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
  F: Float + FromPrimitive + AsPrimitive<f64>,
{
  let e0 = in_delta(actual[0], expected[0], delta);
  let e1 = in_delta(actual[1], expected[1], delta);
  println!(
    " actual [{:?},{:?}] expected , [{:?},{:?}]",
    actual[0].as_(),
    actual[1].as_(),
    expected[0].as_(),
    expected[1].as_()
  );
  println!("planar equal {:?} {:?}", e0, e1);
  return e0 && e1;
}

fn spherical_equal<F>(actual: [F; 2], expected: &[F; 2], delta: F) -> bool
where
  F: Float + FromPrimitive + AsPrimitive<f64>,
{
  let e0 = logitude_equal(actual[0], expected[0], delta);
  let e1 = in_delta(actual[1], expected[1], delta);
  println!(
    " actual [{:?},{:?}] expected , [{:?},{:?}]",
    actual[0].as_(),
    actual[1].as_(),
    expected[0].as_(),
    expected[1].as_()
  );
  println!("longitude equal {:?} {:?}", e0, e1);
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
