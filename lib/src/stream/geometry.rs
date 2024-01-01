use geo::CoordFloat;
use geo::Geometry;

use super::Stream;
use super::Streamable;

impl<T> Streamable for Geometry<T>
where
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn to_stream<EP, SD>(&self, stream: &mut SD)
    where
        SD: Stream<EP = EP, T = T>,
    {
        match self {
            Self::Point(p) => p.to_stream(stream),
            Self::LineString(ls) => ls.to_stream(stream),
            Self::Polygon(p) => p.to_stream(stream),
            Self::MultiPoint(multi_point) => multi_point.to_stream(stream),
            Self::MultiLineString(mls) => mls.to_stream(stream),
            Self::MultiPolygon(multi_polygon) => {
                multi_polygon.to_stream(stream);
            }
            Self::GeometryCollection(gc) => {
                for g in gc {
                    g.to_stream(stream);
                }
            }
            _ => {
                todo!("Covers Line, Rect and Triangle, The javascript original does not implement these")
            }
        }
    }
}
