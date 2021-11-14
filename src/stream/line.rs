use geo::CoordFloat;
use geo::Line;

use super::Stream;
use super::Streamable;

impl<T> Streamable for Line<T>
where
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn to_stream<SD: Stream<T = T>>(&self, stream: &mut SD) {
        stream.line_start();
        stream.point(&self.start, None);
        stream.point(&self.end, None);
        stream.line_end();
    }
}
