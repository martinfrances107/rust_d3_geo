use geo::CoordFloat;

use crate::projection::AngleGet;

use super::Builder;

impl<CLIPU, DRAIN, PCNU, PR, RU, T> AngleGet
    for Builder<CLIPU, DRAIN, PCNU, PR, RU, T>
where
    T: CoordFloat,
{
    type T = T;

    /// Returns the projection’s post-projection planar rotation angle.
    /// defaults to 0°.
    #[inline]
    fn angle(&self) -> Self::T {
        self.alpha.to_degrees()
    }
}
