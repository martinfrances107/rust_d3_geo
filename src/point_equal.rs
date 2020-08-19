use num_traits::cast::FromPrimitive;
use num_traits::Float;

use crate::math::epsilon;


// export default function(a, b) {
//   return abs(a[0] - b[0]) < epsilon && abs(a[1] - b[1]) < epsilon;
// }

/// An aspect of the javascrtipt
/// point_equal may be fed 3 floats but only checks values 0 and 1. (so not all) !!!
pub fn point_equal<F>(a: [F;2], b : [F;2]) -> bool
where F: Float + FromPrimitive{
  return ((a[0] - b[0]).abs() < epsilon::<F>()) &&
         ((a[1] - b[1]).abs() < epsilon::<F>());
}
