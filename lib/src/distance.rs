use geo::{CoordFloat, Coordinate, LineString};

use crate::length::Stream;

/// Computes the distance between two 2-D surface points.
pub fn distance<T: CoordFloat>(a: &Coordinate<T>, b: &Coordinate<T>) -> T {
    let object = LineString(vec![*a, *b]);

    Stream::<T>::calc(&object)
}
