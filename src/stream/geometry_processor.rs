use super::Stream;
use geo::CoordFloat;
use geo::Point;
use num_traits::FloatConst;

pub fn line_processor<T: CoordFloat + FloatConst>(
    coordinates: &[Point<T>],
    stream: &mut impl Stream<T>,
    closed: usize,
) {
    let n = coordinates.len() - closed;
    let mut coordinate;
    stream.line_start();
    for i in 0..n {
        coordinate = coordinates[i].clone();
        stream.point(coordinate.x(), coordinate.y(), None);
    }
    stream.line_end();
}
