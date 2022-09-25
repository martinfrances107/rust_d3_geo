use geo::CoordFloat;

use crate::projection::AngleGet;

use super::Builder;

impl<DRAIN, PCNU, T> AngleGet for Builder<DRAIN, PCNU, T>
where
    T: CoordFloat,
{
    /// f64 or f32.
    type T = T;

    /// Returns the projection’s post-projection planar rotation angle.
    /// defaults to 0°.
    #[inline]
    fn angle(&self) -> Self::T {
        self.alpha.to_degrees()
    }
}
