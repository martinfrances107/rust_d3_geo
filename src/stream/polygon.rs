use std::fmt::Debug;

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
        EP: Clone + Debug + Stream<EP = EP, T = T>,
        SD: Stream<EP = EP, T = T>,
    {
        stream_polygon(self, stream);
    }
}
