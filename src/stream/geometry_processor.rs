use super::Stream;
use geo::{CoordFloat, Coordinate};

use num_traits::FloatConst;

pub fn line_processor<T: CoordFloat + FloatConst>(
    coordinates: &[Coordinate<T>],
    stream: impl Stream<ScC = Coordinate<T>>,
    closed: usize,
) {
    let n = coordinates.len() - closed;
    let mut coordinate;
    stream.line_start();
    for i in 0..n {
        coordinate = coordinates[i].clone();
        stream.point(coordinate, None);
    }
    stream.line_end();
}
