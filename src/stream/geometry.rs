use geo::CoordFloat;
use geo::Geometry;
use num_traits::FloatConst;

use super::Stream;
use super::Streamable;

impl<T> Streamable for Geometry<T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn to_stream<SD: Stream<T = T>>(&self, stream: &mut SD) {
        match self {
            Geometry::Point(p) => p.to_stream(stream),

            Geometry::LineString(ls) => ls.to_stream(stream),
            Geometry::Polygon(p) => p.to_stream(stream),
            Geometry::MultiPoint(mp) => mp.to_stream(stream),
            Geometry::MultiLineString(mls) => mls.to_stream(stream),
            Geometry::MultiPolygon(mp) => mp.to_stream(stream),
            Geometry::GeometryCollection(gc) => {
                for g in gc {
                    g.to_stream(stream);
                }
            }
            // Geometry::Line(l) => l.to_stream(stream),
            _ => {
                todo!("Covers Line, Rect and Triangle, The javascript original does not implement theese")
            }
        }
    }
}
