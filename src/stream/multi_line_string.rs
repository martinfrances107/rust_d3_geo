use std::fmt::Display;
use std::ops::AddAssign;

use geo::MultiLineString;
use geo::{coords_iter::CoordsIter, CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use super::stream_line::stream_line;
use super::{Stream, Streamable};

/// MultiLineString - an array of arrays of positions forming several lines.

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst> Streamable<T>
    for MultiLineString<T>
{
    type SC = Coordinate<T>;
    fn to_stream(&self, stream: &mut impl Stream<T, C = Self::SC>) {
        for ls in self {
            let points: Vec<Coordinate<T>> = ls.coords_iter().collect();
            stream_line(&points, stream, 0);
        }
    }
}
