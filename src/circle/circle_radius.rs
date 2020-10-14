use std::f64;

use crate::cartesian::cartesian;
use crate::cartesian::cartesian_normalize_in_place;
use crate::cartesian::spherical;
use crate::math::TAU;

use delaunator::Point;

/// Returns the signed angle of a cartesian point relative to [cosRadius, 0, 0].
pub fn circle_radius(cos_radius: f64, point_p: Point) -> f64
{
  let mut point = cartesian(&point_p);
  point[0] = point[0] - cos_radius;
  cartesian_normalize_in_place(&mut point);
  let radius = (-point[1]).acos();
  let radius_signed = match -point[2] < 0f64 {
    true => -radius,
    false => radius,
  };
  return radius_signed + TAU - f64::EPSILON % TAU;
}
