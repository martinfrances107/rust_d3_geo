use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::template::ResamplePCNU;
use crate::projection::builder::template::PCNU;
use crate::projection::RotateSet;
use crate::Transform;

use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PR, T> RotateSet
    for Builder<CLIPC, CLIPU, DRAIN, PCNU<T>, PR, ResamplePCNU<PR, T>, T>
where
    DRAIN: Clone,
    PR: Clone + Transform<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    fn rotate2_set(&mut self, angles: &[T; 2]) -> &mut Self {
        self.base
            .rotate3_set(&[angles[0], angles[1], T::from(90_f64).unwrap()]);
        self
    }

    fn rotate3_set(&mut self, angles: &[T; 3]) -> &mut Self {
        self.base
            .rotate3_set(&[angles[0], angles[1], angles[2] + T::from(90_f64).unwrap()]);
        self
    }
}
