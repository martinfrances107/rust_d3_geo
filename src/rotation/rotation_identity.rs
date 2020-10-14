use std::f64;
use delaunator::Point;

use crate::Transform;
use crate::math::TAU;

#[derive(Clone, Debug)]
pub struct RotationIdentity {}

// By design a stateless function.
// TODO maybe add attributes to suggest inlining this where possible.
fn normalise(p: &Point) -> Point
{
  let lambda = p.x;
  let phi = p.y;

  return match lambda.abs() > f64::consts::PI {
    true => Point{x:lambda + (-lambda / TAU).round() * TAU, y:phi},
    false => Point{x:lambda, y:phi},
  };
}

impl RotationIdentity {
  pub fn new() -> Box<dyn Transform>
  {
    return Box::new(RotationIdentity {});
  }
}

impl Transform for RotationIdentity
{
  fn transform(&self, p: &Point) -> Point {
    return normalise(p);
  }

  fn invert(&self, p: &Point) -> Point {
    return normalise(p);
  }
}
