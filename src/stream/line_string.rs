use std::fmt::Display;
use std::ops::AddAssign;

use geo::LineString;
use geo::{coords_iter::CoordsIter, CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use super::stream_line::stream_line;
use super::Streamable;
use crate::stream::Stream;

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst> Streamable
    for LineString<T>
{
    // type SD = LineString<T>;
    type T = T;
    fn to_stream<SD: Stream<T = T>>(&self, stream: &mut SD) {
        let points: Vec<Coordinate<T>> = self.coords_iter().collect();
        stream_line(&points, stream, 0);
    }
}
