use std::fmt::Debug;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::ScaleGet;

use super::Builder;

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> ScaleGet
    for Builder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    PCNU: Debug,
    RU: Debug,
    T: CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn get_scale(&self) -> Self::T {
        self.k
    }
}
