use geo::CoordFloat;
use geo::MultiPoint;

use crate::stream::Stream;

use super::Streamable;

impl<T> Streamable for MultiPoint<T>
where
    T: CoordFloat,
{
    type T = T;

    fn to_stream<SD: Stream<T = T>>(&self, stream: &mut SD) {
        for p in self.iter() {
            stream.point(&p.0, None);
        }
    }
}
