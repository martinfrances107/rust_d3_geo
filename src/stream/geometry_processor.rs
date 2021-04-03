use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use super::Stream;

pub fn line_processor<T: AddAssign + CoordFloat + Default + FloatConst>(
    coordinates: &[Coordinate<T>],
    stream: &mut impl Stream<T, C = Coordinate<T>>,
    closed: usize,
) {
    let n = coordinates.len() - closed;
    let mut coordinate;
    stream.line_start();
    for i in 0..n {
        coordinate = coordinates[i].clone();
        stream.point(&coordinate, None);
    }
    stream.line_end();
}
