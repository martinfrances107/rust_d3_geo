use super::Streamable;
use crate::stream::Stream;
use geo::CoordFloat;
use geo::Geometry;
use num_traits::FloatConst;

impl<T> Streamable<T> for Geometry<T>
where
    T: CoordFloat + FloatConst,
{
    fn to_stream(&self, stream: &mut impl Stream<T>) {
        match self {
            Geometry::GeometryCollection(gc) => {
                for g in gc {
                    g.to_stream(stream);
                }
            }
            g => {
                g.to_stream(stream);
            }
        }
    }
}
