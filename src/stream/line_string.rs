use geo::LineString;
use geo::{coords_iter::CoordsIter, CoordFloat, Coordinate};

use super::stream_line::stream_line;
use super::Streamable;
use crate::stream::Stream;

impl<T: CoordFloat> Streamable for LineString<T> {
    // type SD = LineString<T>;
    type T = T;
    fn to_stream<SD: Stream<T = T>>(&self, stream: &mut SD) {
        let points: Vec<Coordinate<T>> = self.coords_iter().collect();
        stream_line(&points, stream, 0);
    }
}
