use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::ScaleGet;

use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PCNU, PR, RU, T> ScaleGet
    for Builder<CLIPC, CLIPU, DRAIN, PCNU, PR, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    T: CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn scale(&self) -> Self::T {
        self.k
    }
}
