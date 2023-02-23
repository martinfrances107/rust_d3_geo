use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::types::BuilderAntimeridianResampleNoClip;
use crate::projection::RotateSet;
use crate::Transform;

use super::Builder;

impl<DRAIN, PR, T> RotateSet for Builder<BuilderAntimeridianResampleNoClip<DRAIN, PR, T>, PR, T>
where
    DRAIN: Clone,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
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
