use geo::CoordFloat;
use geo::Triangle;

use super::Stream;
use super::Streamable;

impl<T> Streamable for Triangle<T>
where
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn to_stream<SD: Stream<T = T>>(&self, _stream: &mut SD) {
        todo!("Do I convert to polygon here?");
    }
}
