use delaunator::Point;

use super::Stream;

pub fn point(coordinates: &[Point], stream: &mut impl Stream, closed: usize) {
    // let i = -1;
    let n = coordinates.len() - closed;
    stream.line_start();
    for i in 0..n {
        let coordinate = coordinates[i].clone();
        stream.point(coordinate.x, coordinate.y, None);
    }
    stream.line_end();
}