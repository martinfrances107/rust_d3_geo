use geo::CoordFloat;

use crate::clip::circle::ClipCircleU;
use crate::projection::ClipAngleGet;

use super::Builder;

impl<DRAIN, PCNU, PR, RC, RU, T> ClipAngleGet
    for Builder<ClipCircleU<RC, T>, DRAIN, PCNU, PR, RU, T>
where
    T: CoordFloat,
{
    type T = T;

    // Given an angle in degrees. Sets the internal clip angle and returns a builder
    // which uses the clip circle strategy.
    fn clip_angle(&self) -> T {
        // Unwrap is safe here, the context is ClipCircleU.
        self.theta.unwrap()
    }
}
