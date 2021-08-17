use std::fmt::Display;

use geo::CoordFloat;
use geo::{Coordinate, Point};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use super::Stream;
use super::Streamable;

// Move this to another file.
impl<T: AsPrimitive<T> + CoordFloat + Display + FloatConst> Streamable for Point<T> {
    type T = T;
    #[inline]
    fn to_stream<SD: Stream<T = T>>(&self, stream: &mut SD) {
        // TODO there must be a better way to cast a Point to Coordinate.
        stream.point(
            &Coordinate {
                x: self.x(),
                y: self.y(),
            },
            None,
        );
    }
}
