use super::geometry_processor::line_processor;
use super::Streamable;
use crate::stream::Stream;
use geo::CoordFloat;
use geo::LineString;
use num_traits::FloatConst;

use geo::Point;
impl<T: CoordFloat + FloatConst> Streamable<T> for LineString<T> {
    fn to_stream(&self, stream: &mut impl Stream<T>) {
        // processor(&Geometry::LineString(self.clone()), stream);
        let points: Vec<Point<T>> = self.points_iter().collect();
        line_processor(&points, stream, 0);
    }
}
