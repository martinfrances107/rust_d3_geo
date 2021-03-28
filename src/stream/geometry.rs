use geo::CoordFloat;
use geo::Coordinate;
use geo::Geometry;
use num_traits::FloatConst;

use super::Stream;
use super::Streamable;

impl<T> Streamable<T> for Geometry<T>
where
    T: CoordFloat + Default + FloatConst,
{
    type SC = Coordinate<T>;
    fn to_stream(&self, stream: &mut impl Stream<T, C = Self::SC>) {
        match self {
            Geometry::Point(p) => p.to_stream(stream),
            Geometry::Line(l) => l.to_stream(stream),
            Geometry::LineString(ls) => ls.to_stream(stream),
            Geometry::Polygon(p) => p.to_stream(stream),
            Geometry::MultiPoint(mp) => mp.to_stream(stream),
            Geometry::MultiLineString(mls) => mls.to_stream(stream),
            Geometry::MultiPolygon(mp) => mp.to_stream(stream),
            Geometry::Rect(r) => r.to_stream(stream),
            Geometry::Triangle(t) => t.to_stream(stream),
            Geometry::GeometryCollection(gc) => {
                for g in gc {
                    g.to_stream(stream);
                }
            }
        }
    }
}
