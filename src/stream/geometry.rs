use super::Streamable;
use crate::stream::Stream;
use geo::CoordFloat;
use geo::Coordinate;
use geo::Geometry;
use num_traits::FloatConst;

impl<T> Streamable for Geometry<T>
where
    T: CoordFloat + FloatConst,
{
    type SC = Coordinate<T>;
    fn to_stream(&self, stream: impl Stream<ScC = Self::SC>) {
        match self {
            Geometry::GeometryCollection(gc) => {
                for g in gc {
                    g.to_stream(stream);
                }
            }
            Geometry::Point(p) => {
                p.to_stream(stream);
            }
            g => {
                g.to_stream(stream);
            }
        }
    }
}
