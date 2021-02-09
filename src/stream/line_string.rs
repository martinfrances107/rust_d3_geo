use super::geometry_processor::line_processor;
use super::Streamable;
use crate::stream::Stream;
use geo::LineString;
use geo::{coords_iter::CoordsIter, CoordFloat, Coordinate};
use num_traits::FloatConst;

impl<T: CoordFloat + FloatConst> Streamable<T> for LineString<T> {
    fn to_stream(&self, stream: &mut impl Stream<T>) {
        // processor(&Geometry::LineString(self.clone()), stream);
        let points: Vec<Coordinate<T>> = self.coords_iter().collect();
        line_processor(&points, stream, 0);
    }
}
