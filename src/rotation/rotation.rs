use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::rotation::rotate_radians::RotateRadiams;
use crate::Transform;

use super::rotate_radians;

/// TODO can I simplify this.
#[derive(Clone, Debug)]
pub struct Rotation<T>
where
    T: CoordFloat + FloatConst,
{
    rotate: RotateRadiams<T>,
}

impl<T> Rotation<T>
where
    T: CoordFloat + FloatConst,
{
    /// Constructor.
    pub fn new(delta_lambda: T, delta_phi: T, delta_gamma: T) -> Self {
        Self {
            rotate: rotate_radians(
                delta_lambda.to_radians(),
                delta_phi.to_radians(),
                delta_gamma.to_radians(),
            ),
        }
    }
}

impl<T> Transform for Rotation<T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;
    fn transform(&self, coordinate: &Coordinate<T>) -> Coordinate<T> {
        let temp = self.rotate.transform(&Coordinate {
            x: coordinate.x.to_radians(),
            y: coordinate.y.to_radians(),
        });
        Coordinate {
            x: temp.x.to_degrees(),
            y: temp.y.to_degrees(),
        }
    }

    fn invert(&self, coordinate: &Coordinate<T>) -> Coordinate<T> {
        let temp = self.rotate.invert(&Coordinate {
            x: coordinate.x.to_radians(),
            y: coordinate.y.to_radians(),
        });
        Coordinate {
            x: temp.x.to_degrees(),
            y: temp.y.to_degrees(),
        }
    }
}
