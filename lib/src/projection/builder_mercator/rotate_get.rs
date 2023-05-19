use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::RotateGet;

use super::Builder;

impl<CLIPU, PCNU, PR, RU, T> RotateGet for Builder<CLIPU, PCNU, PR, RU, T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn rotate(&self) -> [T; 3] {
        self.base.rotate()
    }
}
