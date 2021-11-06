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
        stream.point(&self.0, None);
    }
}
