use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::ClipAngleGet;
use crate::stream::Unconnected;

use super::Builder;

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> ClipAngleGet
    for Builder<
        DRAIN,
        I,
        LB,
        LC,
        LU,
        PCNC,
        PCNU,
        PR,
        PV,
        Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
        Resample<DRAIN, PR, PCNC, PCNU, Unconnected, T>,
        T,
    >
where
    DRAIN: Clone + Debug,
    I: Clone,
    LB: Clone,
    LC: Clone + Debug,
    LU: Clone + Debug,
    PCNC: Clone + Debug,
    PCNU: Clone + Debug,
    // PR: Transform<T = T>,
    PR: Clone + Debug,
    PV: Clone + Debug,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    // Given an angle in degrees. Sets the internal clip angle and returns a builder
    // which uses the clip circle stratergy.
    fn clip_angle_get(&self) -> T {
        self.theta.unwrap()
    }
}
