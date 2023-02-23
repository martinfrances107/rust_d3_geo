use geo::CoordFloat;
use num_traits::FloatConst;

use super::Builder;
use super::PRConic;

use crate::projection::builder::types::BuilderAntimeridianResampleNoClip;
use crate::projection::ScaleSet;
use crate::Transform;

impl<DRAIN, PR, T> ScaleSet for Builder<BuilderAntimeridianResampleNoClip<DRAIN, PR, T>, PR, T>
where
    DRAIN: Clone,
    PR: Clone + PRConic + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn scale_set(&mut self, scale: T) -> &mut Self {
        self.base.scale_set(scale);
        self
    }
}
