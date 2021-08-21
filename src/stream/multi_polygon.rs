use geo::CoordFloat;
use geo::MultiPolygon;

use crate::stream::Stream;

use super::Streamable;

impl<T> Streamable for MultiPolygon<T>
where
    T: CoordFloat,
{
    type T = T;

    fn to_stream<SD: Stream<T = T>>(&self, stream: &mut SD) {
        for p in self.iter() {
            p.to_stream(stream);
        }
    }
}
