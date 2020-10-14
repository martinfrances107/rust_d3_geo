use delaunator::Point;

use super::Stream;
use super::line::line;

pub fn polygon(coordinates: &[Vec<Point>], stream: &mut impl Stream)
{
  stream.polygon_start();

  for c in coordinates {
    line(&c, stream, 1);
  }
  stream.polygon_end();
}
