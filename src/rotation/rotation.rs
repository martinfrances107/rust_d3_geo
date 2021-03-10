use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use super::rotate_radians_transform::rotate_radians_transform;
use crate::Transform;
use crate::TransformClone;

// impl <T: CoordFloat> Clone for <dyn Transform<TcC=Coordinate<T>>>
// {
//     fn clone(&self) -> Self{

//     }

// }

pub struct Rotation<T>
where
    T: CoordFloat,
{
    rotate: Box<dyn Transform<TcC = Coordinate<T>>>,
}

impl<T: CoordFloat + FloatConst + Default + 'static> Rotation<T> {
    pub fn new(delta_lambda: T, delta_phi: T, delta_gamma: T) -> Self {
        return Self {
            rotate: rotate_radians_transform(
                delta_lambda.to_radians(),
                delta_phi.to_radians(),
                delta_gamma.to_radians(),
            ),
        };
    }
}

// impl<T: CoordFloat + 'static> Clone for Rotation<T> {
//     fn clone(&self) -> Self {
//         let s: RotateRadians = *self;
//         s
//     }
// }
impl<T: CoordFloat + 'static> TransformClone for Rotation<T> {
    type TcC = Coordinate<T>;
    fn box_clone(&self) -> Box<dyn Transform<TcC = Self::TcC>> {
        Box::new(Self {
            rotate: (*self.rotate).box_clone(),
        })
    }
}

impl<T: CoordFloat + 'static> Transform for Rotation<T> {
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
