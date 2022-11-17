use geo::CoordFloat;
use geo::LineString;
use geo_types::Coord;

use crate::length::Stream;

/// Computes the distance between two 2-D surface points.
pub fn distance<T: CoordFloat>(a: &Coord<T>, b: &Coord<T>) -> T {
    let object = LineString(vec![*a, *b]);

    Stream::<T>::calc(&object)
}
