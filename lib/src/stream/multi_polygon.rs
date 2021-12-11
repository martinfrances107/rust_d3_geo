use std::fmt::Debug;

use geo::CoordFloat;
use geo::MultiPolygon;

use crate::stream::Stream;

use super::Streamable;

impl<T> Streamable for MultiPolygon<T>
where
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn to_stream<EP, SD>(&self, stream: &mut SD)
    where
        EP: Clone + Debug + Stream<EP = EP, T = T>,
        SD: Stream<EP = EP, T = T>,
    {
        for p in self.iter() {
            p.to_stream(stream);
        }
    }
}
