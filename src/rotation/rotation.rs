use num_traits::Float;
use num_traits::FloatConst;

use super::rotate_radians::rotate_radians;
use crate::Transform;

pub struct Rotation<F> {
  rotate: Box<dyn Transform<F>>,
}

impl<F> Rotation<F> {
  pub fn new(delta_lambda: F, delta_phi: F, delta_gamma: F) -> Self
  where
    F: Float + FloatConst + 'static,
  {
    return Self {
      rotate: rotate_radians(
        delta_lambda.to_radians(),
        delta_phi.to_radians(),
        delta_gamma.to_radians(),
      ),
    };
  }
}

impl<F> Transform<F> for Rotation<F>
where
  F: Float,
{
  fn transform(&self, coordinates: &[F; 2]) -> [F; 2] {
    let temp = self
      .rotate
      .transform(&[coordinates[0].to_radians(), coordinates[1].to_radians()]);
    return [temp[0].to_degrees(), temp[1].to_degrees()];
  }

  fn invert(&self, coordinates: &[F; 2]) -> [F; 2] {
    let temp = self
      .rotate
      .invert(&[coordinates[0].to_radians(), coordinates[1].to_radians()]);
    return [temp[0].to_degrees(), temp[1].to_degrees()];
  }
}
