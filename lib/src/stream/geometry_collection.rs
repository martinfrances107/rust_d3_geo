use geo::CoordFloat;
use geo::GeometryCollection;

use crate::stream::Stream;

use super::Streamable;

impl<T> Streamable for GeometryCollection<T>
where
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn to_stream<EP, SD>(&self, stream: &mut SD)
    where
        SD: Stream<EP = EP, T = T>,
    {
        for g in self {
            g.to_stream(stream);
        }
    }
}
