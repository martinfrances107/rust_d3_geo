use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::ClipAngleGet;

use super::Builder;

impl<DRAIN, I, LB, LC, LU, PCNU, PR, PV, RC, RU, T> ClipAngleGet
    for Builder<DRAIN, I, LB, LC, LU, PCNU, PR, PV, RC, RU, T>
where
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    // Given an angle in degrees. Sets the internal clip angle and returns a builder
    // which uses the clip circle stratergy.
    fn get_clip_angle(&self) -> T {
        self.theta.unwrap()
    }
}
