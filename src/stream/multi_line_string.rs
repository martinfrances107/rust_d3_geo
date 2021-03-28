use geo::MultiLineString;
use geo::{coords_iter::CoordsIter, CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::stream::geometry_processor::line_processor;

use super::{Stream, Streamable};

/// MultiLineString - an array of arrays of positions forming several lines.

impl<T: CoordFloat + Default + FloatConst> Streamable<T> for MultiLineString<T> {
    type SC = Coordinate<T>;
    fn to_stream(&self, stream: &mut impl Stream<T, C = Self::SC>) {
        for ls in self {
            let points: Vec<Coordinate<T>> = ls.coords_iter().collect();
            line_processor(&points, stream, 0);
        }
    }
}
