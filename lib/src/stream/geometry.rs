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
    fn to_stream<EP, SD>(&self, stream: &mut SD)
    where
        EP: Stream<EP = EP, T = T> + Default,
        SD: Stream<EP = EP, T = T>,
    {
        match self {
            Geometry::Point(p) => p.to_stream(stream),

            Geometry::LineString(ls) => ls.to_stream(stream),
            Geometry::Polygon(p) => p.to_stream(stream),
            Geometry::MultiPoint(multi_point) => multi_point.to_stream(stream),
            Geometry::MultiLineString(mls) => mls.to_stream(stream),
            Geometry::MultiPolygon(multi_polygon) => multi_polygon.to_stream(stream),
            Geometry::GeometryCollection(gc) => {
                for g in gc {
                    g.to_stream(stream);
                }
            }
            // Geometry::Line(l) => l.to_stream(stream),
            _ => {
                todo!("Covers Line, Rect and Triangle, The javascript original does not implement these")
            }
        }
    }
}
