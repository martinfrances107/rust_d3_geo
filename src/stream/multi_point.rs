use geo::MultiPoint;
use geo::{CoordFloat, Coordinate};

use crate::stream::Stream;

use super::Streamable;

impl<T> Streamable for MultiPoint<T>
where
    T: CoordFloat,
{
    type T = T;
    // type SD = Self;
    fn to_stream<SD: Stream<T = T>>(&self, stream: &mut SD) {
        for p in self.iter() {
            // TODO there must be a better conversion.
            stream.point(&Coordinate { x: p.x(), y: p.y() }, None);
        }
    }
}
