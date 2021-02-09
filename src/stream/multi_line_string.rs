use super::{Stream, Streamable};
use crate::stream::geometry_processor::line_processor;
use geo::MultiLineString;
use geo::{coords_iter::CoordsIter, CoordFloat, Coordinate};
use num_traits::FloatConst;

/// MultiLineString - an array of arrays of positions forming several lines.

impl<T: CoordFloat + FloatConst> Streamable<T> for MultiLineString<T> {
    fn to_stream(&self, stream: &mut impl Stream<T>) {
        for ls in self {
            let points: Vec<Coordinate<T>> = ls.coords_iter().collect();
            line_processor(&points, stream, 0);
        }
    }
}
