use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::template::ResamplePCNU;
use crate::projection::builder::template::PCNU;
use crate::projection::RotateSet;
use crate::Transform;

use super::Builder;

impl<CLIPU, DRAIN, PR, T> RotateSet for Builder<CLIPU, DRAIN, PCNU<T>, PR, ResamplePCNU<PR, T>, T>
where
    PR: Clone + Transform<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    fn rotate2_set(&mut self, angles: &[T; 2]) -> &mut Self {
        self.base.rotate2_set(angles);
        self
    }

    fn rotate3_set(&mut self, angles: &[T; 3]) -> &mut Self {
        self.base.rotate3_set(angles);
        self
    }
}
