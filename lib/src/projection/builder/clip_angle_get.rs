use geo::CoordFloat;

use crate::projection::ClipAngleGet;

use super::Builder;

impl<CLIPU, DRAIN, PCNU, PR, RU, T> ClipAngleGet for Builder<CLIPU, DRAIN, PCNU, PR, RU, T>
where
    T: CoordFloat,
{
    type T = T;

    // Given an angle in degrees. Sets the internal clip angle and returns a builder
    // which uses the clip circle stratergy.
    fn clip_angle(&self) -> T {
        self.theta.unwrap()
    }
}
