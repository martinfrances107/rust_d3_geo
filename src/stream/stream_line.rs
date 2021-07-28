// use std::fmt::Display;
// use std::ops::AddAssign;

// use geo::{CoordFloat, Coordinate};
// use num_traits::AsPrimitive;
// use num_traits::FloatConst;

use super::Stream;

pub fn stream_line<C, S>(coordinates: &[C], stream: &mut S, closed: usize)
where
    S: Stream<SC = C>,
{
    let n = coordinates.len() - closed;
    stream.line_start();
    for c in &coordinates[0..n] {
        stream.point(c, None);
    }
    stream.line_end();
}
