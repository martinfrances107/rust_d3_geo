use geo::CoordFloat;
use geo::Point;

use super::Stream;
use super::Streamable;

impl<T> Streamable for Point<T>
where
    T: CoordFloat,
{
    type T = T;
    #[inline]
    fn to_stream<EP, SD>(&self, stream: &mut SD)
    where
        SD: Stream<EP = EP, T = T>,
    {
        stream.point(&self.0, None);
    }
}
