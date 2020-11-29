use delaunator::Point;

use crate::data_object::line_string::LineString;
use crate::length::LengthStream;

pub fn distance(a: &Point, b: &Point) -> f64 {
    // TODO consider making object static outside of distance.
    // It does not need to be created each time.
    let object = LineString {
        coordinates: vec![(*a).clone(), (*b).clone()],
    };

    return LengthStream::calc(&object);
}
