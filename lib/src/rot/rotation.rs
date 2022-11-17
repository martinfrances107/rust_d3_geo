use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::Transform;

use super::rotate_radians;
use super::rotate_radians::RotateRadians;

/// Transform converting degree to radians.
///
/// All other rotations elements assume all angles are in radians.
#[derive(Clone, Debug)]
pub struct Rotation<T>
where
    T: CoordFloat,
{
    rotate: RotateRadians<T>,
}

impl<T> Rotation<T>
where
    T: CoordFloat + FloatConst,
{
    /// Constructor.
    #[inline]
    pub fn new(delta_lambda: T, delta_phi: T, delta_gamma: T) -> Self {
        Self {
            rotate: rotate_radians([
                delta_lambda.to_radians(),
                delta_phi.to_radians(),
                delta_gamma.to_radians(),
            ]),
        }
    }
}

impl<T> Transform for Rotation<T>
where
    T: CoordFloat + FloatConst,
{
    ///f64 or f32.
    type T = T;

    fn transform(&self, coordinate: &Coord<T>) -> Coord<T> {
        let temp = self.rotate.transform(&Coord {
            x: coordinate.x.to_radians(),
            y: coordinate.y.to_radians(),
        });
        Coord {
            x: temp.x.to_degrees(),
            y: temp.y.to_degrees(),
        }
    }

    fn invert(&self, coordinate: &Coord<T>) -> Coord<T> {
        let temp = self.rotate.invert(&Coord {
            x: coordinate.x.to_radians(),
            y: coordinate.y.to_radians(),
        });
        Coord {
            x: temp.x.to_degrees(),
            y: temp.y.to_degrees(),
        }
    }
}
