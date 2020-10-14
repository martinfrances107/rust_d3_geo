use delaunator::Point;

pub fn in_delta(actual: f64, expected: f64, delta: f64) -> bool
{
  println!("expected {:?} actual {:?} delta {:?}", expected, actual, delta);
  return (actual - expected).abs() <= delta;
}
