use num_traits::cast::AsPrimitive;
use num_traits::cast::FromPrimitive;
use num_traits::Float;

pub fn in_delta<F>(actual: F, expected: F, delta: F) -> bool
where
  F: Float + FromPrimitive + AsPrimitive<f64>,
{
  return (actual - expected).abs() <= delta;
}
