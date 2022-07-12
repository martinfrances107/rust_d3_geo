use geo::CoordFloat;

use crate::projection::PrecisionGet;

use super::Builder;

impl<DRAIN, I, LB, LC, LU, PCNU, PR, PV, RC, RU, T> PrecisionGet
    for Builder<DRAIN, I, LB, LC, LU, PCNU, PR, PV, RC, RU, T>
where
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn get_precision(&self) -> T {
        self.delta2.sqrt()
    }
}
