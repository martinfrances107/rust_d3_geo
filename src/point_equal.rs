use geo::{CoordFloat, Coordinate};

/// An aspect of the javascrtipt
/// point_equal may be fed 3 floats but only checks values 0 and 1. (so not all) !!!
#[cfg(not(tarpaulin_include))]
#[inline]
pub fn point_equal<T: CoordFloat>(a: Coordinate<T>, b: Coordinate<T>) -> bool {
    let epsilon: T = T::from(1e-6).unwrap();
    ((a.x - b.x).abs() < epsilon) && ((a.y - b.y).abs() < epsilon)
}
