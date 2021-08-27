use geo::{CoordFloat, Coordinate, LineString};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::length::Stream;

/// Computes the distancre between two 2-D surface points.
pub fn distance<T: AsPrimitive<T> + CoordFloat + FloatConst>(
    a: &Coordinate<T>,
    b: &Coordinate<T>,
) -> T {
    // TODO consider making object static outside of distance.
    // It does not need to be created each time.
    let object = LineString(vec![*a, *b]);

    Stream::<T>::calc(&object)
}
