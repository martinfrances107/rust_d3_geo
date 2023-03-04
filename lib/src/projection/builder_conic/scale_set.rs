use geo::CoordFloat;
use num_traits::FloatConst;

use super::Builder;
use super::PRConic;
use crate::projection::builder::Builder as BuilderCommon;

use crate::projection::ScaleSet;
use crate::Transform;

impl<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T> ScaleSet
    for Builder<BuilderCommon<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T>, T>
where
    BuilderCommon<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T>: Clone + ScaleSet<T = T>,
    PR: Clone + PRConic<T = T> + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn scale_set(&mut self, scale: T) -> &mut Self {
        self.base.scale_set(scale);
        self
    }
}
