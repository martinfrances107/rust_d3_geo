use geo::CoordFloat;

use crate::projection::ReflectGet;

use super::Builder;

impl<PCNU, T> ReflectGet for Builder<PCNU, T>
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
