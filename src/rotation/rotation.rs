use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use super::rotate_radians::RotateRadians;
use crate::Transform;

pub struct Rotation<T> {
    rotate: Box<dyn Transform<T>>,
}

impl<T: CoordFloat + FloatConst + 'static> Rotation<T> {
    pub fn new(delta_lambda: T, delta_phi: T, delta_gamma: T) -> Self {
        return Self {
            rotate: RotateRadians::new(
                delta_lambda.to_radians(),
                delta_phi.to_radians(),
                delta_gamma.to_radians(),
            ),
        };
    }
}

impl<T: CoordFloat> Transform<T> for Rotation<T> {
    fn transform(&self, coordinates: &Coordinate<T>) -> Coordinate<T> {
        let temp = self.rotate.transform(&Coordinate {
            x: coordinates.x.to_radians(),
            y: coordinates.y.to_radians(),
        });
        return Coordinate {
            x: temp.x.to_degrees(),
            y: temp.y.to_degrees(),
        };
    }

    fn invert(&self, coordinates: &Coordinate<T>) -> Coordinate<T> {
        let temp = self.rotate.invert(&Coordinate {
            x: coordinates.x.to_radians(),
            y: coordinates.y.to_radians(),
        });
        return Coordinate {
            x: temp.x.to_degrees(),
            y: temp.y.to_degrees(),
        };
    }
}
