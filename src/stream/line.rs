use delaunator::Point;

use super::Stream;

pub fn line(coordinates: &[Point], stream: &mut impl Stream, closed: usize)
{
  // let i = -1;
  let n = coordinates.len() - closed;
  stream.line_start();
  for i in 0..n {
    let coordinate = coordinates[i].clone();
    stream.point(coordinate.x, coordinate.y, None);
  }
  stream.line_end();
}

// function streamLine(coordinates, stream, closed) {
//   var i = -1, n = coordinates.length - closed, coordinate;
//   stream.lineStart();
//   while (++i < n) coordinate = coordinates[i], stream.point(coordinate[0], coordinate[1], coordinate[2]);
//   stream.lineEnd();
// }
