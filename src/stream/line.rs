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
    fn to_stream<SD: Stream<T = T>>(&self, _stream: &mut SD) {
        todo!("line start line end?");
    }
}
