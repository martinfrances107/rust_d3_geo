use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use super::Stream;

pub fn stream_line<T: AddAssign + CoordFloat + Default + FloatConst>(
    coordinates: &[Coordinate<T>],
    stream: &mut impl Stream<T, C = Coordinate<T>>,
    closed: usize,
) {
    let n = coordinates.len() - closed;
    stream.line_start();
    for i in 0..n {
        stream.point(&coordinates[i], None);
    }
    stream.line_end();
}
