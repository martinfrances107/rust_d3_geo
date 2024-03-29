use geo::CoordFloat;
use geo::MultiLineString;

use super::stream_line;
use super::Stream;
use super::Streamable;

impl<T> Streamable for MultiLineString<T>
where
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn to_stream<EP, SD>(&self, stream: &mut SD)
    where
        SD: Stream<EP = EP, T = T>,
    {
        for ls in self {
            stream_line(ls, stream, 0);
        }
    }
}
