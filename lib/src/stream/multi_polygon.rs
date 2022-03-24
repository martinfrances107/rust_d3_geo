use geo::CoordFloat;
use geo::MultiPolygon;

use crate::stream::Stream;

use super::stream_polygon;
use super::Streamable;

impl<T> Streamable for MultiPolygon<T>
where
    T: CoordFloat,
{
    /// f64 or f32.
    type T = T;

    #[inline]
    fn to_stream<EP, SD>(&self, stream: &mut SD)
    where
        EP: Stream<EP = EP, T = T> + Default,
        SD: Stream<EP = EP, T = T>,
    {
        for p in self.iter() {
            stream_polygon(p, stream);
        }
    }
}
