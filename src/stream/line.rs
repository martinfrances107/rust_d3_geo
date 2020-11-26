use delaunator::Point;

use super::Stream;

pub fn line(coordinates: &[Point], stream: &mut impl Stream, closed: usize) {
    let n = coordinates.len() - closed;
    let mut coordinate;
    stream.line_start();
    for i in 0..n {
        coordinate = coordinates[i].clone();
        stream.point(coordinate.x, coordinate.y, None);
    }
    stream.line_end();
}
