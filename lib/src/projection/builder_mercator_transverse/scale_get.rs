use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::ScaleGet;

use super::Builder;

impl<CLIPU, DRAIN, PCNU, PR, RU, T> ScaleGet
    for Builder<CLIPU, DRAIN, PCNU, PR, RU, T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn scale(&self) -> T {
        self.base.scale()
    }
}
