use geo::Point;
use num_traits::Float;

use super::line::line;
use super::Stream;

pub fn polygon<T: Float>(coordinates: &[Vec<Point<T>>], stream: &mut impl Stream<T>) {
    stream.polygon_start();

    for c in coordinates {
        line(&c, stream, 1);
    }
    stream.polygon_end();
}
