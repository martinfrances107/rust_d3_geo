use geo::Polygon;
use geo::{CoordFloat, Point};
use num_traits::FloatConst;

use super::geometry_processor::line_processor;
use super::Stream;
use super::Streamable;

impl<T: CoordFloat + FloatConst> Streamable<T> for Polygon<T> {
    fn to_stream(&self, stream: &mut impl Stream<T>) {
        stream.polygon_start();

        let e_points: Vec<Point<T>> = self.exterior().points_iter().collect();
        line_processor(&e_points, stream, 1);

        for i in self.interiors() {
            let line_points: Vec<Point<T>> = i.points_iter().collect();
            line_processor(&line_points, stream, 1);
        }
        stream.polygon_end();
    }
}
