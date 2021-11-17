use geo::CoordFloat;
use geo::Polygon;

use super::stream_polygon;
use super::Stream;
use super::Streamable;

impl<T> Streamable for Polygon<T>
where
    T: CoordFloat,
{
    type T = T;

    fn to_stream<SD: Stream<T = T>>(&self, stream: &mut SD) {
        stream_polygon(self, stream);
    }
}
