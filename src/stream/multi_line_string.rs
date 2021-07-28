use std::fmt::Display;
use std::ops::AddAssign;

use geo::MultiLineString;
use geo::{coords_iter::CoordsIter, CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use super::stream_line::stream_line;
use super::{Stream, Streamable};

/// MultiLineString - an array of arrays of positions forming several lines.

impl<T> Streamable for MultiLineString<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type T = T;

    fn to_stream<SD: Stream<SC = Coordinate<T>>>(&self, stream: &mut SD) {
        for ls in self {
            let points: Vec<Coordinate<T>> = ls.coords_iter().collect();
            stream_line(&points, stream, 0);
        }
    }
}
