use geo::LineString;
use geo::MultiPoint;
use geo::Point;
use geo::Polygon;
use num_traits::Float;
#[derive(Clone, Debug)]
pub enum FeatureGeometry<T: Float> {
    Point(Point<T>),
    MultiPoint(MultiPoint<T>),
    Polygon(Polygon<T>),
    LineString(LineString<T>),
}
