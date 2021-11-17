use geo::CoordFloat;
use geo::LineString;

use crate::stream::Stream;

use super::stream_line;
use super::Streamable;

impl<T: CoordFloat> Streamable for LineString<T> {
    type T = T;

    #[inline]
    fn to_stream<SD: Stream<T = T>>(&self, stream: &mut SD) {
        stream_line(self, stream, 0);
    }
}
