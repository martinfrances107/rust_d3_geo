use std::fmt::Display;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::rotation::rotate_radians_enum::RotateRadiansEnum;
use crate::Transform;

use super::rotate_radians::rotate_radians;

#[derive(Clone, Debug)]
pub struct Rotation<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    rotate: RotateRadiansEnum<T>,
}

impl<T> Rotation<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
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

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst> Transform for Rotation<T> {
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
