use delaunator::Point;

use super::line::line;
use super::Stream;

pub fn polygon(coordinates: &[Vec<Point>], stream: &mut impl Stream) {
    stream.polygon_start();

    for c in coordinates {
        line(&c, stream, 1);
    }
    stream.polygon_end();
}
