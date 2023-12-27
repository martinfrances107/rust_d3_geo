use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder_mercator::Builder;
use crate::projection::ReflectGet;

impl<CLIPU, DRAIN, PCNU, PR, RU, T> ReflectGet
    for Builder<CLIPU, DRAIN, PCNU, PR, RU, T>
where
    CLIPU: Clone,
    T: CoordFloat + FloatConst,
{
    type T = T;

    /// Is the projection builder set to invert the x-coordinate.
    #[inline]
    fn is_x_reflected(&self) -> bool {
        self.base.is_x_reflected()
    }

    /// Is the projection builder set to invert the y-coordinate.
    #[inline]
    fn is_y_reflected(&self) -> bool {
        self.base.is_y_reflected()
    }
}
