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
    fn to_stream<SD: Stream<T = T>>(&self, stream: &mut SD) {
        for g in self {
            g.to_stream(stream);
        }
    }
}
