use num_traits::Float;

use super::Stream;
use super::line::line;

pub fn polygon<F>(coordinates: Vec<Vec<[F;2]>>, stream: &mut impl Stream<F>)
where
  F: Float,
{
  stream.polygon_start();

  for c in coordinates {
    line(&c, stream, 1);
  }
  stream.polygon_end();
}
