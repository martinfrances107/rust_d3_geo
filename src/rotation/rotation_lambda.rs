use std::f64;

use delaunator::Point;

use crate::math::TAU;
use crate::Transform;

#[derive(Debug)]
pub struct RotationLambda
{
  pub delta_lambda: f64,
}

// TODO why can't I #[inline] this.
fn forward_rotation_lambda(delta_lambda: f64, p: &Point) -> Point
{
  let lambda = p.x + delta_lambda;
  let phi = p.y;
  return match (lambda > f64::consts::PI, lambda < -f64::consts::PI) {
    (false, false) => Point{x:lambda, y:phi},       // -PI <= lambda <= PI
    (true, _) => Point{x:lambda - TAU, y:phi}, // lambda >  PI
    (_, true) => Point{x:lambda + TAU, y:phi}, // lambda < -PI
  };
}

impl RotationLambda
{
  pub fn new(delta_lambda: f64) -> Box<dyn Transform> {
    return Box::new(Self { delta_lambda });
  }
}

impl Transform for RotationLambda
{
  fn transform(&self, coordinates: &Point) -> Point {
    return forward_rotation_lambda(self.delta_lambda, coordinates);
  }
  fn invert(&self, coordinates: &Point) -> Point {
    return forward_rotation_lambda(-self.delta_lambda, coordinates);
  }
}
