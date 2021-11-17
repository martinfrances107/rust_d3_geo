use geo::CoordFloat;
use geo::MultiLineString;

use super::stream_line;
use super::{Stream, Streamable};

impl<T> Streamable for MultiLineString<T>
where
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn to_stream<SD: Stream<T = T>>(&self, stream: &mut SD) {
        for ls in self {
            stream_line(&ls, stream, 0);
        }
    }
}
