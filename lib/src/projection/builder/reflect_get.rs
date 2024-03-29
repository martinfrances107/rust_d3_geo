use geo::CoordFloat;

use crate::projection::ReflectGet;

use super::Builder;

impl<CLIPU, DRAIN, PCNU, PR, RU, T> ReflectGet
    for Builder<CLIPU, DRAIN, PCNU, PR, RU, T>
where
    T: CoordFloat,
{
    type T = T;

    /// Is the projection builder set to invert the x-coordinate.
    #[inline]
    fn is_x_reflected(&self) -> bool {
        self.sx < T::zero()
    }

    /// Is the projection builder set to invert the y-coordinate.
    #[inline]
    fn is_y_reflected(&self) -> bool {
        self.sy < T::zero()
    }
}
