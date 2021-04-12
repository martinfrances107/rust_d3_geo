use std::ops::AddAssign;

use geo::LineString;
use geo::{coords_iter::CoordsIter, CoordFloat, Coordinate};
use num_traits::FloatConst;

use super::stream_line::stream_line;
use super::Streamable;
use crate::stream::Stream;

impl<T: AddAssign + CoordFloat + Default + FloatConst> Streamable<T> for LineString<T> {
    type SC = Coordinate<T>;
    fn to_stream(&self, stream: &mut impl Stream<T, C = Coordinate<T>>) {
        let points: Vec<Coordinate<T>> = self.coords_iter().collect();
        stream_line(&points, stream, 0);
    }
}
