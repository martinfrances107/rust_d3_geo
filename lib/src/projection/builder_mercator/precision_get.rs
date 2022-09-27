use geo::CoordFloat;

use crate::projection::PrecisionGet;

use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T> PrecisionGet
    for Builder<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn precision(&self) -> T {
        self.base.precision()
    }
}
