// use std::fmt::Display;
// use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
// use num_traits::AsPrimitive;
use num_traits::FloatConst;

use super::Stream;

pub fn stream_line<T, S>(coordinates: &[Coordinate<T>], stream: &mut S, closed: usize)
where
    S: Stream<T = T>,
    T: CoordFloat + FloatConst,
{
    let n = coordinates.len() - closed;
    stream.line_start();
    for c in &coordinates[0..n] {
        stream.point(c, None);
    }
    stream.line_end();
}
