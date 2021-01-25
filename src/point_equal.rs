use geo::{CoordFloat, Coordinate};

// use crate::math::EPSILON;

// export default function(a, b) {
//   return abs(a[0] - b[0]) < epsilon && abs(a[1] - b[1]) < epsilon;
// }

/// An aspect of the javascrtipt
/// point_equal may be fed 3 floats but only checks values 0 and 1. (so not all) !!!
pub fn point_equal<T: CoordFloat>(a: Coordinate<T>, b: Coordinate<T>) -> bool {
    (a.x - b.x.abs() < T::epsilon()) && ((a.y - b.y).abs() < T::epsilon())
}
