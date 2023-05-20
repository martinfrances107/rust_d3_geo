use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::ScaleGet;

use super::Builder;

impl<PCNU, T> ScaleGet for Builder<PCNU, T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn scale(&self) -> Self::T {
        self.k
    }
}
