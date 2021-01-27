use super::Streamable;
use crate::stream::Stream;
use geo::Geometry;
use geo::{CoordFloat, GeometryCollection};
use num_traits::FloatConst;

impl<T> Streamable<T> for Geometry<T>
where
    T: CoordFloat + FloatConst,
{
    fn to_stream(&self, stream: &mut impl Stream<T>) {
        match self {
            g => {
                g.to_stream(stream);
            }
        }
    }
}
