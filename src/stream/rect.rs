use geo::CoordFloat;
use geo::Rect;

use super::Stream;
use super::Streamable;

impl<T> Streamable for Rect<T>
where
    T: CoordFloat,
{
    type T = T;
    #[inline]
    fn to_stream<SD: Stream<T = T>>(&self, _stream: &mut SD) {
        todo!("Do I convert to polygon here?");
    }
}
