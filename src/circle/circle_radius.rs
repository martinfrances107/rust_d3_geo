use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use crate::cartesian::cartesian;
use crate::cartesian::cartesian_normalize_in_place;
use crate::cartesian::spherical;

/// Returns the signed angle of a cartesian point relative to [cosRadius, 0, 0].
pub fn circle_radius<F>(cos_radius: F, point_p: [F; 2]) -> F
where
  F: Float + FloatConst + FromPrimitive,
{
  let mut point = cartesian(&point_p);
  point[0] = point[0] - cos_radius;
  cartesian_normalize_in_place(&mut point);
  let radius = (-point[1]).acos();
  let radius_signed = match -point[2] < F::zero() {
    true => -radius,
    false => radius,
  };
  return (radius_signed + F::TAU() - F::epsilon()) % F::TAU();
}
