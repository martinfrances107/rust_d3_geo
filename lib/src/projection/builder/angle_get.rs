use geo::CoordFloat;

use crate::projection::AngleGet;

use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T> AngleGet
    for Builder<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
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
