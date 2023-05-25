use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::RotateGet;

use super::Builder;

impl<CLIPU, DRAIN, PCNU, PR, RU, T> RotateGet for Builder<CLIPU, DRAIN, PCNU, PR, RU, T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn rotate(&self) -> [T; 3] {
        let r = self.base.rotate();
        [r[0], r[1], r[2] - T::from(90).unwrap()]
    }
}
