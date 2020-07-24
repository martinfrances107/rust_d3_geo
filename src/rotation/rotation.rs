use num_traits::Float;
use num_traits::FloatConst;

use super::rotate_radians::rotate_radians;
use crate::Transform;

pub struct Rotation<T> {
  rotate: Box<dyn Transform<T>>,
}

impl<T: 'static> Rotation<T> {
  pub fn new(delta_lambda: T, delta_phi: T, delta_gamma: T) -> Self
  where
    T: Float + FloatConst,
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

impl<T> Transform<T> for Rotation<T>
where
  T: Float,
{
  fn transform(&self, coordinates: &[T; 2]) -> [T; 2] {
    let temp = self
      .rotate
      .transform(&[coordinates[0].to_radians(), coordinates[1].to_radians()]);
    return [temp[0].to_degrees(), temp[1].to_degrees()];
  }

  fn invert(&self, coordinates: &[T; 2]) -> [T; 2] {
    let temp = self
      .rotate
      .invert(&[coordinates[0].to_radians(), coordinates[1].to_radians()]);
    return [temp[0].to_degrees(), temp[1].to_degrees()];
  }
}
