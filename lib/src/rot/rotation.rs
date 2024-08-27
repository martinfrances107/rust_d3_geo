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
    frac_pi_180: T,
    frac_180_pi: T,
}

impl<T> Rotation<T>
where
    T: CoordFloat + FloatConst,
{
    /// Constructor.
    #[inline]
    pub fn new(delta_lambda: T, delta_phi: T, delta_gamma: T) -> Self {
        let frac_pi_180 = T::PI() / T::from(180).unwrap();
        let frac_180_pi = T::from(180).unwrap() / T::PI();

        Self {
            rotate: rotate_radians([
                delta_lambda * frac_pi_180,
                delta_phi * frac_pi_180,
                delta_gamma * frac_pi_180,
            ]),
            frac_pi_180,
            frac_180_pi,
        }
    }
}

impl<T> Transform for Rotation<T>
where
    T: CoordFloat + FloatConst,
{
    /// f64 or f32.
    type T = T;

    fn transform(&self, coordinate: &Coord<T>) -> Coord<T> {
        let temp = self.rotate.transform(&Coord {
            x: coordinate.x * self.frac_pi_180,
            y: coordinate.y * self.frac_pi_180,
        });
        Coord {
            x: temp.x * self.frac_180_pi,
            y: temp.y * self.frac_180_pi,
        }
    }

    fn invert(&self, coordinate: &Coord<T>) -> Coord<T> {
        let temp = self.rotate.invert(&Coord {
            x: coordinate.x * self.frac_pi_180,
            y: coordinate.y * self.frac_pi_180,
        });
        Coord {
            x: temp.x * self.frac_180_pi,
            y: temp.y * self.frac_180_pi,
        }
    }
}
