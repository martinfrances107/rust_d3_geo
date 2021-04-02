use super::geometry_processor::line_processor;
use super::Streamable;
use crate::stream::Stream;
use geo::LineString;
use geo::{coords_iter::CoordsIter, CoordFloat, Coordinate};
use num_traits::FloatConst;

impl<T: CoordFloat + Default + FloatConst> Streamable<T> for LineString<T> {
    type SC = Coordinate<T>;
    fn to_stream(&self, stream: &mut impl Stream<T, C = Coordinate<T>>) {
        let points: Vec<Coordinate<T>> = self.coords_iter().collect();
        line_processor(&points, stream, 0);
    }
}
