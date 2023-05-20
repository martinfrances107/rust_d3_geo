use geo::CoordFloat;

use crate::projection::AngleGet;

use super::Builder;

impl<PCNU, T> AngleGet for Builder<PCNU, T>
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
