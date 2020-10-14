use delaunator::Point;

use super::rotate_radians::RotateRadians;
use crate::Transform;

pub struct Rotation{
  rotate: Box<dyn Transform>,
}

impl Rotation {
  pub fn new(delta_lambda: f64, delta_phi: f64, delta_gamma: f64) -> Self
  {
    return Self {
      rotate: RotateRadians::new(
        delta_lambda.to_radians(),
        delta_phi.to_radians(),
        delta_gamma.to_radians(),
      ),
    };
  }
}

impl Transform for Rotation
{
  fn transform(&self, coordinates: &Point) -> Point {
    let temp = self
      .rotate
      .transform(&Point{x:coordinates.x.to_radians(), y:coordinates.y.to_radians()});
    return Point{x:temp.x.to_degrees(), y:temp.y.to_degrees()};
  }

  fn invert(&self, coordinates: &Point) -> Point {
    let temp = self
      .rotate
      .invert(&Point{x:coordinates.x.to_radians(), y:coordinates.y.to_radians()});
    return Point{x:temp.x.to_degrees(), y:temp.y.to_degrees()};
  }
}
