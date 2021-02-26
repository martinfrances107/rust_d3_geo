use geo::CoordFloat;
use geo::{coords_iter::CoordsIter, Coordinate, Polygon};
use num_traits::FloatConst;

use super::geometry_processor::line_processor;
use super::Stream;
use super::Streamable;

impl<T: CoordFloat + FloatConst> Streamable for Polygon<T> {
    type SC = Coordinate<T>;
    fn to_stream(&self, stream: impl Stream<ScC = Self::SC>) {
        stream.polygon_start();

        let e_points: Vec<Coordinate<T>> = self.exterior().coords_iter().collect();
        line_processor(&e_points, stream, 1);

        for i in self.interiors() {
            let line_points: Vec<Coordinate<T>> = i.coords_iter().collect();
            line_processor(&line_points, stream, 1);
        }
        stream.polygon_end();
    }
}
