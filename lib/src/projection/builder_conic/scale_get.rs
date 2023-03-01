use geo::CoordFloat;
use num_traits::FloatConst;

use super::Builder;
use super::PRConic;

use crate::projection::builder::types::BuilderAntimeridianResampleNoClip;
use crate::projection::ScaleGet;
use crate::Transform;

impl<DRAIN, PR, T> ScaleGet for Builder<BuilderAntimeridianResampleNoClip<DRAIN, PR, T>, PR, T>
where
    DRAIN: Clone,
    PR: Clone + PRConic<T = T> + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn scale(&self) -> Self::T {
        self.base.scale()
    }
}
