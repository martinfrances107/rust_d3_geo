use geo::CoordFloat;
use geo::MultiPoint;

use crate::stream::Stream;

use super::Streamable;

impl<T> Streamable for MultiPoint<T>
where
    T: CoordFloat,
{
    type T = T;

    fn to_stream<EP, SD>(&self, stream: &mut SD)
    where
        // EP: Stream<EP = EP, T = T> + Default,
        SD: Stream<EP = EP, T = T>,
    {
        for p in self.iter() {
            stream.point(&p.0, None);
        }
    }
}
