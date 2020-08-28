use num_traits::Float;

use super::Stream;

pub fn line<F>(coordinates: &Vec<[F; 2]>, stream: &mut impl Stream<F>, closed: usize)
where
  F: Float,
{
  // let i = -1;
  let n = coordinates.len() - closed;
  stream.line_start();
  for i in 0..n {
    let coordinate = coordinates[i];
    stream.point(coordinate[0], coordinate[1], None);
  }
  stream.line_end();
}

// function streamLine(coordinates, stream, closed) {
//   var i = -1, n = coordinates.length - closed, coordinate;
//   stream.lineStart();
//   while (++i < n) coordinate = coordinates[i], stream.point(coordinate[0], coordinate[1], coordinate[2]);
//   stream.lineEnd();
// }
