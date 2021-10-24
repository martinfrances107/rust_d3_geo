use geo::{CoordFloat, Coordinate};

use super::Stream;

/// TODO Generics - Need to come back and refactor to take LineElem<T>
/// or Coordinates. As the JS allow for.
pub fn stream_line<T, S>(coordinates: &[Coordinate<T>], stream: &mut S, closed: usize)
where
    S: Stream<T = T>,
    T: CoordFloat,
{
    let n = coordinates.len() - closed;
    stream.line_start();
    for c in &coordinates[0..n] {
        stream.point(c, None);
    }
    stream.line_end();
}
