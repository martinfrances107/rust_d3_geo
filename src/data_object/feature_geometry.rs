use geo::MultiPoint;
use geo::Point;
use geo::Polygon;
use geo::{CoordFloat, LineString};
// use num_traits::Float;

#[derive(Clone, Debug)]
pub enum FeatureGeometry<T: CoordFloat> {
    Point(Point<T>),
    MultiPoint(MultiPoint<T>),
    Polygon(Polygon<T>),
    LineString(LineString<T>),
}
