use geo::CoordFloat;
use geo::Polygon;

use super::stream_line::stream_line;
use super::Stream;
use super::Streamable;

impl<T> Streamable for Polygon<T>
where
    T: CoordFloat,
{
    type T = T;

    fn to_stream<SD: Stream<T = T>>(&self, stream: &mut SD) {
        stream.polygon_start();

        stream_line(&self.exterior().0, stream, 1);

        for i in self.interiors() {
            stream_line(&i.0, stream, 1);
        }
        stream.polygon_end();
    }
}
