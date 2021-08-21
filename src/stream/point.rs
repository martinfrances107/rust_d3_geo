use geo::CoordFloat;
use geo::{Coordinate, Point};

use super::Stream;
use super::Streamable;

impl<T> Streamable for Point<T>
where
    T: CoordFloat,
{
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
