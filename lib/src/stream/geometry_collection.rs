use geo::{CoordFloat, GeometryCollection};
use num_traits::FloatConst;

use crate::stream::Stream;

use super::Streamable;

impl<T> Streamable for GeometryCollection<T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;
    #[inline]
    fn to_stream<EP, SD>(&self, stream: &mut SD)
    where
        EP: Stream<EP = EP, T = T> + Default,
        SD: Stream<EP = EP, T = T>,
    {
        for g in self {
            g.to_stream(stream);
        }
    }
}
