use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::RotateGet;

use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PCNU, PR, RU, T> RotateGet
    for Builder<CLIPC, CLIPU, DRAIN, PCNU, PR, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    T: CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn rotate(&self) -> [T; 3] {
        let r = self.base.rotate();
        [r[0], r[1], r[2] - T::from(90).unwrap()]
    }
}
