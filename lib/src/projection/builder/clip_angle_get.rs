use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::ClipAngleGet;

use super::Builder;

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> ClipAngleGet
    for Builder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    PCNU: Debug,
    RU: Debug,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    // Given an angle in degrees. Sets the internal clip angle and returns a builder
    // which uses the clip circle stratergy.
    fn clip_angle_get(&self) -> T {
        self.theta.unwrap()
    }
}
