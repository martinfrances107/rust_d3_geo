use geo::CoordFloat;
use geo::{coords_iter::CoordsIter, Coordinate, Polygon};

use super::stream_line::stream_line;
use super::Stream;
use super::Streamable;

impl<T> Streamable for Polygon<T>
where
    T: CoordFloat,
{
    // type T=T;
    // type SD = Self;
    type T = T;
    fn to_stream<SD: Stream<T = T>>(&self, stream: &mut SD) {
        stream.polygon_start();

        let e_points: Vec<Coordinate<T>> = self.exterior().coords_iter().collect();
        stream_line(&e_points, stream, 1);

        for i in self.interiors() {
            let line_points: Vec<Coordinate<T>> = i.coords_iter().collect();
            stream_line(&line_points, stream, 1);
        }
        stream.polygon_end();
    }
}
