use geo::MultiLineString;
use geo::{coords_iter::CoordsIter, CoordFloat, Coordinate};

use super::stream_line::stream_line;
use super::{Stream, Streamable};

/// MultiLineString - an array of arrays of positions forming several lines.

impl<T> Streamable for MultiLineString<T>
where
    T: CoordFloat,
{
    type T = T;

    fn to_stream<SD: Stream<T = T>>(&self, stream: &mut SD) {
        for ls in self {
            let points: Vec<Coordinate<T>> = ls.coords_iter().collect();
            stream_line(&points, stream, 0);
        }
    }
}
