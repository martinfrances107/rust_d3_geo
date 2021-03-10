use super::geometry_processor::line_processor;
use super::Streamable;
use crate::stream::Stream;
use geo::LineString;
use geo::{coords_iter::CoordsIter, CoordFloat, Coordinate};
use num_traits::FloatConst;

impl<T: CoordFloat + FloatConst> Streamable for LineString<T> {
    type SC = Coordinate<T>;
    fn to_stream(&self, stream: &mut Box<dyn Stream<C = Coordinate<T>>>) {
        // processor(&Geometry::LineString(self.clone()), stream);
        let points: Vec<Coordinate<T>> = self.coords_iter().collect();
        line_processor(&points, stream, 0);
    }
}
