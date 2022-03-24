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

    fn to_stream<EP, SD>(&self, stream: &mut SD)
    where
        EP: Stream<EP = EP, T = T> + Default,
        SD: Stream<EP = EP, T = T>,
    {
        stream_polygon(self, stream);
    }
}
