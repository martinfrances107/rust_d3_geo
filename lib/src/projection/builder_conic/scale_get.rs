use geo::CoordFloat;
use num_traits::FloatConst;

use super::Builder;

use crate::projection::ScaleGet;

impl<BASE, T> ScaleGet for Builder<BASE, T>
where
    BASE: ScaleGet<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn scale(&self) -> Self::T {
        self.base.scale()
    }
}
