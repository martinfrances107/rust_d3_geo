use delaunator::Point;

use crate::math::EPSILON;

// export default function(a, b) {
//   return abs(a[0] - b[0]) < epsilon && abs(a[1] - b[1]) < epsilon;
// }

/// An aspect of the javascrtipt
/// point_equal may be fed 3 floats but only checks values 0 and 1. (so not all) !!!
pub fn point_equal(a: Point, b : Point) -> bool  {
  return ((a.x - b.x).abs() < EPSILON) &&
         ((a.y - b.y).abs() < EPSILON);
}
